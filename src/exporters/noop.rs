// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! No-op exporter implementation.
//!
//! This module provides a no-op (no operation) tracer implementation that can be used
//! when tracing is disabled or when no specific exporter features are enabled.
//! It fulfills the tracer interface without performing any actual tracing operations.

use crate::errors::TracesError;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::info;

/// Installs a no-op tracer provider that doesn't export any telemetry data.
///
/// This function provides a fallback tracer implementation when no specific
/// exporter features (like stdout or otlp) are enabled, or when tracing is
/// disabled in the application configuration.
///
/// # Returns
///
/// * `Ok(SdkTracerProvider)` - A default tracer provider that doesn't export data
/// * `Err(TracesError)` - If installation fails (unlikely with no-op implementation)
///
/// # Examples
///
/// ```
/// use traces::exporters::noop;
///
/// fn main() {
///     let provider = noop::install().expect("Failed to install no-op tracer");
///     // Application continues with tracing disabled
/// }
/// ```
pub fn install() -> Result<SdkTracerProvider, TracesError> {
    info!("traces::install noop tracer installed");

    Ok(SdkTracerProvider::default())
}
