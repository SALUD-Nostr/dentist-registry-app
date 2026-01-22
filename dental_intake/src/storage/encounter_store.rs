//! Encounter storage using IndexedDB

use gloo_console::log;
use idb::{Database, TransactionMode};
use salud_types::Encounter;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::{stores, PatientStore};

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
    pub async fn save(&self, encounter: &Encounter) -> Result<(), String> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadWrite).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::ENCOUNTERS).map_err(|e| format!("{:?}", e))?;

        let encounter_value = to_value(encounter).map_err(|e| format!("{:?}", e))?;

        if let Some(id) = &encounter.id {
            store.put(&encounter_value, Some(&JsValue::from_str(id))).map_err(|e| format!("{:?}", e))?;
        } else {
            store.add(&encounter_value, None).map_err(|e| format!("{:?}", e))?;
        }

        tx.await.map_err(|e| JsValue::from_str(&format!("{:?}", e))).map_err(|e| format!("{:?}", e))?;

        log!("Encounter saved successfully");
        Ok(())
    }

    /// Get an encounter by ID
    pub async fn get(&self, id: &str) -> Result<Option<Encounter>, String> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadOnly).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::ENCOUNTERS).map_err(|e| format!("{:?}", e))?;

        let value = store.get(JsValue::from_str(id))?.await.map_err(|e| format!("{:?}", e))?;

        if value.is_undefined() || value.is_null() {
            return Ok(None);
        }

        let encounter: Encounter = from_value(value).map_err(|e| format!("{:?}", e))?;
        Ok(Some(encounter))
    }

    /// Get all encounters
    pub async fn get_all(&self) -> Result<Vec<Encounter>, String> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadOnly).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::ENCOUNTERS).map_err(|e| format!("{:?}", e))?;

        let values = store.get_all(None, None)?.await.map_err(|e| format!("{:?}", e))?;

        let mut encounters = Vec::new();
        for value in values.iter() {
            let encounter: Encounter = from_value(value.clone()).map_err(|e| format!("{:?}", e))?;
            encounters.push(encounter);
        }

        Ok(encounters)
    }

    /// Get encounters for a specific patient
    pub async fn get_by_patient(&self, patient_id: &str) -> Result<Vec<Encounter>, String> {
        let all_encounters = self.get_all().await.map_err(|e| format!("{:?}", e))?;

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
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let tx = self
            .db
            .transaction(&[stores::ENCOUNTERS], TransactionMode::ReadWrite).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::ENCOUNTERS).map_err(|e| format!("{:?}", e))?;

        store.delete(JsValue::from_str(id)).map_err(|e| format!("{:?}", e))?;

        tx.await.map_err(|e| JsValue::from_str(&format!("{:?}", e))).map_err(|e| format!("{:?}", e))?;

        log!("Encounter deleted successfully");
        Ok(())
    }
}

// Yew context provider
#[derive(Clone)]
pub struct EncounterStoreContext {
    pub store: Rc<EncounterStore>,
}

impl PartialEq for EncounterStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
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

    let context = EncounterStoreContext {
        store: store.clone(),
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
