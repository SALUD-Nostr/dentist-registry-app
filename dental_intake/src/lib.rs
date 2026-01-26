//! Salud Dental - FHIR-compliant Dental Health Management System
//!
//! This library exposes the core modules for testing.

#![warn(clippy::all, clippy::style, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_errors_doc,
    clippy::future_not_send,
    clippy::type_repetition_in_bounds
)]

pub mod components;
pub mod error;
pub mod salud_note;
pub mod storage;

// Re-export commonly used types for testing
pub use error::AppError;
