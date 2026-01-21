//! Simplified FHIR data types and resources for salud-dental
//!
//! This library provides minimal yet compliant FHIR R4 resources for:
//! - Patient registration and management
//! - Encounter scheduling and tracking
//! - Clinical impression recording
//!
//! All types include serde serialization support and builder patterns.

pub mod datatypes;
pub mod patient;
pub mod encounter;
pub mod clinical_impression;

// Re-export main types for convenience
pub use patient::{Patient, PatientBuilder, AdministrativeGender};
pub use encounter::{Encounter, EncounterBuilder, EncounterStatus, EncounterClass, EncounterParticipant};
pub use clinical_impression::{ClinicalImpression, ClinicalImpressionBuilder, ClinicalImpressionStatus, ClinicalImpressionFinding};

// Re-export common datatypes
pub use datatypes::{
    Identifier, HumanName, ContactPoint, ContactPointSystem, ContactPointUse,
    Address, CodeableConcept, Coding, Period, Reference, Annotation,
    FhirDate, FhirDateTime,
};
