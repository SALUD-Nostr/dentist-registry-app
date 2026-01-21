//! Simplified FHIR data types for salud-dental
//! Minimal yet compliant with FHIR R4 specification

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Basic identifier for resources
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Identifier {
    /// The namespace for the identifier value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// The unique value
    pub value: String,
}

/// Human name with text representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HumanName {
    /// Text representation of the full name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Family name (surname)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,

    /// Given names (first name, middle names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given: Option<Vec<String>>,
}

/// Contact point (phone, email, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactPoint {
    /// phone | email | fax | sms
    #[serde(rename = "system")]
    pub system: ContactPointSystem,

    /// The actual contact value
    pub value: String,

    /// home | work | mobile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_: Option<ContactPointUse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContactPointSystem {
    Phone,
    Email,
    Fax,
    Sms,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContactPointUse {
    Home,
    Work,
    Mobile,
}

/// Simple address
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    /// Full address as text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Street address lines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<Vec<String>>,

    /// City
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// State/Province
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Postal/Zip code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// CodeableConcept - simplified to just text
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeableConcept {
    /// Plain text representation
    pub text: String,

    /// Optional coding information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding: Option<Vec<Coding>>,
}

/// Coding - reference to a terminology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coding {
    /// Identity of the terminology system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Symbol/code in the system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Display text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

/// Time period with start and end
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Period {
    /// Start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,

    /// End time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
}

/// Reference to another resource
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reference {
    /// Relative, internal or absolute URL reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Type of resource (e.g., "Patient", "Practitioner")
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// Text alternative for the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

/// Text annotation with author
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
    /// When the annotation was made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,

    /// The annotation text
    pub text: String,
}

/// FHIR date type
pub type FhirDate = NaiveDate;

/// FHIR dateTime type
pub type FhirDateTime = DateTime<Utc>;
