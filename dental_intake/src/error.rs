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

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::JsError(err) => write!(f, "JavaScript error: {:?}", err),
            AppError::IdbError(err) => write!(f, "IndexedDB error: {:?}", err),
            AppError::SerdeError(err) => write!(f, "Serialization error: {:?}", err),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
