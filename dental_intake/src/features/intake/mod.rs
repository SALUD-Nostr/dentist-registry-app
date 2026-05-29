//! Medical intake feature - captures the key initial clinical questions
//! (age, reason for consult, last dental visit, allergies, systemic
//! conditions) and stores them as FHIR resources.

mod form;

pub use form::MedicalIntakeForm;
