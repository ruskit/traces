// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Error types for the traces crate.
//!
//! This module defines all possible errors that can occur when working with distributed tracing.

use thiserror::Error;

/// Errors that can occur when working with distributed traces.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum TracesError {
    /// An unexpected internal error occurred.
    #[error("internal error")]
    InternalError,

    /// The requested exporter requires specific feature flags to be enabled.
    #[error("this exporter requires specific features")]
    InvalidFeaturesError,

    /// Error occurred during type conversion.
    #[error("conversion error")]
    ConversionError,

    /// Failed to create the OpenTelemetry exporter provider.
    #[error("failure to create the exporter provider")]
    ExporterProviderError,
}
