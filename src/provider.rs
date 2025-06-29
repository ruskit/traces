// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Trace provider initialization module.
//!
//! This module provides functionality to initialize and configure the OpenTelemetry trace provider
//! based on the application configuration.

use crate::errors::TracesError;
use crate::exporters;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::info;

/// Initialize the OpenTelemetry trace provider based on feature flags.
///
/// This function selects and configures the appropriate tracer exporter based on enabled features:
/// - When both `otlp` and `stdout` features are enabled, OTLP takes precedence
/// - When only `otlp` is enabled, uses the OTLP gRPC exporter
/// - When only `stdout` is enabled, uses the stdout exporter for console output
/// - When no features are enabled, uses a no-op tracer
///
/// # Returns
///
/// * `Ok(SdkTracerProvider)` if initialization is successful
/// * `Err(TracesError)` if initialization fails or required features are not enabled
///
/// # Examples
///
/// ```no_run
/// use traces::provider;
///
/// fn main() {
///     let tracer_provider = provider::install().expect("Failed to initialize tracing");
/// }
/// ```
pub fn install() -> Result<SdkTracerProvider, TracesError> {
    info!("traces::install configuring tracer provider");

    #[cfg(all(feature = "otlp", feature = "stdout"))]
    {
        // When both features are enabled, prefer OTLP
        let tracer = exporters::otlp_grpc::install()?;
        return Ok(tracer);
    }

    #[cfg(all(feature = "otlp", not(feature = "stdout")))]
    {
        let tracer = exporters::otlp_grpc::install()?;
        return Ok(tracer);
    }

    #[cfg(all(feature = "stdout", not(feature = "otlp")))]
    {
        let tracer = exporters::stdout::install()?;
        return Ok(tracer);
    }

    #[cfg(not(any(feature = "stdout", feature = "otlp")))]
    return exporters::noop::install();
}
