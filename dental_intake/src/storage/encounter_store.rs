//! Encounter storage using IndexedDB

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
    pub fn from_db(db: Rc<Database>) -> Self {
        Self { db }
    }

    /// Save an encounter to the database
    pub async fn save(&self, encounter: &Encounter) -> Result<(), AppError> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadWrite)?;

        let store = tx.object_store(stores::ENCOUNTERS)?;

        let encounter_value = to_value(encounter)?;

        if let Some(id) = &encounter.id {
            store.put(&encounter_value, Some(&JsValue::from_str(id)))?;
        } else {
            store.add(&encounter_value, None)?;
        }

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
                let encounter: Encounter = from_value(js_val)?;
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
        for value in values.iter() {
            let encounter: Encounter = from_value(value.clone())?;
            encounters.push(encounter);
        }

        Ok(encounters)
    }

    /// Get encounters for a specific patient
    pub async fn get_by_patient(&self, patient_id: &str) -> Result<Vec<Encounter>, AppError> {
        let all_encounters = self.get_all().await?;

        let patient_ref = format!("Patient/{}", patient_id);

        let filtered: Vec<Encounter> = all_encounters
            .into_iter()
            .filter(|encounter| {
                encounter
                    .subject
                    .reference
                    .as_ref()
                    .map_or(false, |r| r == &patient_ref)
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
}

impl PartialEq for EncounterStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store) && *self.version == *other.version
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

    let context = EncounterStoreContext {
        store: store.clone(),
        version: version.clone(),
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
    let context = use_context::<EncounterStoreContext>()
        .expect("EncounterStoreContext not found");

    let version = context.version.clone();

    Callback::from(move |_| {
        version.set(*version + 1);
    })
}
