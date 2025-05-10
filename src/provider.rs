// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Trace provider initialization module.
//!
//! This module provides functionality to initialize and configure the OpenTelemetry trace provider
//! based on the application configuration.

use crate::errors::TracesError;
use configs::{Configs, DynamicConfigs, TraceExporterKind};
use tracing::{debug, error};

#[cfg(any(feature = "otlp", feature = "stdout"))]
use crate::exporters;

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
///     provider::init(&cfg).expect("Failed to initialize tracing");
/// }
/// ```
pub fn init<T>(cfg: &Configs<T>) -> Result<(), TracesError>
where
    T: DynamicConfigs,
{
    if !cfg.trace.enable {
        debug!("traces::init skipping trace export setup");
        return Ok(());
    }

    debug!("traces::init creating the tracer...");

    match cfg.trace.exporter {
        TraceExporterKind::Stdout => {
            #[cfg(feature = "stdout")]
            {
                exporters::stdout::install(cfg)
            }

            #[cfg(not(feature = "stdout"))]
            {
                error!("stdout traces required to configure features = [stdout]");
                Err(TracesError::InvalidFeaturesError)
            }
        }
        TraceExporterKind::OtlpGrpc => {
            #[cfg(feature = "otlp")]
            {
                exporters::otlp_grpc::install(cfg)
            }

            #[cfg(not(feature = "otlp"))]
            {
                error!("otlp traces required to configure features = [otlp]");
                Err(TracesError::InvalidFeaturesError)
            }
        }
    }
}
