//! Simplified FHIR data types and resources for salud-dental
//!
//! This library provides minimal yet compliant FHIR R4 resources for:
//! - Patient registration and management
//! - Encounter scheduling and tracking
//! - Clinical impression recording
//!
//! All types include serde serialization support and builder patterns.

pub mod clinical_impression;
pub mod datatypes;
pub mod encounter;
pub mod patient;

// Re-export main types for convenience
pub use clinical_impression::{
    ClinicalImpression, ClinicalImpressionBuilder, ClinicalImpressionFinding,
    ClinicalImpressionStatus,
};
pub use encounter::{
    Encounter, EncounterBuilder, EncounterClass, EncounterParticipant, EncounterStatus,
};
pub use patient::{AdministrativeGender, Patient, PatientBuilder};

// Re-export common datatypes
pub use datatypes::{
    Address, Annotation, CodeableConcept, Coding, ContactPoint, ContactPointSystem,
    ContactPointUse, FhirDate, FhirDateTime, HumanName, Identifier, Period, Reference,
};
