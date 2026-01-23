//! FHIR Encounter resource - simplified for salud-dental

use crate::datatypes::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// An interaction between a patient and healthcare provider(s)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Encounter {
    /// Logical id of this artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource type (always "Encounter")
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// Identifier(s) by which this encounter is known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    /// Current status of the encounter
    pub status: EncounterStatus,

    /// Classification of patient encounter
    pub class: EncounterClass,

    /// Specific type of encounter
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,

    /// The patient present at the encounter
    pub subject: Reference,

    /// List of participants involved in the encounter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant: Option<Vec<EncounterParticipant>>,

    /// The start and end time of the encounter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// Reason the encounter takes place (text)
    #[serde(skip_serializing_if = "Option::is_none", rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
}

/// Current state of the encounter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum EncounterStatus {
    Planned,
    Arrived,
    Triaged,
    InProgress,
    Onleave,
    Finished,
    Cancelled,
    EnteredInError,
    Unknown,
}

/// Classification of the encounter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EncounterClass {
    /// Ambulatory/Outpatient
    #[serde(rename = "AMB")]
    Ambulatory,

    /// Emergency
    #[serde(rename = "EMER")]
    Emergency,

    /// Field visit
    #[serde(rename = "FLD")]
    Field,

    /// Home health visit
    #[serde(rename = "HH")]
    HomeHealth,

    /// Inpatient encounter
    #[serde(rename = "IMP")]
    Inpatient,

    /// Inpatient acute
    #[serde(rename = "ACUTE")]
    Acute,

    /// Virtual encounter
    #[serde(rename = "VR")]
    Virtual,
}

/// Participants involved in the encounter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncounterParticipant {
    /// Role of participant in encounter
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,

    /// Period of time during the encounter participant was present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// Person involved in the encounter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Reference>,
}

impl Default for Encounter {
    fn default() -> Self {
        Self {
            id: None,
            resource_type: "Encounter".to_string(),
            identifier: None,
            status: EncounterStatus::Planned,
            class: EncounterClass::Ambulatory,
            type_: None,
            subject: Reference {
                reference: None,
                type_: Some("Patient".to_string()),
                display: None,
            },
            participant: None,
            period: None,
            reason_code: None,
        }
    }
}

impl Encounter {
    /// Create a new encounter with required fields
    pub fn new(status: EncounterStatus, class: EncounterClass, subject: Reference) -> Self {
        Self {
            status,
            class,
            subject,
            ..Default::default()
        }
    }

    /// Check if encounter is currently active
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            EncounterStatus::InProgress | EncounterStatus::Arrived | EncounterStatus::Triaged
        )
    }

    /// Check if encounter is completed
    pub fn is_completed(&self) -> bool {
        matches!(self.status, EncounterStatus::Finished)
    }

    /// Get the primary practitioner
    pub fn primary_practitioner(&self) -> Option<&Reference> {
        self.participant
            .as_ref()?
            .first()
            .and_then(|p| p.individual.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_encounter_builder() {
        let encounter = EncounterBuilder::default()
            .id("enc-123")
            .status(EncounterStatus::Planned)
            .class(EncounterClass::Ambulatory)
            .subject(Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: Some("Juan Pérez".to_string()),
            })
            .build()
            .unwrap();

        assert_eq!(encounter.id, Some("enc-123".to_string()));
        assert_eq!(encounter.status, EncounterStatus::Planned);
        assert!(!encounter.is_active());
    }

    #[test]
    fn test_encounter_serialization() {
        let encounter = Encounter {
            id: Some("e1".to_string()),
            resource_type: "Encounter".to_string(),
            identifier: None,
            status: EncounterStatus::InProgress,
            class: EncounterClass::Ambulatory,
            type_: Some(vec![CodeableConcept {
                text: "Dental Checkup".to_string(),
                coding: None,
            }]),
            subject: Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: Some("María García".to_string()),
            },
            participant: Some(vec![EncounterParticipant {
                type_: Some(vec![CodeableConcept {
                    text: "Primary Practitioner".to_string(),
                    coding: None,
                }]),
                period: None,
                individual: Some(Reference {
                    reference: Some("Practitioner/pr1".to_string()),
                    type_: Some("Practitioner".to_string()),
                    display: Some("Dr. Smith".to_string()),
                }),
            }]),
            period: Some(Period {
                start: Some(Utc::now()),
                end: None,
            }),
            reason_code: None,
        };

        let json = serde_json::to_string_pretty(&encounter).unwrap();
        assert!(json.contains("\"resourceType\": \"Encounter\""));
        assert!(json.contains("\"status\": \"in-progress\""));
        assert!(json.contains("\"class\": \"AMB\""));

        // Test deserialization
        let deserialized: Encounter = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.status, EncounterStatus::InProgress);
        assert_eq!(deserialized.class, EncounterClass::Ambulatory);
    }

    #[test]
    fn test_encounter_status_checks() {
        let mut encounter = Encounter::new(
            EncounterStatus::InProgress,
            EncounterClass::Ambulatory,
            Reference {
                reference: Some("Patient/p1".to_string()),
                type_: Some("Patient".to_string()),
                display: None,
            },
        );

        assert!(encounter.is_active());
        assert!(!encounter.is_completed());

        encounter.status = EncounterStatus::Finished;
        assert!(!encounter.is_active());
        assert!(encounter.is_completed());
    }
}
