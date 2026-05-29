//! FHIR AllergyIntolerance resource - simplified for salud-dental

use crate::datatypes::*;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Risk of harmful or undesirable physiological response to a substance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AllergyIntolerance {
    /// Logical id of this artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource type (always "AllergyIntolerance")
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// Business identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    /// active | inactive | resolved
    #[serde(skip_serializing_if = "Option::is_none", rename = "clinicalStatus")]
    pub clinical_status: Option<CodeableConcept>,

    /// allergy | intolerance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<AllergyCategory>,

    /// low | high | unable-to-assess
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<AllergyCriticality>,

    /// Code that identifies the allergy or intolerance (substance)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<CodeableConcept>,

    /// Who the allergy is for
    pub patient: Reference,

    /// Encounter when the allergy was asserted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,

    /// When the allergy was recorded
    #[serde(skip_serializing_if = "Option::is_none", rename = "recordedDate")]
    pub recorded_date: Option<DateTime<Utc>>,

    /// Additional narrative about the allergy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Annotation>>,
}

/// Category of an identified substance, product or class
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AllergyCategory {
    Food,
    Medication,
    Environment,
    Biologic,
}

/// Estimate of the potential clinical harm
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum AllergyCriticality {
    Low,
    High,
    UnableToAssess,
}

impl Default for AllergyIntolerance {
    fn default() -> Self {
        Self {
            id: None,
            resource_type: "AllergyIntolerance".to_string(),
            identifier: None,
            clinical_status: None,
            category: None,
            criticality: None,
            code: None,
            patient: Reference {
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
    fn test_allergy_builder() {
        let allergy = AllergyIntoleranceBuilder::default()
            .id("a-1")
            .category(AllergyCategory::Medication)
            .criticality(AllergyCriticality::High)
            .code(CodeableConcept {
                text: "Penicilina".to_string(),
                coding: None,
            })
            .patient(Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: None,
            })
            .build()
            .unwrap();

        assert_eq!(allergy.resource_type, "AllergyIntolerance");
        assert_eq!(allergy.category, Some(AllergyCategory::Medication));
    }

    #[test]
    fn test_allergy_serialization() {
        let allergy = AllergyIntolerance {
            code: Some(CodeableConcept {
                text: "Latex".to_string(),
                coding: None,
            }),
            patient: Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: None,
            },
            ..Default::default()
        };

        let json = serde_json::to_string(&allergy).unwrap();
        assert!(json.contains("\"resourceType\":\"AllergyIntolerance\""));

        let deserialized: AllergyIntolerance = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, allergy);
    }
}
