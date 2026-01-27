//! Encounter storage using `IndexedDB`

use gloo_console::log;
use idb::{Database, TransactionMode};
use salud_types::Encounter;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::stores;
use crate::error::AppError;

#[derive(Clone)]
pub struct EncounterStore {
    db: Rc<Database>,
}

impl EncounterStore {
    /// Create from existing database
    #[must_use] pub const fn from_db(db: Rc<Database>) -> Self {
        Self { db }
    }

    /// Save an encounter to the database
    pub async fn save(
        &self,
        encounter: &Encounter,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<(), AppError> {
        // Wrap in SaludNote
        let note = crate::salud_note::SaludNote::from_fhir(encounter, "Encounter", keypair)?;

        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadWrite)?;

        let store = tx.object_store(stores::ENCOUNTERS)?;

        let note_value = to_value(&note)?;

        let id = encounter.id.as_ref().ok_or(AppError::MissingResourceId)?;
        store.put(&note_value, Some(&JsValue::from_str(id)))?;

        tx.await?;

        log!("Encounter saved successfully");
        Ok(())
    }

    /// Get an encounter by ID
    pub async fn get(&self, id: &str) -> Result<Option<Encounter>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::ENCOUNTERS)?;

        let value = store.get(JsValue::from_str(id))?.await?;

        match value {
            Some(js_val) if !js_val.is_undefined() && !js_val.is_null() => {
                let note: nostr_minions::nostro2::NostrNote = from_value(js_val)?;
                let encounter = crate::salud_note::SaludNote::parse_fhir(&note)?;
                Ok(Some(encounter))
            }
            _ => Ok(None),
        }
    }

    /// Get all encounters
    pub async fn get_all(&self) -> Result<Vec<Encounter>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::ENCOUNTERS)?;

        let values = store.get_all(None, None)?.await?;

        let mut encounters = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone())
                && let Ok(encounter) = crate::salud_note::SaludNote::parse_fhir(&note) {
                    encounters.push(encounter);
                }
        }

        Ok(encounters)
    }

    /// Get all encounter notes (raw `NostrNotes`)
    pub async fn get_all_notes(&self) -> Result<Vec<nostr_minions::nostro2::NostrNote>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::ENCOUNTERS)?;

        let values = store.get_all(None, None)?.await?;

        let mut notes = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone()) {
                notes.push(note);
            }
        }

        Ok(notes)
    }

    /// Get encounters for a specific patient
    pub async fn get_by_patient(&self, patient_id: &str) -> Result<Vec<Encounter>, AppError> {
        let all_encounters = self.get_all().await?;

        let patient_ref = format!("Patient/{patient_id}");

        let filtered: Vec<Encounter> = all_encounters
            .into_iter()
            .filter(|encounter| {
                encounter
                    .subject
                    .reference
                    .as_ref() == Some(&patient_ref)
            })
            .collect();

        Ok(filtered)
    }

    /// Delete an encounter by ID
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadWrite)?;

        let store = tx.object_store(stores::ENCOUNTERS)?;

        store.delete(JsValue::from_str(id))?;

        tx.await?;

        log!("Encounter deleted successfully");
        Ok(())
    }
}

// Yew context provider
#[derive(Clone)]
pub struct EncounterStoreContext {
    pub store: Rc<EncounterStore>,
    pub version: UseStateHandle<u32>,
    // Cache of NostrNotes in memory
    pub notes_cache: UseStateHandle<Vec<nostr_minions::nostro2::NostrNote>>,
}

impl PartialEq for EncounterStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
            && *self.version == *other.version
            && *self.notes_cache == *other.notes_cache
    }
}

#[derive(Properties, PartialEq)]
pub struct EncounterStoreProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(EncounterStoreProvider)]
pub fn encounter_store_provider(props: &EncounterStoreProviderProps) -> Html {
    // Get patient store to reuse the database
    let patient_store = crate::storage::use_patient_store();

    let store = Rc::new(EncounterStore::from_db(patient_store.db.clone()));
    let version = use_state(|| 0u32);
    let notes_cache = use_state(Vec::new);

    // Load notes into cache when version changes
    {
        let store = store.clone();
        let notes_cache = notes_cache.clone();
        let version_val = *version;

        use_effect_with(version_val, move |_| {
            let notes_cache = notes_cache.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match store.get_all_notes().await {
                    Ok(all_notes) => {
                        notes_cache.set(all_notes);
                    }
                    Err(e) => {
                        gloo_console::error!("Failed to load encounter notes:", format!("{:?}", e));
                    }
                }
            });
            || ()
        });
    }

    let context = EncounterStoreContext {
        store,
        version,
        notes_cache,
    };

    html! {
        <ContextProvider<EncounterStoreContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<EncounterStoreContext>>
    }
}

/// Hook to use encounter store from context
#[hook]
pub fn use_encounter_store() -> Rc<EncounterStore> {
    use_context::<EncounterStoreContext>()
        .expect("EncounterStoreContext not found")
        .store
}

/// Hook to get the encounter store version for reactive updates
#[hook]
pub fn use_encounter_store_version() -> u32 {
    *use_context::<EncounterStoreContext>()
        .expect("EncounterStoreContext not found")
        .version
}

/// Hook to notify that encounters have been updated
#[hook]
pub fn use_notify_encounters_changed() -> Callback<()> {
    let context = use_context::<EncounterStoreContext>().expect("EncounterStoreContext not found");

    let version = context.version;

    Callback::from(move |()| {
        version.set(*version + 1);
    })
}

/// Hook to access raw encounter `NostrNotes`
#[hook]
pub fn use_encounter_notes() -> Vec<nostr_minions::nostro2::NostrNote> {
    let context = use_context::<EncounterStoreContext>().expect("EncounterStoreContext not found");
    (*context.notes_cache).clone()
}

/// Hook to get a specific encounter note by ID
#[hook]
pub fn use_encounter_note(id: &str) -> Option<nostr_minions::nostro2::NostrNote> {
    let notes = use_encounter_notes();
    let id = id.to_string();
    notes.into_iter().find(|note| {
        if let Ok(encounter) = crate::salud_note::SaludNote::parse_fhir::<Encounter>(note) {
            encounter
                .id
                .as_ref() == Some(&id)
        } else {
            false
        }
    })
}
