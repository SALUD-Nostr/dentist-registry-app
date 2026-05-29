//! AllergyIntolerance storage using `IndexedDB`

use gloo_console::log;
use idb::{Database, TransactionMode};
use salud_types::AllergyIntolerance;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::stores;
use crate::error::AppError;

#[derive(Clone)]
pub struct AllergyStore {
    db: Rc<Database>,
}

impl AllergyStore {
    /// Create from existing database
    pub const fn from_db(db: Rc<Database>) -> Self {
        Self { db }
    }

    /// Save an allergy to the database
    pub async fn save(
        &self,
        allergy: &AllergyIntolerance,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<(), AppError> {
        let note = crate::salud_note::SaludNote::from_fhir(allergy, "AllergyIntolerance", keypair)?;

        let tx = self
            .db
            .transaction(&[stores::ALLERGIES], TransactionMode::ReadWrite)?;
        let store = tx.object_store(stores::ALLERGIES)?;
        let note_value = to_value(&note)?;
        let id = allergy.id.as_ref().ok_or(AppError::MissingResourceId)?;
        store.put(&note_value, Some(&JsValue::from_str(id)))?;
        tx.await?;

        log!("Allergy saved successfully");
        Ok(())
    }

    /// Get all allergies
    pub async fn get_all(&self) -> Result<Vec<AllergyIntolerance>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::ALLERGIES], TransactionMode::ReadOnly)?;
        let store = tx.object_store(stores::ALLERGIES)?;
        let values = store.get_all(None, None)?.await?;

        let mut allergies = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone())
                && let Ok(allergy) = crate::salud_note::SaludNote::parse_fhir(&note)
            {
                allergies.push(allergy);
            }
        }
        Ok(allergies)
    }

    /// Get allergies for a specific patient
    pub async fn get_by_patient(
        &self,
        patient_id: &str,
    ) -> Result<Vec<AllergyIntolerance>, AppError> {
        let all = self.get_all().await?;
        let patient_ref = format!("Patient/{patient_id}");
        Ok(all
            .into_iter()
            .filter(|a| a.patient.reference.as_ref() == Some(&patient_ref))
            .collect())
    }

    /// Delete an allergy by ID
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let tx = self
            .db
            .transaction(&[stores::ALLERGIES], TransactionMode::ReadWrite)?;
        let store = tx.object_store(stores::ALLERGIES)?;
        store.delete(JsValue::from_str(id))?;
        tx.await?;
        log!("Allergy deleted successfully");
        Ok(())
    }
}

// Yew context provider
#[derive(Clone)]
pub struct AllergyStoreContext {
    pub store: Rc<AllergyStore>,
}

impl PartialEq for AllergyStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
    }
}

#[derive(Properties, PartialEq)]
pub struct AllergyStoreProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AllergyStoreProvider)]
pub fn allergy_store_provider(props: &AllergyStoreProviderProps) -> Html {
    let patient_store = crate::storage::use_patient_store();
    let store = Rc::new(AllergyStore::from_db(patient_store.db.clone()));
    let context = AllergyStoreContext { store };

    html! {
        <ContextProvider<AllergyStoreContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<AllergyStoreContext>>
    }
}

/// Hook to use allergy store from context
#[hook]
pub fn use_allergy_store() -> Rc<AllergyStore> {
    use_context::<AllergyStoreContext>()
        .expect("AllergyStoreContext not found")
        .store
}
