//! Clinical Impression storage using IndexedDB

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
    pub fn from_db(db: Rc<Database>) -> Self {
        Self { db }
    }

    /// Save a clinical impression to the database
    pub async fn save(&self, impression: &ClinicalImpression) -> Result<(), AppError> {
        let tx = self
            .db
            .transaction(&[stores::CLINICAL_IMPRESSIONS], TransactionMode::ReadWrite)?;

        let store = tx.object_store(stores::CLINICAL_IMPRESSIONS)?;

        let impression_value = to_value(impression)?;

        if let Some(id) = &impression.id {
            store.put(&impression_value, Some(&JsValue::from_str(id)))?;
        } else {
            store.add(&impression_value, None)?;
        }

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
                let impression: ClinicalImpression = from_value(js_val)?;
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
        for value in values.iter() {
            let impression: ClinicalImpression = from_value(value.clone())?;
            impressions.push(impression);
        }

        Ok(impressions)
    }

    /// Get clinical impressions for a specific encounter
    pub async fn get_by_encounter(
        &self,
        encounter_id: &str,
    ) -> Result<Vec<ClinicalImpression>, AppError> {
        let all_impressions = self.get_all().await?;

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
