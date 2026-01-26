//! Helper functions for working with FHIR-containing NostrNotes (kind=82)
//!
//! All SaludNotes use `kind = 82` and include a `["fhir", "<ResourceType>"]` tag.

use nostr_minions::nostro2::NostrNote;
use nostr_minions::nostro2_signer::keypair::NostrKeypair;
use serde::{Deserialize, Serialize};

/// Helper functions for working with SaludNotes
pub struct SaludNote;

impl SaludNote {
    /// Extract FHIR resource type from tags
    pub fn get_fhir_type(note: &NostrNote) -> Result<String, SaludNoteError> {
        note.tags
            .0
            .iter()
            .find(|tag| tag.first().map_or(false, |s| s == "fhir"))
            .and_then(|tag| tag.get(1))
            .cloned()
            .ok_or(SaludNoteError::MissingFhirTag)
    }

    /// Parse FHIR resource from note content
    pub fn parse_fhir<T>(note: &NostrNote) -> Result<T, SaludNoteError>
    where
        T: for<'de> Deserialize<'de>,
    {
        if note.kind != 82 {
            return Err(SaludNoteError::InvalidKind(note.kind));
        }

        serde_json::from_str(&note.content)
            .map_err(|e| SaludNoteError::ParseError(e.to_string()))
    }

    /// Create a NostrNote from FHIR resource
    pub fn from_fhir<T>(
        resource: &T,
        fhir_type: &str,
        keypair: &NostrKeypair,
    ) -> Result<NostrNote, SaludNoteError>
    where
        T: Serialize,
    {
        let content = serde_json::to_string(resource)
            .map_err(|e| SaludNoteError::SerializeError(e.to_string()))?;

        let mut note = NostrNote {
            content,
            kind: 82,
            created_at: current_timestamp(),
            ..Default::default()
        };

        // Set FHIR tag
        note.tags.0 = vec![vec![
            "fhir".to_string(),
            fhir_type.to_string(),
        ]];

        keypair
            .sign_note(&mut note)
            .map_err(|e| SaludNoteError::SigningError(format!("{:?}", e)))?;

        Ok(note)
    }
}

/// Get current Unix timestamp in seconds
fn current_timestamp() -> i64 {
    (js_sys::Date::now() / 1000.0) as i64
}

#[derive(Debug)]
pub enum SaludNoteError {
    InvalidKind(u32),
    MissingFhirTag,
    ParseError(String),
    SerializeError(String),
    SigningError(String),
}

impl std::fmt::Display for SaludNoteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaludNoteError::InvalidKind(kind) => write!(f, "Invalid note kind: {}", kind),
            SaludNoteError::MissingFhirTag => write!(f, "Missing FHIR tag"),
            SaludNoteError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SaludNoteError::SerializeError(msg) => write!(f, "Serialize error: {}", msg),
            SaludNoteError::SigningError(msg) => write!(f, "Signing error: {}", msg),
        }
    }
}

impl std::error::Error for SaludNoteError {}
