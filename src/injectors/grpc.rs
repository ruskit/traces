// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! gRPC trace context injector.
//!
//! This module provides functionality to inject OpenTelemetry context
//! into gRPC metadata, allowing distributed tracing across gRPC service boundaries.

use opentelemetry::{
    global::{self},
    propagation::Injector,
    Context,
};

/// An OpenTelemetry context injector for gRPC requests.
///
/// This struct implements the `Injector` trait to allow injecting trace context
/// into gRPC metadata headers.
pub struct GRPCInjector<'a>(&'a mut tonic::metadata::MetadataMap);

impl<'a> GRPCInjector<'a> {
    /// Creates a new `GRPCInjector` from a mutable gRPC metadata map.
    ///
    /// # Arguments
    ///
    /// * `m` - Mutable reference to a gRPC metadata map
    ///
    /// # Returns
    ///
    /// A new `GRPCInjector` instance
    pub fn new(m: &'a mut tonic::metadata::MetadataMap) -> GRPCInjector<'a> {
        GRPCInjector(m)
    }
}

impl Injector for GRPCInjector<'_> {
    /// Sets a key and value in the gRPC MetadataMap.
    ///
    /// Does nothing if the key or value cannot be converted into valid metadata.
    ///
    /// # Arguments
    ///
    /// * `key` - The metadata key to set
    /// * `value` - The value to set for the given key
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(val) = tonic::metadata::MetadataValue::try_from(&value) {
                self.0.insert(key, val);
            }
        }
    }
}

/// Injects trace context into gRPC metadata.
///
/// This function injects the current trace context into gRPC metadata
/// so that it can be propagated to the next service in the call chain.
///
/// # Arguments
///
/// * `ctx` - The OpenTelemetry context to propagate
/// * `meta` - Mutable reference to gRPC metadata where the context will be injected
///
/// # Examples
///
/// ```rust,no_run
/// use opentelemetry::Context;
/// use traces::injectors::grpc;
/// use tonic::metadata::MetadataMap;
///
/// fn make_grpc_call(ctx: &Context) {
///     let mut metadata = MetadataMap::new();
///     // Inject trace context into the metadata
///     grpc::inject(ctx, &mut metadata);
///     // Now use the metadata for your gRPC call
/// }
/// ```
pub fn inject(ctx: &Context, meta: &mut tonic::metadata::MetadataMap) {
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(ctx, &mut GRPCInjector(meta))
    });
}
