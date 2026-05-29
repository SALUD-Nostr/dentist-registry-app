//! Procedure storage using `IndexedDB`

use gloo_console::log;
use idb::{Database, TransactionMode};
use salud_types::Procedure;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::stores;
use crate::error::AppError;

#[derive(Clone)]
pub struct ProcedureStore {
    db: Rc<Database>,
}

impl ProcedureStore {
    /// Create from existing database
    pub const fn from_db(db: Rc<Database>) -> Self {
        Self { db }
    }

    /// Save a procedure to the database
    pub async fn save(
        &self,
        procedure: &Procedure,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<(), AppError> {
        let note = crate::salud_note::SaludNote::from_fhir(procedure, "Procedure", keypair)?;

        let tx = self
            .db
            .transaction(&[stores::PROCEDURES], TransactionMode::ReadWrite)?;
        let store = tx.object_store(stores::PROCEDURES)?;
        let note_value = to_value(&note)?;
        let id = procedure.id.as_ref().ok_or(AppError::MissingResourceId)?;
        store.put(&note_value, Some(&JsValue::from_str(id)))?;
        tx.await?;

        log!("Procedure saved successfully");
        Ok(())
    }

    /// Get all procedures
    pub async fn get_all(&self) -> Result<Vec<Procedure>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::PROCEDURES], TransactionMode::ReadOnly)?;
        let store = tx.object_store(stores::PROCEDURES)?;
        let values = store.get_all(None, None)?.await?;

        let mut procedures = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone())
                && let Ok(procedure) = crate::salud_note::SaludNote::parse_fhir(&note)
            {
                procedures.push(procedure);
            }
        }
        Ok(procedures)
    }

    /// Get procedures for a specific patient
    pub async fn get_by_patient(&self, patient_id: &str) -> Result<Vec<Procedure>, AppError> {
        let all = self.get_all().await?;
        let patient_ref = format!("Patient/{patient_id}");
        Ok(all
            .into_iter()
            .filter(|p| p.subject.reference.as_ref() == Some(&patient_ref))
            .collect())
    }

    /// Delete a procedure by ID
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let tx = self
            .db
            .transaction(&[stores::PROCEDURES], TransactionMode::ReadWrite)?;
        let store = tx.object_store(stores::PROCEDURES)?;
        store.delete(JsValue::from_str(id))?;
        tx.await?;
        log!("Procedure deleted successfully");
        Ok(())
    }
}

// Yew context provider
#[derive(Clone)]
pub struct ProcedureStoreContext {
    pub store: Rc<ProcedureStore>,
}

impl PartialEq for ProcedureStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
    }
}

#[derive(Properties, PartialEq)]
pub struct ProcedureStoreProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ProcedureStoreProvider)]
pub fn procedure_store_provider(props: &ProcedureStoreProviderProps) -> Html {
    let patient_store = crate::storage::use_patient_store();
    let store = Rc::new(ProcedureStore::from_db(patient_store.db.clone()));
    let context = ProcedureStoreContext { store };

    html! {
        <ContextProvider<ProcedureStoreContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ProcedureStoreContext>>
    }
}

/// Hook to use procedure store from context
#[hook]
pub fn use_procedure_store() -> Rc<ProcedureStore> {
    use_context::<ProcedureStoreContext>()
        .expect("ProcedureStoreContext not found")
        .store
}
