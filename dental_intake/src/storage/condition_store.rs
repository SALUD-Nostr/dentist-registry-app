//! Condition storage using `IndexedDB`

use gloo_console::log;
use idb::{Database, TransactionMode};
use salud_types::Condition;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::stores;
use crate::error::AppError;

#[derive(Clone)]
pub struct ConditionStore {
    db: Rc<Database>,
}

impl ConditionStore {
    /// Create from existing database
    pub const fn from_db(db: Rc<Database>) -> Self {
        Self { db }
    }

    /// Save a condition to the database
    pub async fn save(
        &self,
        condition: &Condition,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<(), AppError> {
        let note = crate::salud_note::SaludNote::from_fhir(condition, "Condition", keypair)?;

        let tx = self
            .db
            .transaction(&[stores::CONDITIONS], TransactionMode::ReadWrite)?;
        let store = tx.object_store(stores::CONDITIONS)?;
        let note_value = to_value(&note)?;
        let id = condition.id.as_ref().ok_or(AppError::MissingResourceId)?;
        store.put(&note_value, Some(&JsValue::from_str(id)))?;
        tx.await?;

        log!("Condition saved successfully");
        Ok(())
    }

    /// Get all conditions
    pub async fn get_all(&self) -> Result<Vec<Condition>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::CONDITIONS], TransactionMode::ReadOnly)?;
        let store = tx.object_store(stores::CONDITIONS)?;
        let values = store.get_all(None, None)?.await?;

        let mut conditions = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone())
                && let Ok(condition) = crate::salud_note::SaludNote::parse_fhir(&note)
            {
                conditions.push(condition);
            }
        }
        Ok(conditions)
    }

    /// Get conditions for a specific patient
    pub async fn get_by_patient(&self, patient_id: &str) -> Result<Vec<Condition>, AppError> {
        let all = self.get_all().await?;
        let patient_ref = format!("Patient/{patient_id}");
        Ok(all
            .into_iter()
            .filter(|c| c.subject.reference.as_ref() == Some(&patient_ref))
            .collect())
    }

    /// Delete a condition by ID
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let tx = self
            .db
            .transaction(&[stores::CONDITIONS], TransactionMode::ReadWrite)?;
        let store = tx.object_store(stores::CONDITIONS)?;
        store.delete(JsValue::from_str(id))?;
        tx.await?;
        log!("Condition deleted successfully");
        Ok(())
    }
}

// Yew context provider
#[derive(Clone)]
pub struct ConditionStoreContext {
    pub store: Rc<ConditionStore>,
}

impl PartialEq for ConditionStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
    }
}

#[derive(Properties, PartialEq)]
pub struct ConditionStoreProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ConditionStoreProvider)]
pub fn condition_store_provider(props: &ConditionStoreProviderProps) -> Html {
    let patient_store = crate::storage::use_patient_store();
    let store = Rc::new(ConditionStore::from_db(patient_store.db.clone()));
    let context = ConditionStoreContext { store };

    html! {
        <ContextProvider<ConditionStoreContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ConditionStoreContext>>
    }
}

/// Hook to use condition store from context
#[hook]
pub fn use_condition_store() -> Rc<ConditionStore> {
    use_context::<ConditionStoreContext>()
        .expect("ConditionStoreContext not found")
        .store
}
