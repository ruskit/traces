// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Trace sampling configuration.
//!
//! This module provides functionality to configure trace sampling strategies
//! based on environment and application configuration.

use configs::{app::AppConfigs, otlp::OTLPConfigs};
use opentelemetry_sdk::trace::Sampler;

/// Returns a trace sampler configured based on application environment and settings.
///
/// This function determines the appropriate sampling strategy:
/// - In local environments, it uses AlwaysOn sampling for complete visibility
/// - In other environments, it uses a parent-based sampling strategy with a configurable ratio
///
/// # Arguments
///
/// * `app` - Application configuration containing environment settings
/// * `otlp` - OpenTelemetry configuration containing sampling rate settings
///
/// # Returns
///
/// A configured `Sampler` instance appropriate for the environment
pub(crate) fn get_sampler(app: &AppConfigs, otlp: &OTLPConfigs) -> Sampler {
    if app.env.is_local() {
        return Sampler::AlwaysOn;
    }

    let sampler = Sampler::TraceIdRatioBased(otlp.exporter_rate_base);
    return Sampler::ParentBased(Box::new(sampler));
}
