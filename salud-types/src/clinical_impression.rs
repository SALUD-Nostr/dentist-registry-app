//! FHIR ClinicalImpression resource - simplified for salud-dental

use crate::datatypes::*;
use chrono::DateTime;
use chrono::Utc;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A clinical assessment performed to determine what problem(s) may affect the patient
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ClinicalImpression {
    /// Logical id of this artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource type (always "ClinicalImpression")
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// Business identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    /// Workflow status
    pub status: ClinicalImpressionStatus,

    /// Reason for current status
    #[serde(skip_serializing_if = "Option::is_none", rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// Patient or group assessed
    pub subject: Reference,

    /// Encounter created as part of
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Reference>,

    /// Time of assessment
    #[serde(skip_serializing_if = "Option::is_none", rename = "effectiveDateTime")]
    pub effective_date_time: Option<DateTime<Utc>>,

    /// When the assessment was documented
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,

    /// The clinician performing the assessment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessor: Option<Reference>,

    /// Summary of the assessment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Specific findings or diagnoses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<Vec<ClinicalImpressionFinding>>,

    /// Comments made about the impression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Annotation>>,
}

/// Workflow status of the clinical impression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ClinicalImpressionStatus {
    /// The assessment is still ongoing
    InProgress,

    /// The assessment is done and the results are final
    Completed,

    /// This assessment was never actually done
    EnteredInError,
}

/// Specific findings or diagnoses identified
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClinicalImpressionFinding {
    /// What was found (text or coded)
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "itemCodeableConcept"
    )]
    pub item_codeable_concept: Option<CodeableConcept>,

    /// Which investigations support finding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<String>,
}

impl Default for ClinicalImpression {
    fn default() -> Self {
        Self {
            id: None,
            resource_type: "ClinicalImpression".to_string(),
            identifier: None,
            status: ClinicalImpressionStatus::InProgress,
            status_reason: None,
            subject: Reference {
                reference: None,
                type_: Some("Patient".to_string()),
                display: None,
            },
            encounter: None,
            effective_date_time: None,
            date: None,
            assessor: None,
            summary: None,
            finding: None,
            note: None,
        }
    }
}

impl ClinicalImpression {
    /// Create a new clinical impression with required fields
    pub fn new(status: ClinicalImpressionStatus, subject: Reference) -> Self {
        Self {
            status,
            subject,
            ..Default::default()
        }
    }

    /// Check if the assessment is completed
    pub fn is_completed(&self) -> bool {
        matches!(self.status, ClinicalImpressionStatus::Completed)
    }

    /// Check if the assessment is in progress
    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, ClinicalImpressionStatus::InProgress)
    }

    /// Add a finding to the clinical impression
    pub fn add_finding(&mut self, finding: ClinicalImpressionFinding) {
        if let Some(findings) = &mut self.finding {
            findings.push(finding);
        } else {
            self.finding = Some(vec![finding]);
        }
    }

    /// Add a note to the clinical impression
    pub fn add_note(&mut self, note: Annotation) {
        if let Some(notes) = &mut self.note {
            notes.push(note);
        } else {
            self.note = Some(vec![note]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clinical_impression_builder() {
        let impression = ClinicalImpressionBuilder::default()
            .id("ci-123")
            .status(ClinicalImpressionStatus::Completed)
            .subject(Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: Some("Juan Pérez".to_string()),
            })
            .summary("Patient presents with dental pain in upper right molar")
            .build()
            .unwrap();

        assert_eq!(impression.id, Some("ci-123".to_string()));
        assert!(impression.is_completed());
        assert!(!impression.is_in_progress());
    }

    #[test]
    fn test_clinical_impression_serialization() {
        let impression = ClinicalImpression {
            id: Some("ci1".to_string()),
            resource_type: "ClinicalImpression".to_string(),
            identifier: None,
            status: ClinicalImpressionStatus::Completed,
            status_reason: None,
            subject: Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: Some("María García".to_string()),
            },
            encounter: Some(Reference {
                reference: Some("Encounter/e1".to_string()),
                type_: Some("Encounter".to_string()),
                display: None,
            }),
            effective_date_time: Some(Utc::now()),
            date: Some(Utc::now()),
            assessor: Some(Reference {
                reference: Some("Practitioner/pr1".to_string()),
                type_: Some("Practitioner".to_string()),
                display: Some("Dr. Smith".to_string()),
            }),
            summary: Some("Routine dental checkup completed. No significant findings.".to_string()),
            finding: Some(vec![ClinicalImpressionFinding {
                item_codeable_concept: Some(CodeableConcept {
                    text: "Healthy teeth and gums".to_string(),
                    coding: None,
                }),
                basis: Some("Visual examination and X-rays".to_string()),
            }]),
            note: None,
        };

        let json = serde_json::to_string_pretty(&impression).unwrap();
        assert!(json.contains("\"resourceType\": \"ClinicalImpression\""));
        assert!(json.contains("\"status\": \"completed\""));

        // Test deserialization
        let deserialized: ClinicalImpression = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.status, ClinicalImpressionStatus::Completed);
    }

    #[test]
    fn test_add_finding_and_note() {
        let mut impression = ClinicalImpression::new(
            ClinicalImpressionStatus::InProgress,
            Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: None,
            },
        );

        // Add finding
        impression.add_finding(ClinicalImpressionFinding {
            item_codeable_concept: Some(CodeableConcept {
                text: "Cavity detected".to_string(),
                coding: None,
            }),
            basis: Some("X-ray imaging".to_string()),
        });

        assert_eq!(impression.finding.as_ref().unwrap().len(), 1);

        // Add note
        impression.add_note(Annotation {
            time: Some(Utc::now()),
            text: "Patient should schedule follow-up for filling".to_string(),
        });

        assert_eq!(impression.note.as_ref().unwrap().len(), 1);
    }
}
