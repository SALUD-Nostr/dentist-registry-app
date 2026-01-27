//! Unified error handling for the application

use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum AppError {
    /// JavaScript errors
    JsError(JsValue),
    /// `IndexedDB` errors
    IdbError(idb::Error),
    /// Serialization/deserialization errors
    SerdeError(serde_wasm_bindgen::Error),
    /// Resource not found
    NotFound(String),
    /// `SaludNote` errors
    SaludNoteError(crate::salud_note::SaludNoteError),
    /// No keypair available for signing
    NoKeypair,
    /// Resource missing required ID field
    MissingResourceId,
}

impl From<JsValue> for AppError {
    fn from(err: JsValue) -> Self {
        Self::JsError(err)
    }
}

impl From<idb::Error> for AppError {
    fn from(err: idb::Error) -> Self {
        Self::IdbError(err)
    }
}

impl From<serde_wasm_bindgen::Error> for AppError {
    fn from(err: serde_wasm_bindgen::Error) -> Self {
        Self::SerdeError(err)
    }
}

impl From<crate::salud_note::SaludNoteError> for AppError {
    fn from(err: crate::salud_note::SaludNoteError) -> Self {
        Self::SaludNoteError(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsError(err) => write!(f, "JavaScript error: {err:?}"),
            Self::IdbError(err) => write!(f, "IndexedDB error: {err:?}"),
            Self::SerdeError(err) => write!(f, "Serialization error: {err:?}"),
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::SaludNoteError(err) => write!(f, "SaludNote error: {err}"),
            Self::NoKeypair => write!(f, "No keypair available for signing"),
            Self::MissingResourceId => write!(f, "Resource missing required ID field"),
        }
    }
}

impl std::error::Error for AppError {}
