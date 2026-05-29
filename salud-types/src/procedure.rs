//! FHIR Procedure resource - simplified for salud-dental

use crate::datatypes::*;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// An action that is or was performed on or for a patient (used here to record
/// a past dental visit / last visit to the dentist).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Procedure {
    /// Logical id of this artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource type (always "Procedure")
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// Business identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    /// preparation | in-progress | completed | entered-in-error | ...
    pub status: ProcedureStatus,

    /// Identification of the procedure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CodeableConcept>,

    /// Who the procedure was performed on
    pub subject: Reference,

    /// Encounter the procedure is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,

    /// When the procedure was performed
    #[serde(skip_serializing_if = "Option::is_none", rename = "performedDateTime")]
    pub performed_date_time: Option<DateTime<Utc>>,

    /// Additional information about the procedure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Annotation>>,
}

/// Status of a procedure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum ProcedureStatus {
    Preparation,
    InProgress,
    NotDone,
    OnHold,
    Stopped,
    Completed,
    EnteredInError,
    Unknown,
}

impl Default for Procedure {
    fn default() -> Self {
        Self {
            id: None,
            resource_type: "Procedure".to_string(),
            identifier: None,
            status: ProcedureStatus::Completed,
            code: None,
            subject: Reference {
                reference: None,
                type_: Some("Patient".to_string()),
                display: None,
            },
            encounter: None,
            performed_date_time: None,
            note: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedure_builder() {
        let procedure = ProcedureBuilder::default()
            .id("pr-1")
            .status(ProcedureStatus::Completed)
            .code(CodeableConcept {
                text: "Limpieza dental".to_string(),
                coding: None,
            })
            .subject(Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: None,
            })
            .build()
            .unwrap();

        assert_eq!(procedure.resource_type, "Procedure");
        assert_eq!(procedure.status, ProcedureStatus::Completed);
    }

    #[test]
    fn test_procedure_serialization() {
        let procedure = Procedure {
            code: Some(CodeableConcept {
                text: "Última visita al dentista".to_string(),
                coding: None,
            }),
            subject: Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: None,
            },
            ..Default::default()
        };

        let json = serde_json::to_string(&procedure).unwrap();
        assert!(json.contains("\"resourceType\":\"Procedure\""));
        assert!(json.contains("\"status\":\"completed\""));

        let deserialized: Procedure = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, procedure);
    }
}
