// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Stdout exporter implementation.
//!
//! This module provides functionality to export trace data to the standard output.
//! This exporter is particularly useful for development and debugging environments
//! where trace data can be viewed directly in the console.

use crate::{errors::TracesError, exporters::sampler::get_sampler};
use configs::{app::AppConfigs, otlp::OTLPConfigs};
use opentelemetry::{KeyValue, global, propagation::TextMapCompositePropagator};
use opentelemetry_sdk::{
    Resource,
    propagation::{BaggagePropagator, TraceContextPropagator},
    trace::{RandomIdGenerator, SdkTracerProvider, TracerProviderBuilder},
};
use tracing::info;

/// Installs the stdout exporter for OpenTelemetry tracing.
///
/// This function configures and installs an exporter that sends trace data
/// to the standard output, making it visible in the console logs.
///
/// # Arguments
///
/// * `cfg` - Application configuration containing trace settings
///
/// # Returns
///
/// * `Ok(())` if installation is successful
/// * `Err(TracesError)` if installation fails
///
/// # Examples
///
/// ```no_run
/// use traces::exporters::stdout;
///
/// fn main() {
///     let provider = stdout::install().expect("Failed to install stdout exporter");
/// }
/// ```
pub fn install() -> Result<SdkTracerProvider, TracesError> {
    let app_cfgs = AppConfigs::new();
    let otlp_cfgs = OTLPConfigs::new();

    let exporter = opentelemetry_stdout::SpanExporter::default();

    let provider = TracerProviderBuilder::default()
        .with_sampler(get_sampler(&app_cfgs, &otlp_cfgs))
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_resource(
            Resource::builder()
                .with_service_name(app_cfgs.name.clone())
                .with_attribute(KeyValue::new(
                    "service.namespace",
                    app_cfgs.namespace.clone(),
                ))
                .with_attribute(KeyValue::new("environment", format!("{}", app_cfgs.env)))
                .with_attribute(KeyValue::new("library.language", "rust"))
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

    global::set_tracer_provider(provider.clone());
    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]));

    info!("traces::install stdout tracer installed");

    Ok(provider)
}
