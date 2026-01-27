//! Patient storage using `IndexedDB`

use gloo_console::log;
use idb::{Database, Factory, TransactionMode};
use salud_types::Patient;
use serde_wasm_bindgen::{from_value, to_value};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::{DB_NAME, DB_VERSION, stores};
use crate::error::AppError;

#[derive(Clone)]
pub struct PatientStore {
    pub db: Rc<Database>,
}

impl PatientStore {
    /// Open or create the database
    pub async fn new() -> Result<Self, AppError> {
        let factory = Factory::new()?;

        let mut open_request = factory.open(DB_NAME, Some(DB_VERSION))?;

        open_request.on_upgrade_needed(|event| {
            let database = match idb::DatabaseEvent::database(&event) {
                Ok(db) => db,
                Err(e) => {
                    gloo_console::error!("Error getting database:", format!("{:?}", e));
                    return;
                }
            };

            // Create patients object store if it doesn't exist
            if !database
                .store_names()
                .iter()
                .any(|name| name == stores::PATIENTS)
                && let Ok(store) =
                    database.create_object_store(stores::PATIENTS, idb::ObjectStoreParams::new())
                {
                    let mut params = idb::IndexParams::new();
                    params.unique(true);
                    if let Err(e) =
                        store.create_index("by_id", idb::KeyPath::new_single("id"), Some(params))
                    {
                        gloo_console::error!("Error creating index:", format!("{:?}", e));
                    }
                }

            // Create encounters object store
            if !database
                .store_names()
                .iter()
                .any(|name| name == stores::ENCOUNTERS)
                && let Ok(store) =
                    database.create_object_store(stores::ENCOUNTERS, idb::ObjectStoreParams::new())
                {
                    let mut id_params = idb::IndexParams::new();
                    id_params.unique(true);
                    if let Err(e) =
                        store.create_index("by_id", idb::KeyPath::new_single("id"), Some(id_params))
                    {
                        gloo_console::error!("Error creating index:", format!("{:?}", e));
                    }

                    let mut patient_params = idb::IndexParams::new();
                    patient_params.unique(false);
                    if let Err(e) = store.create_index(
                        "by_patient",
                        idb::KeyPath::new_single("subject.reference"),
                        Some(patient_params),
                    ) {
                        gloo_console::error!("Error creating index:", format!("{:?}", e));
                    }
                }

            // Create clinical impressions object store
            if !database
                .store_names()
                .iter()
                .any(|name| name == stores::CLINICAL_IMPRESSIONS)
                && let Ok(store) = database.create_object_store(
                    stores::CLINICAL_IMPRESSIONS,
                    idb::ObjectStoreParams::new(),
                ) {
                    let mut id_params = idb::IndexParams::new();
                    id_params.unique(true);
                    if let Err(e) =
                        store.create_index("by_id", idb::KeyPath::new_single("id"), Some(id_params))
                    {
                        gloo_console::error!("Error creating index:", format!("{:?}", e));
                    }

                    let mut encounter_params = idb::IndexParams::new();
                    encounter_params.unique(false);
                    if let Err(e) = store.create_index(
                        "by_encounter",
                        idb::KeyPath::new_single("encounter.reference"),
                        Some(encounter_params),
                    ) {
                        gloo_console::error!("Error creating index:", format!("{:?}", e));
                    }
                }
        });

        let db = open_request.await?;

        Ok(Self { db: Rc::new(db) })
    }

    /// Save a patient to the database
    pub async fn save(
        &self,
        patient: &Patient,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<(), AppError> {
        // Wrap in SaludNote
        let note = crate::salud_note::SaludNote::from_fhir(patient, "Patient", keypair)?;

        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadWrite)?;

        let store = tx.object_store(stores::PATIENTS)?;

        let note_value = to_value(&note)?;

        let id = patient.id.as_ref().ok_or(AppError::MissingResourceId)?;
        store.put(&note_value, Some(&JsValue::from_str(id)))?;

        tx.await?;

        log!("Patient saved successfully");
        Ok(())
    }

    /// Get a patient by ID
    pub async fn get(&self, id: &str) -> Result<Option<Patient>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::PATIENTS)?;

        let value = store.get(JsValue::from_str(id))?.await?;

        match value {
            Some(js_val) if !js_val.is_undefined() && !js_val.is_null() => {
                let note: nostr_minions::nostro2::NostrNote = from_value(js_val)?;
                let patient = crate::salud_note::SaludNote::parse_fhir(&note)?;
                Ok(Some(patient))
            }
            _ => Ok(None),
        }
    }

    /// Get all patients
    pub async fn get_all(&self) -> Result<Vec<Patient>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::PATIENTS)?;

        let values = store.get_all(None, None)?.await?;

        let mut patients = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone())
                && let Ok(patient) = crate::salud_note::SaludNote::parse_fhir(&note) {
                    patients.push(patient);
                }
        }

        Ok(patients)
    }

    /// Get all patient notes (raw `NostrNotes`)
    pub async fn get_all_notes(&self) -> Result<Vec<nostr_minions::nostro2::NostrNote>, AppError> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::PATIENTS)?;

        let values = store.get_all(None, None)?.await?;

        let mut notes = Vec::new();
        for value in &values {
            if let Ok(note) = from_value::<nostr_minions::nostro2::NostrNote>(value.clone()) {
                notes.push(note);
            }
        }

        Ok(notes)
    }

    /// Search patients by name (case-insensitive substring match)
    pub async fn search(&self, query: &str) -> Result<Vec<Patient>, AppError> {
        let all_patients = self.get_all().await?;

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
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadWrite)?;

        let store = tx.object_store(stores::PATIENTS)?;

        store.delete(JsValue::from_str(id))?;

        tx.await?;

        log!("Patient deleted successfully");
        Ok(())
    }

    /// Count total patients
    pub async fn count(&self) -> Result<u32, AppError> {
        let tx = self
            .db
            .transaction(&[stores::PATIENTS], TransactionMode::ReadOnly)?;

        let store = tx.object_store(stores::PATIENTS)?;

        let count = store.count(None)?.await?;

        Ok(count as u32)
    }
}

// Yew context provider
#[derive(Clone)]
pub struct PatientStoreContext {
    pub store: Rc<PatientStore>,
    pub version: UseStateHandle<u32>,
    // Cache of NostrNotes in memory
    pub notes_cache: UseStateHandle<Vec<nostr_minions::nostro2::NostrNote>>,
}

impl PartialEq for PatientStoreContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store)
            && *self.version == *other.version
            && *self.notes_cache == *other.notes_cache
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
    let version = use_state(|| 0u32);
    let notes_cache = use_state(Vec::new);

    // Initialize store
    {
        let store = store.clone();
        use_effect_with((), move |()| {
            wasm_bindgen_futures::spawn_local(async move {
                match PatientStore::new().await {
                    Ok(db) => {
                        log!("Patient store initialized");
                        store.set(Some(Rc::new(db)));
                    }
                    Err(e) => {
                        gloo_console::error!(
                            "Failed to initialize patient store:",
                            format!("{:?}", e)
                        );
                    }
                }
            });
            || ()
        });
    }

    // Load notes into cache when store is ready or version changes
    {
        let store_opt = (*store).clone();
        let notes_cache = notes_cache.clone();
        let version_val = *version;

        use_effect_with((store_opt.is_some(), version_val), move |_| {
            if let Some(store) = store_opt {
                let notes_cache = notes_cache.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match store.get_all_notes().await {
                        Ok(all_notes) => {
                            notes_cache.set(all_notes);
                        }
                        Err(e) => {
                            gloo_console::error!(
                                "Failed to load patient notes:",
                                format!("{:?}", e)
                            );
                        }
                    }
                });
            }
            || ()
        });
    }

    if let Some(store) = (*store).as_ref() {
        let context = PatientStoreContext {
            store: store.clone(),
            version,
            notes_cache,
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

/// Hook to get the patient store version for reactive updates
#[hook]
pub fn use_patient_store_version() -> u32 {
    *use_context::<PatientStoreContext>()
        .expect("PatientStoreContext not found")
        .version
}

/// Hook to notify that patients have been updated
#[hook]
pub fn use_notify_patients_changed() -> Callback<()> {
    let context = use_context::<PatientStoreContext>().expect("PatientStoreContext not found");

    let version = context.version;

    Callback::from(move |()| {
        version.set(*version + 1);
    })
}

/// Hook to access raw patient `NostrNotes`
#[hook]
pub fn use_patient_notes() -> Vec<nostr_minions::nostro2::NostrNote> {
    let context = use_context::<PatientStoreContext>().expect("PatientStoreContext not found");
    (*context.notes_cache).clone()
}

/// Hook to get a specific patient note by ID
#[hook]
pub fn use_patient_note(id: &str) -> Option<nostr_minions::nostro2::NostrNote> {
    let notes = use_patient_notes();
    let id = id.to_string();
    notes.into_iter().find(|note| {
        if let Ok(patient) = crate::salud_note::SaludNote::parse_fhir::<Patient>(note) {
            patient
                .id
                .as_ref() == Some(&id)
        } else {
            false
        }
    })
}
