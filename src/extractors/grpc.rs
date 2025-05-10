// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! gRPC trace context extractor.
//!
//! This module provides functionality to extract OpenTelemetry context
//! from gRPC metadata, allowing distributed tracing across gRPC service boundaries.

use opentelemetry::{
    global::{self, BoxedSpan, BoxedTracer},
    propagation::Extractor,
    trace::Tracer,
    Context,
};

/// An OpenTelemetry context extractor for gRPC requests.
///
/// This struct implements the `Extractor` trait to allow extracting trace context
/// from gRPC metadata headers.
pub struct GRPCExtractor<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> GRPCExtractor<'a> {
    /// Creates a new `GRPCExtractor` from a gRPC metadata map.
    ///
    /// # Arguments
    ///
    /// * `m` - Reference to a gRPC metadata map
    ///
    /// # Returns
    ///
    /// A new `GRPCExtractor` instance
    pub fn new(m: &'a tonic::metadata::MetadataMap) -> GRPCExtractor<'a> {
        GRPCExtractor(m)
    }
}

impl Extractor for GRPCExtractor<'_> {
    /// Get a value for a key from the MetadataMap.
    ///
    /// If the value can't be converted to &str, returns None.
    ///
    /// # Arguments
    ///
    /// * `key` - The metadata key to look up
    ///
    /// # Returns
    ///
    /// Option containing the value as a &str if found and convertible
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    ///
    /// # Returns
    ///
    /// A vector of all keys in the metadata map as string slices
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

/// Creates a span from gRPC metadata using the provided tracer.
///
/// This function extracts trace context from the gRPC metadata and creates a new span
/// within that context.
///
/// # Arguments
///
/// * `meta` - gRPC metadata containing potential trace context information
/// * `tracer` - OpenTelemetry tracer to use for creating the span
///
/// # Returns
///
/// A tuple containing the extracted context and a new span
pub fn span(meta: &tonic::metadata::MetadataMap, tracer: &BoxedTracer) -> (Context, BoxedSpan) {
    let ctx = global::get_text_map_propagator(|prop| prop.extract(&GRPCExtractor(meta)));
    let span = tracer.start_with_context("gRPC", &ctx);
    (ctx, span)
}
