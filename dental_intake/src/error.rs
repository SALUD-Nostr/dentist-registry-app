//! Unified error handling for the application

use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum AppError {
    /// JavaScript errors
    JsError(JsValue),
    /// IndexedDB errors
    IdbError(idb::Error),
    /// Serialization/deserialization errors
    SerdeError(serde_wasm_bindgen::Error),
    /// Resource not found
    NotFound(String),
    /// SaludNote errors
    SaludNoteError(crate::salud_note::SaludNoteError),
    /// No keypair available for signing
    NoKeypair,
    /// Resource missing required ID field
    MissingResourceId,
}

impl From<JsValue> for AppError {
    fn from(err: JsValue) -> Self {
        AppError::JsError(err)
    }
}

impl From<idb::Error> for AppError {
    fn from(err: idb::Error) -> Self {
        AppError::IdbError(err)
    }
}

impl From<serde_wasm_bindgen::Error> for AppError {
    fn from(err: serde_wasm_bindgen::Error) -> Self {
        AppError::SerdeError(err)
    }
}

impl From<crate::salud_note::SaludNoteError> for AppError {
    fn from(err: crate::salud_note::SaludNoteError) -> Self {
        AppError::SaludNoteError(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::JsError(err) => write!(f, "JavaScript error: {:?}", err),
            AppError::IdbError(err) => write!(f, "IndexedDB error: {:?}", err),
            AppError::SerdeError(err) => write!(f, "Serialization error: {:?}", err),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::SaludNoteError(err) => write!(f, "SaludNote error: {}", err),
            AppError::NoKeypair => write!(f, "No keypair available for signing"),
            AppError::MissingResourceId => write!(f, "Resource missing required ID field"),
        }
    }
}

impl std::error::Error for AppError {}
