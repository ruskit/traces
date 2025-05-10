// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! OTLP gRPC exporter implementation.
//!
//! This module provides functionality to export trace data using the OpenTelemetry Protocol (OTLP)
//! over gRPC. This exporter is suitable for production environments where traces need to be
//! sent to an OpenTelemetry collector or compatible backend.

use crate::{errors::TracesError, get_sampler};
use configs::{Configs, DynamicConfigs};
use opentelemetry::{global, propagation::TextMapCompositePropagator, KeyValue};
use opentelemetry_otlp::{Compression, Protocol, SpanExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    propagation::{BaggagePropagator, TraceContextPropagator},
    resource::Resource,
    trace::{RandomIdGenerator, TracerProviderBuilder},
};
use std::time::Duration;
use tonic::metadata::{Ascii, MetadataKey, MetadataMap};
use tracing::{debug, error};

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
///     otlp_grpc::install(&cfg).expect("Failed to install OTLP exporter");
/// }
/// ```
pub fn install<T>(cfg: &Configs<T>) -> Result<(), TracesError>
where
    T: DynamicConfigs,
{
    let key: MetadataKey<Ascii> = match cfg.trace.header_access_key.clone().parse() {
        Ok(key) => key,
        Err(_) => {
            error!("failure to convert cfg.trace.header_key");
            MetadataKey::<Ascii>::from_bytes("api-key".as_bytes()).unwrap()
        }
    };

    let value = match cfg.trace.access_key.parse() {
        Ok(value) => Ok(value),
        Err(_) => {
            error!("failure to convert cfg.trace.header_value");
            Err(TracesError::ConversionError)
        }
    }?;

    let mut map = MetadataMap::with_capacity(2);
    map.insert(key, value);

    let exporter = match SpanExporter::builder()
        .with_tonic()
        .with_protocol(Protocol::Grpc)
        .with_timeout(Duration::from_secs(cfg.metric.export_timeout))
        .with_endpoint(cfg.metric.host.clone())
        .with_metadata(map)
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
        .with_batch_exporter(exporter)
        .build();

    global::set_tracer_provider(provider);
    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]));

    debug!("traces::install otlp tracer installed");

    Ok(())
}
