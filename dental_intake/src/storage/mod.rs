//! IndexedDB storage layer for FHIR resources
//!
//! This module provides persistent storage for Patient, Encounter, and ClinicalImpression
//! resources using browser IndexedDB.

use yew::prelude::*;

mod clinical_impression_store;
mod encounter_store;
mod patient_store;

pub use clinical_impression_store::{
    ClinicalImpressionStoreProvider, use_clinical_impression_store,
};
pub use encounter_store::{
    EncounterStore, EncounterStoreProvider, use_encounter_store, use_encounter_store_version,
    use_notify_encounters_changed,
};
pub use patient_store::{
    PatientStore, PatientStoreProvider, use_patient_store, use_patient_store_version,
    use_notify_patients_changed,
};

/// Database name for all FHIR resources
pub const DB_NAME: &str = "salud-dental-db";

/// Current database version
pub const DB_VERSION: u32 = 1;

/// Object store names
pub mod stores {
    pub const PATIENTS: &str = "patients";
    pub const ENCOUNTERS: &str = "encounters";
    pub const CLINICAL_IMPRESSIONS: &str = "clinical_impressions";
}

/// Combined storage provider that wraps all store contexts
#[derive(Properties, PartialEq)]
pub struct StorageProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(StorageProvider)]
pub fn storage_provider(props: &StorageProviderProps) -> Html {
    html! {
        <PatientStoreProvider>
            <EncounterStoreProvider>
                <ClinicalImpressionStoreProvider>
                    { for props.children.iter() }
                </ClinicalImpressionStoreProvider>
            </EncounterStoreProvider>
        </PatientStoreProvider>
    }
}
