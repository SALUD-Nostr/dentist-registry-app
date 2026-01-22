//! IndexedDB storage layer for FHIR resources
//!
//! This module provides persistent storage for Patient, Encounter, and ClinicalImpression
//! resources using browser IndexedDB.

use yew::prelude::*;

mod patient_store;
mod encounter_store;
mod clinical_impression_store;

pub use patient_store::{PatientStore, use_patient_store, PatientStoreProvider};
pub use encounter_store::{EncounterStore, use_encounter_store, EncounterStoreProvider};
pub use clinical_impression_store::{ClinicalImpressionStore, use_clinical_impression_store, ClinicalImpressionStoreProvider};

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
