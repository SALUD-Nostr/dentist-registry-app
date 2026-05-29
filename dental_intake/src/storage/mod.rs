//! `IndexedDB` storage layer for FHIR resources
//!
//! This module provides persistent storage for Patient, Encounter, and `ClinicalImpression`
//! resources using browser `IndexedDB`.

use yew::prelude::*;

mod allergy_store;
mod clinical_impression_store;
mod condition_store;
mod encounter_store;
mod patient_store;
mod procedure_store;

pub use allergy_store::{AllergyStoreProvider, use_allergy_store};
pub use clinical_impression_store::{
    ClinicalImpressionStoreProvider, use_clinical_impression_store,
};
pub use condition_store::{ConditionStoreProvider, use_condition_store};
pub use encounter_store::{
    EncounterStore, EncounterStoreProvider, use_encounter_store, use_encounter_store_version,
    use_notify_encounters_changed,
};
pub use patient_store::{
    PatientStore, PatientStoreProvider, use_notify_patients_changed, use_patient_store,
    use_patient_store_version,
};
pub use procedure_store::{ProcedureStoreProvider, use_procedure_store};

/// Database name for all FHIR resources
pub const DB_NAME: &str = "salud-dental-db";

/// Current database version. Bumped to 2 to add the intake-related stores
/// (allergies, conditions, procedures).
pub const DB_VERSION: u32 = 2;

/// Object store names
pub mod stores {
    pub const PATIENTS: &str = "patients";
    pub const ENCOUNTERS: &str = "encounters";
    pub const CLINICAL_IMPRESSIONS: &str = "clinical_impressions";
    pub const ALLERGIES: &str = "allergies";
    pub const CONDITIONS: &str = "conditions";
    pub const PROCEDURES: &str = "procedures";
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
                    <AllergyStoreProvider>
                        <ConditionStoreProvider>
                            <ProcedureStoreProvider>
                                { for props.children.iter() }
                            </ProcedureStoreProvider>
                        </ConditionStoreProvider>
                    </AllergyStoreProvider>
                </ClinicalImpressionStoreProvider>
            </EncounterStoreProvider>
        </PatientStoreProvider>
    }
}
