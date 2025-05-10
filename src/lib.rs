// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Traces
//!
//! A Rust library for distributed tracing using OpenTelemetry.
//!
//! This crate provides utilities to configure and use OpenTelemetry tracing in Rust applications,
//! with support for multiple exporters (OTLP gRPC and stdout) and propagation of trace context
//! across service boundaries, particularly focused on gRPC communication.
//!
//! ## Features
//!
//! - `otlp`: Enables the OpenTelemetry Protocol (OTLP) exporter over gRPC
//! - `stdout`: Enables console output for traces, useful for development
//!
//! ## Usage
//!
//! To initialize tracing with a configuration:
//!
//! ```rust
//! use traces::provider;
//! use configs::Configs;
//!
//! fn main() {
//!     // Initialize configuration
//!     let cfg = Configs::new();
//!     
//!     // Initialize tracing
//!     provider::init(&cfg).expect("Failed to initialize tracing");
//! }
//! ```

pub mod errors;
pub mod exporters;
pub mod extractors;
pub mod injectors;
pub mod provider;

#[cfg(any(feature = "otlp", feature = "stdout"))]
use configs::{Configs, DynamicConfigs, Environment};
use opentelemetry::trace::TraceContextExt;
use opentelemetry::{
    global::BoxedTracer,
    trace::{SpanKind, Tracer},
    Context,
};
#[cfg(any(feature = "otlp", feature = "stdout"))]
use opentelemetry_sdk::trace::Sampler;
use std::borrow::Cow;

/// Returns the appropriate sampler based on the environment configuration.
///
/// In local development environments, it always samples traces.
/// In other environments, it uses a trace ID ratio-based parent sampler.
#[cfg(any(feature = "otlp", feature = "stdout"))]
fn get_sampler<T>(cfg: &Configs<T>) -> Sampler
where
    T: DynamicConfigs,
{
    if cfg.app.env == Environment::Local {
        return Sampler::AlwaysOn;
    }

    let sampler = Sampler::TraceIdRatioBased(cfg.trace.export_rate_base);
    return Sampler::ParentBased(Box::new(sampler));
}

/// Creates a new span context with the specified kind and name.
///
/// # Arguments
///
/// * `tracer` - The OpenTelemetry tracer to use
/// * `kind` - The kind of span to create (Server, Client, etc.)
/// * `name` - The name of the span
///
/// # Returns
///
/// A new Context containing the created span
pub fn span_ctx(tracer: &BoxedTracer, kind: SpanKind, name: &str) -> Context {
    let span = tracer
        .span_builder(Cow::from(name.to_owned()))
        .with_kind(kind)
        .start(tracer);

    Context::current_with_span(span)
}

/// Extracts the trace ID from a Context.
///
/// # Arguments
///
/// * `ctx` - The Context containing the span
///
/// # Returns
///
/// A string representation of the trace ID, or an empty string if the span is not recording
pub fn trace_id(ctx: &Context) -> String {
    let span = ctx.span();

    if span.is_recording() {
        let span_ctx = span.span_context();

        return span_ctx.trace_id().to_string();
    }

    String::new()
}

/// Extracts the span ID from a Context.
///
/// # Arguments
///
/// * `ctx` - The Context containing the span
///
/// # Returns
///
/// A string representation of the span ID, or an empty string if the span is not recording
pub fn span_id(ctx: &Context) -> String {
    let span = ctx.span();

    if span.is_recording() {
        let span_ctx = span.span_context();

        return span_ctx.span_id().to_string();
    }

    String::new()
}
