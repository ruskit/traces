// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! OTLP gRPC exporter implementation.
//!
//! This module provides functionality to export trace data using the OpenTelemetry Protocol (OTLP)
//! over gRPC. This exporter is suitable for production environments where traces need to be
//! sent to an OpenTelemetry collector or compatible backend.

use crate::{errors::TracesError, exporters::sampler::get_sampler};
use configs::{app::AppConfigs, otlp::OTLPConfigs};
use opentelemetry::{KeyValue, global, propagation::TextMapCompositePropagator};
use opentelemetry_otlp::{Compression, Protocol, SpanExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    propagation::{BaggagePropagator, TraceContextPropagator},
    resource::Resource,
    trace::{RandomIdGenerator, SdkTracerProvider, TracerProviderBuilder},
};
use tracing::{error, info};

/// Installs the OTLP gRPC exporter for OpenTelemetry tracing.
///
/// This function configures and installs a gRPC-based exporter that sends trace data
/// to an OpenTelemetry collector or compatible backend.
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
/// use traces::exporters::otlp_grpc;
/// use configs::Configs;
///
/// fn main() {
///     let cfg = Configs::new();
///     otlp_grpc::install().expect("Failed to install OTLP exporter");
/// }
/// ```
pub fn install() -> Result<SdkTracerProvider, TracesError> {
    let app_cfgs = AppConfigs::new();
    let otlp_cfgs = OTLPConfigs::new();

    let exporter = match SpanExporter::builder()
        .with_tonic()
        .with_protocol(Protocol::Grpc)
        .with_timeout(otlp_cfgs.exporter_timeout)
        .with_endpoint(&otlp_cfgs.endpoint)
        .with_compression(Compression::Gzip)
        .build()
    {
        Ok(p) => Ok(p),
        Err(err) => {
            error!(
                error = err.to_string(),
                "failure to create exporter provider"
            );
            Err(TracesError::ExporterProviderError)
        }
    }?;

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
                    format!("{}", app_cfgs.namespace),
                ))
                .with_attribute(KeyValue::new("environment", format!("{}", app_cfgs.env)))
                .with_attribute(KeyValue::new("library.language", "rust"))
                .build(),
        )
        .with_batch_exporter(exporter)
        .build();

    global::set_tracer_provider(provider.clone());
    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]));

    info!("traces::install otlp tracer installed");

    Ok(provider)
}
