//! Clinical Impression storage using `IndexedDB`

use gloo_console::log;
use idb::{Database, TransactionMode};
use salud_types::ClinicalImpression;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::stores;
use crate::error::AppError;

#[derive(Clone)]
pub struct ClinicalImpressionStore {
    db: Rc<Database>,
}

impl ClinicalImpressionStore {
    /// Create from existing database
    pub const fn from_db(db: Rc<Database>) -> Self {
        Self { db }
    }

    /// Save a clinical impression to the database
    pub async fn save(
        &self,
        impression: &ClinicalImpression,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<(), AppError> {
        // Wrap in SaludNote
        let note =
            crate::salud_note::SaludNote::from_fhir(impression, "ClinicalImpression", keypair)?;

        let tx = self
            .db
            .transaction(&[stores::CLINICAL_IMPRESSIONS], TransactionMode::ReadWrite)?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS)?;

        let note_value = to_value(&note)?;

        let id = impression.id.as_ref().ok_or(AppError::MissingResourceId)?;
        store.put(&note_value, Some(&JsValue::from_str(id)))?;

        tx.await?;

        log!("Clinical impression saved successfully");
        Ok(())
    }

    /// Get a clinical impression by ID
    pub async fn get(&self, id: &str) -> Result<Option<ClinicalImpression>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::CLINICAL_IMPRESSIONS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS)?;

        let value = store.get(JsValue::from_str(id))?.await?;

        match value {
            Some(js_val) if !js_val.is_undefined() && !js_val.is_null() => {
                let note: nostr_minions::nostro2::NostrNote = from_value(js_val)?;
                let impression = crate::salud_note::SaludNote::parse_fhir(&note)?;
                Ok(Some(impression))
            }
            _ => Ok(None),
        }
    }

    /// Get all clinical impressions
    pub async fn get_all(&self) -> Result<Vec<ClinicalImpression>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::CLINICAL_IMPRESSIONS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS)?;

        let values = store.get_all(None, None)?.await?;

        let mut impressions = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone())
                && let Ok(impression) = crate::salud_note::SaludNote::parse_fhir(&note)
            {
                impressions.push(impression);
            }
        }

        Ok(impressions)
    }

    /// Get all clinical impression notes (raw `NostrNotes`)
    pub async fn get_all_notes(&self) -> Result<Vec<nostr_minions::nostro2::NostrNote>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::CLINICAL_IMPRESSIONS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS)?;

        let values = store.get_all(None, None)?.await?;

        let mut notes = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone()) {
                notes.push(note);
            }
        }

        Ok(notes)
    }

    /// Get clinical impressions for a specific encounter
    pub async fn get_by_encounter(
        &self,
        encounter_id: &str,
    ) -> Result<Vec<ClinicalImpression>, AppError> {
        let all_impressions = self.get_all().await?;

        let encounter_ref = format!("Encounter/{encounter_id}");

        let filtered: Vec<ClinicalImpression> = all_impressions
            .into_iter()
            .filter(|impression| {
                impression
                    .encounter
                    .as_ref()
                    .and_then(|e| e.reference.as_ref())
                    == Some(&encounter_ref)
            })
            .collect();

        Ok(filtered)
    }

    /// Delete a clinical impression by ID
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let tx = self
            .db
            .transaction(&[stores::CLINICAL_IMPRESSIONS], TransactionMode::ReadWrite)?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS)?;

        store.delete(JsValue::from_str(id))?;

        tx.await?;

        log!("Clinical impression deleted successfully");
        Ok(())
    }
}

// Yew context provider
#[derive(Clone)]
pub struct ClinicalImpressionStoreContext {
    pub store: Rc<ClinicalImpressionStore>,
    pub version: UseStateHandle<u32>,
    // Cache of NostrNotes in memory
    pub notes_cache: UseStateHandle<Vec<nostr_minions::nostro2::NostrNote>>,
}

impl PartialEq for ClinicalImpressionStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
            && *self.version == *other.version
            && *self.notes_cache == *other.notes_cache
    }
}

#[derive(Properties, PartialEq)]
pub struct ClinicalImpressionStoreProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ClinicalImpressionStoreProvider)]
pub fn clinical_impression_store_provider(props: &ClinicalImpressionStoreProviderProps) -> Html {
    // Get patient store to reuse the database
    let patient_store = crate::storage::use_patient_store();

    let store = Rc::new(ClinicalImpressionStore::from_db(patient_store.db.clone()));
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
                        gloo_console::error!(
                            "Failed to load clinical impression notes:",
                            format!("{:?}", e)
                        );
                    }
                }
            });
            || ()
        });
    }

    let context = ClinicalImpressionStoreContext {
        store,
        version,
        notes_cache,
    };

    html! {
        <ContextProvider<ClinicalImpressionStoreContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ClinicalImpressionStoreContext>>
    }
}

/// Hook to use clinical impression store from context
#[hook]
pub fn use_clinical_impression_store() -> Rc<ClinicalImpressionStore> {
    use_context::<ClinicalImpressionStoreContext>()
        .expect("ClinicalImpressionStoreContext not found")
        .store
}

/// Hook to get the clinical impression store version for reactive updates
#[hook]
pub fn use_clinical_impression_store_version() -> u32 {
    *use_context::<ClinicalImpressionStoreContext>()
        .expect("ClinicalImpressionStoreContext not found")
        .version
}

/// Hook to notify that clinical impressions have been updated
#[hook]
pub fn use_notify_clinical_impressions_changed() -> Callback<()> {
    let context = use_context::<ClinicalImpressionStoreContext>()
        .expect("ClinicalImpressionStoreContext not found");

    let version = context.version;

    Callback::from(move |()| {
        version.set(*version + 1);
    })
}

/// Hook to access raw clinical impression `NostrNotes`
#[hook]
pub fn use_clinical_impression_notes() -> Vec<nostr_minions::nostro2::NostrNote> {
    let context = use_context::<ClinicalImpressionStoreContext>()
        .expect("ClinicalImpressionStoreContext not found");
    (*context.notes_cache).clone()
}

/// Hook to get a specific clinical impression note by ID
#[hook]
pub fn use_clinical_impression_note(id: &str) -> Option<nostr_minions::nostro2::NostrNote> {
    let notes = use_clinical_impression_notes();
    let id = id.to_string();
    notes.into_iter().find(|note| {
        if let Ok(impression) = crate::salud_note::SaludNote::parse_fhir::<ClinicalImpression>(note)
        {
            impression.id.as_ref() == Some(&id)
        } else {
            false
        }
    })
}
