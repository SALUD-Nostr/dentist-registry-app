//! FHIR Condition resource - simplified for salud-dental

use crate::datatypes::*;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A clinical condition, problem, diagnosis, or other event/situation/concept
/// that has risen to a level of concern (used here for systemic conditions).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Condition {
    /// Logical id of this artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource type (always "Condition")
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// Business identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    /// active | recurrence | relapse | inactive | remission | resolved
    #[serde(skip_serializing_if = "Option::is_none", rename = "clinicalStatus")]
    pub clinical_status: Option<CodeableConcept>,

    /// problem-list-item | encounter-diagnosis (category)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<CodeableConcept>>,

    /// Identification of the condition, problem or diagnosis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CodeableConcept>,

    /// Who has the condition
    pub subject: Reference,

    /// Encounter the condition was asserted in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,

    /// When the condition was recorded
    #[serde(skip_serializing_if = "Option::is_none", rename = "recordedDate")]
    pub recorded_date: Option<DateTime<Utc>>,

    /// Additional information about the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Annotation>>,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            id: None,
            resource_type: "Condition".to_string(),
            identifier: None,
            clinical_status: None,
            category: None,
            code: None,
            subject: Reference {
                reference: None,
                type_: Some("Patient".to_string()),
                display: None,
            },
            encounter: None,
            recorded_date: None,
            note: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_builder() {
        let condition = ConditionBuilder::default()
            .id("c-1")
            .code(CodeableConcept {
                text: "Diabetes".to_string(),
                coding: None,
            })
            .subject(Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: None,
            })
            .build()
            .unwrap();

        assert_eq!(condition.resource_type, "Condition");
        assert_eq!(
            condition.code.unwrap().text,
            "Diabetes".to_string()
        );
    }

    #[test]
    fn test_condition_serialization() {
        let condition = Condition {
            code: Some(CodeableConcept {
                text: "Hipertensión".to_string(),
                coding: None,
            }),
            subject: Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: None,
            },
            ..Default::default()
        };

        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains("\"resourceType\":\"Condition\""));

        let deserialized: Condition = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, condition);
    }
}
