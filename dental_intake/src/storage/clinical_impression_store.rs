//! Clinical Impression storage using IndexedDB

use gloo_console::log;
use idb::{Database, TransactionMode};
use salud_types::ClinicalImpression;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::stores;

#[derive(Clone)]
pub struct ClinicalImpressionStore {
    db: Rc<Database>,
}

impl ClinicalImpressionStore {
    /// Create from existing database
    pub fn from_db(db: Rc<Database>) -> Self {
        Self { db }
    }

    /// Save a clinical impression to the database
    pub async fn save(&self, impression: &ClinicalImpression) -> Result<(), String> {
        let tx = self.db.transaction(
            &[stores::CLINICAL_IMPRESSIONS],
            TransactionMode::ReadWrite,
        ).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS).map_err(|e| format!("{:?}", e))?;

        let impression_value = to_value(impression).map_err(|e| format!("{:?}", e))?;

        if let Some(id) = &impression.id {
            store.put(&impression_value, Some(&JsValue::from_str(id))).map_err(|e| format!("{:?}", e))?;
        } else {
            store.add(&impression_value, None).map_err(|e| format!("{:?}", e))?;
        }

        tx.await.map_err(|e| JsValue::from_str(&format!("{:?}", e))).map_err(|e| format!("{:?}", e))?;

        log!("Clinical impression saved successfully");
        Ok(())
    }

    /// Get a clinical impression by ID
    pub async fn get(&self, id: &str) -> Result<Option<ClinicalImpression>, String> {
        let tx = self.db.transaction(
            &[stores::CLINICAL_IMPRESSIONS],
            TransactionMode::ReadOnly,
        ).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS).map_err(|e| format!("{:?}", e))?;

        let value = store.get(JsValue::from_str(id))?.await.map_err(|e| format!("{:?}", e))?;

        if value.is_undefined() || value.is_null() {
            return Ok(None);
        }

        let impression: ClinicalImpression = from_value(value).map_err(|e| format!("{:?}", e))?;
        Ok(Some(impression))
    }

    /// Get all clinical impressions
    pub async fn get_all(&self) -> Result<Vec<ClinicalImpression>, String> {
        let tx = self.db.transaction(
            &[stores::CLINICAL_IMPRESSIONS],
            TransactionMode::ReadOnly,
        ).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS).map_err(|e| format!("{:?}", e))?;

        let values = store.get_all(None, None)?.await.map_err(|e| format!("{:?}", e))?;

        let mut impressions = Vec::new();
        for value in values.iter() {
            let impression: ClinicalImpression = from_value(value.clone()).map_err(|e| format!("{:?}", e))?;
            impressions.push(impression);
        }

        Ok(impressions)
    }

    /// Get clinical impressions for a specific encounter
    pub async fn get_by_encounter(&self, encounter_id: &str) -> Result<Vec<ClinicalImpression>, String> {
        let all_impressions = self.get_all().await.map_err(|e| format!("{:?}", e))?;

        let encounter_ref = format!("Encounter/{}", encounter_id);

        let filtered: Vec<ClinicalImpression> = all_impressions
            .into_iter()
            .filter(|impression| {
                impression
                    .encounter
                    .as_ref()
                    .and_then(|e| e.reference.as_ref())
                    .map_or(false, |r| r == &encounter_ref)
            })
            .collect();

        Ok(filtered)
    }

    /// Delete a clinical impression by ID
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let tx = self.db.transaction(
            &[stores::CLINICAL_IMPRESSIONS],
            TransactionMode::ReadWrite,
        ).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS).map_err(|e| format!("{:?}", e))?;

        store.delete(JsValue::from_str(id)).map_err(|e| format!("{:?}", e))?;

        tx.await.map_err(|e| JsValue::from_str(&format!("{:?}", e))).map_err(|e| format!("{:?}", e))?;

        log!("Clinical impression deleted successfully");
        Ok(())
    }
}

// Yew context provider
#[derive(Clone)]
pub struct ClinicalImpressionStoreContext {
    pub store: Rc<ClinicalImpressionStore>,
}

impl PartialEq for ClinicalImpressionStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
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

    let context = ClinicalImpressionStoreContext {
        store: store.clone(),
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
