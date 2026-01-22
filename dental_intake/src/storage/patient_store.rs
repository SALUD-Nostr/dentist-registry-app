//! Patient storage using IndexedDB

use gloo_console::log;
use idb::{Database, Factory, ObjectStore, TransactionMode};
use salud_types::Patient;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::{stores, DB_NAME, DB_VERSION};

#[derive(Clone)]
pub struct PatientStore {
    pub(super) db: Rc<Database>,
}

impl PatientStore {
    /// Open or create the database
    pub async fn new() -> Result<Self, String> {
        let factory = Factory::new().map_err(|e| format!("{:?}", e))?;

        let db = factory
            .open(DB_NAME, Some(DB_VERSION), |evt| {
                let db = evt.database();

                // Create patients object store if it doesn't exist
                if !db.object_store_names().contains(stores::PATIENTS) {
                    let store = db.create_object_store(stores::PATIENTS).map_err(|e| format!("{:?}", e))?;
                    // Create index on patient id
                    store.create_index("by_id", "id", true).map_err(|e| format!("{:?}", e))?;
                }

                // Create encounters object store
                if !db.object_store_names().contains(stores::ENCOUNTERS) {
                    let store = db.create_object_store(stores::ENCOUNTERS).map_err(|e| format!("{:?}", e))?;
                    store.create_index("by_id", "id", true).map_err(|e| format!("{:?}", e))?;
                    store.create_index("by_patient", "subject.reference", false).map_err(|e| format!("{:?}", e))?;
                }

                // Create clinical impressions object store
                if !db.object_store_names().contains(stores::CLINICAL_IMPRESSIONS) {
                    let store = db.create_object_store(stores::CLINICAL_IMPRESSIONS).map_err(|e| format!("{:?}", e))?;
                    store.create_index("by_id", "id", true).map_err(|e| format!("{:?}", e))?;
                    store.create_index("by_encounter", "encounter.reference", false).map_err(|e| format!("{:?}", e))?;
                }

                Ok(())
            })
            .await.map_err(|e| format!("{:?}", e))?;

        Ok(Self { db: Rc::new(db) })
    }

    /// Save a patient to the database
    pub async fn save(&self, patient: &Patient) -> Result<(), String> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadWrite).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::PATIENTS).map_err(|e| format!("{:?}", e))?;

        let patient_value = to_value(patient).map_err(|e| format!("{:?}", e))?;

        // Use patient id as key if available, otherwise use auto-increment
        if let Some(id) = &patient.id {
            store.put(&patient_value, Some(&JsValue::from_str(id))).map_err(|e| format!("{:?}", e))?;
        } else {
            store.add(&patient_value, None).map_err(|e| format!("{:?}", e))?;
        }

        tx.await.map_err(|e| JsValue::from_str(&format!("{:?}", e))).map_err(|e| format!("{:?}", e))?;

        log!("Patient saved successfully");
        Ok(())
    }

    /// Get a patient by ID
    pub async fn get(&self, id: &str) -> Result<Option<Patient>, String> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadOnly).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::PATIENTS).map_err(|e| format!("{:?}", e))?;

        let value = store.get(JsValue::from_str(id))?.await.map_err(|e| format!("{:?}", e))?;

        if value.is_undefined() || value.is_null() {
            return Ok(None);
        }

        let patient: Patient = from_value(value).map_err(|e| format!("{:?}", e))?;
        Ok(Some(patient))
    }

    /// Get all patients
    pub async fn get_all(&self) -> Result<Vec<Patient>, String> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadOnly).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::PATIENTS).map_err(|e| format!("{:?}", e))?;

        let values = store.get_all(None, None)?.await.map_err(|e| format!("{:?}", e))?;

        let mut patients = Vec::new();
        for value in values.iter() {
            let patient: Patient = from_value(value.clone()).map_err(|e| format!("{:?}", e))?;
            patients.push(patient);
        }

        Ok(patients)
    }

    /// Search patients by name (case-insensitive substring match)
    pub async fn search(&self, query: &str) -> Result<Vec<Patient>, String> {
        let all_patients = self.get_all().await.map_err(|e| format!("{:?}", e))?;

        let query_lower = query.to_lowercase();

        let filtered: Vec<Patient> = all_patients
            .into_iter()
            .filter(|patient| {
                if let Some(full_name) = patient.full_name() {
                    full_name.to_lowercase().contains(&query_lower)
                } else {
                    false
                }
            })
            .collect();

        Ok(filtered)
    }

    /// Delete a patient by ID
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadWrite).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::PATIENTS).map_err(|e| format!("{:?}", e))?;

        store.delete(JsValue::from_str(id)).map_err(|e| format!("{:?}", e))?;

        tx.await.map_err(|e| JsValue::from_str(&format!("{:?}", e))).map_err(|e| format!("{:?}", e))?;

        log!("Patient deleted successfully");
        Ok(())
    }

    /// Count total patients
    pub async fn count(&self) -> Result<u32, String> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadOnly).map_err(|e| format!("{:?}", e))?;

        let store = tx.object_store(stores::PATIENTS).map_err(|e| format!("{:?}", e))?;

        let count = store.count(None)?.await.map_err(|e| format!("{:?}", e))?;

        Ok(count as u32)
    }
}

// Yew context provider
#[derive(Clone)]
pub struct PatientStoreContext {
    pub store: Rc<PatientStore>,
}

impl PartialEq for PatientStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
    }
}

#[derive(Properties, PartialEq)]
pub struct PatientStoreProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PatientStoreProvider)]
pub fn patient_store_provider(props: &PatientStoreProviderProps) -> Html {
    let store = use_state(|| None::<Rc<PatientStore>>);

    {
        let store = store.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match PatientStore::new().await {
                    Ok(db) => {
                        log!("Patient store initialized");
                        store.set(Some(Rc::new(db)));
                    }
                    Err(e) => {
                        gloo_console::error!("Failed to initialize patient store:", e);
                    }
                }
            });
            || ()
        });
    }

    if let Some(store) = (*store).as_ref() {
        let context = PatientStoreContext {
            store: store.clone(),
        };

        html! {
            <ContextProvider<PatientStoreContext> context={context}>
                { for props.children.iter() }
            </ContextProvider<PatientStoreContext>>
        }
    } else {
        html! {
            <div class="flex h-full w-full items-center justify-center">
                <crate::components::Logo class="size-12 animate-pulse" />
            </div>
        }
    }
}

/// Hook to use patient store from context
#[hook]
pub fn use_patient_store() -> Rc<PatientStore> {
    use_context::<PatientStoreContext>()
        .expect("PatientStoreContext not found")
        .store
}
