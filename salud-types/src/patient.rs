//! FHIR Patient resource - simplified for salud-dental

use crate::datatypes::*;
use chrono::NaiveDate;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Demographics and other administrative information about an individual receiving care
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Patient {
    /// Logical id of this artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource type (always "Patient")
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// An identifier for this patient
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Vec<Identifier>>,

    /// Whether this patient record is in active use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A name associated with the patient
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<HumanName>>,

    /// Contact details for the individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom: Option<Vec<ContactPoint>>,

    /// Administrative gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<AdministrativeGender>,

    /// Date of birth
    #[serde(skip_serializing_if = "Option::is_none", rename = "birthDate")]
    pub birth_date: Option<NaiveDate>,

    /// Address(es) for the individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<Address>>,
}

/// Administrative gender codes as defined by FHIR
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AdministrativeGender {
    Male,
    Female,
    Other,
    Unknown,
}

impl Default for Patient {
    fn default() -> Self {
        Self {
            id: None,
            resource_type: "Patient".to_string(),
            identifier: None,
            active: Some(true),
            name: None,
            telecom: None,
            gender: None,
            birth_date: None,
            address: None,
        }
    }
}

impl Patient {
    /// Create a new patient with required fields
    pub fn new() -> Self {
        Self::default()
    }

    /// Get patient's full name as text
    pub fn full_name(&self) -> Option<String> {
        self.name.as_ref()?.first().and_then(|n| {
            if let Some(text) = &n.text {
                Some(text.clone())
            } else {
                // Construct from parts
                let mut parts = Vec::new();
                if let Some(given) = &n.given {
                    parts.extend(given.iter().cloned());
                }
                if let Some(family) = &n.family {
                    parts.push(family.clone());
                }
                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join(" "))
                }
            }
        })
    }

    /// Get primary phone number
    pub fn primary_phone(&self) -> Option<String> {
        self.telecom.as_ref()?.iter()
            .find(|cp| matches!(cp.system, ContactPointSystem::Phone))
            .map(|cp| cp.value.clone())
    }

    /// Get primary email
    pub fn primary_email(&self) -> Option<String> {
        self.telecom.as_ref()?.iter()
            .find(|cp| matches!(cp.system, ContactPointSystem::Email))
            .map(|cp| cp.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patient_builder() {
        let patient = PatientBuilder::default()
            .id("patient-123")
            .name(vec![HumanName {
                text: Some("Juan Pérez".to_string()),
                family: Some("Pérez".to_string()),
                given: Some(vec!["Juan".to_string()]),
            }])
            .gender(AdministrativeGender::Male)
            .birth_date(NaiveDate::from_ymd_opt(1990, 5, 15).unwrap())
            .build()
            .unwrap();

        assert_eq!(patient.id, Some("patient-123".to_string()));
        assert_eq!(patient.gender, Some(AdministrativeGender::Male));
        assert_eq!(patient.full_name(), Some("Juan Pérez".to_string()));
    }

    #[test]
    fn test_patient_serialization() {
        let patient = Patient {
            id: Some("p1".to_string()),
            resource_type: "Patient".to_string(),
            identifier: Some(vec![Identifier {
                system: Some("http://hospital.example.org".to_string()),
                value: "12345".to_string(),
            }]),
            active: Some(true),
            name: Some(vec![HumanName {
                text: Some("María García".to_string()),
                family: Some("García".to_string()),
                given: Some(vec!["María".to_string()]),
            }]),
            telecom: Some(vec![ContactPoint {
                system: ContactPointSystem::Phone,
                value: "+34123456789".to_string(),
                use_: Some(ContactPointUse::Mobile),
            }]),
            gender: Some(AdministrativeGender::Female),
            birth_date: Some(NaiveDate::from_ymd_opt(1985, 3, 20).unwrap()),
            address: None,
        };

        let json = serde_json::to_string_pretty(&patient).unwrap();
        assert!(json.contains("\"resourceType\": \"Patient\""));
        assert!(json.contains("\"gender\": \"female\""));

        // Test deserialization
        let deserialized: Patient = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, patient);
    }
}
