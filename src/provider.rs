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

/// Initialize the OpenTelemetry trace provider based on the provided configuration.
///
/// This function selects and configures the appropriate tracer exporter (stdout or OTLP gRPC)
/// based on the application configuration. If tracing is disabled in the configuration,
/// this function will skip the setup process.
///
/// # Arguments
///
/// * `cfg` - Application configuration containing tracing settings
///
/// # Returns
///
/// * `Ok(())` if initialization is successful or tracing is disabled
/// * `Err(TracesError)` if initialization fails or required features are not enabled
///
/// # Examples
///
/// ```rust
/// use traces::provider;
/// use configs::Configs;
///
/// fn main() {
///     let cfg = Configs::new();
///     provider::install().expect("Failed to initialize tracing");
/// }
/// ```
pub fn install() -> Result<SdkTracerProvider, TracesError> {
    info!("traces::install configuring tracer provider");

    #[cfg(feature = "stdout")]
    {
        let tracer = exporters::stdout::install()?;
        Ok(tracer)
    }

    #[cfg(feature = "otlp")]
    {
        let tracer = exporters::otlp_grpc::install()?;
        Ok(tracer)
    }

    #[cfg(not(any(feature = "stdout", feature = "otlp")))]
    return exporters::noop::install();
}
