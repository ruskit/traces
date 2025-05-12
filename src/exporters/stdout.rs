// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Stdout exporter implementation.
//!
//! This module provides functionality to export trace data to the standard output.
//! This exporter is particularly useful for development and debugging environments
//! where trace data can be viewed directly in the console.

use crate::{errors::TracesError, get_sampler};
use configs::{Configs, DynamicConfigs};
use opentelemetry::{KeyValue, global, propagation::TextMapCompositePropagator};
use opentelemetry_sdk::{
    Resource,
    propagation::{BaggagePropagator, TraceContextPropagator},
    trace::{RandomIdGenerator, SdkTracerProvider, TracerProviderBuilder},
};
use tracing::debug;

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
/// use configs::Configs;
///
/// fn main() {
///     let cfg = Configs::new();
///     stdout::install(&cfg).expect("Failed to install stdout exporter");
/// }
/// ```
pub fn install<T>(cfg: &Configs<T>) -> Result<SdkTracerProvider, TracesError>
where
    T: DynamicConfigs,
{
    let exporter = opentelemetry_stdout::SpanExporter::default();

    let provider = TracerProviderBuilder::default()
        .with_sampler(get_sampler(cfg))
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_resource(
            Resource::builder()
                .with_service_name(cfg.app.name.clone())
                .with_attribute(KeyValue::new(
                    "service.type",
                    cfg.trace.service_type.clone(),
                ))
                .with_attribute(KeyValue::new("environment", format!("{}", cfg.app.env)))
                .with_attribute(KeyValue::new("library.language", "rust"))
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

    global::set_tracer_provider(provider);

    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]));

    debug!("traces::install stdout tracer installed");

    Ok(provider)
}
