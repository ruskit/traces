// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Tracing helper functions.
//!
//! This module provides utility functions for working with OpenTelemetry contexts,
//! spans, and trace/span identifiers. These helpers make it easier to create
//! and inspect trace contexts throughout the application.

use opentelemetry::{
    Context,
    global::BoxedTracer,
    trace::{SpanKind, TraceContextExt, Tracer},
};
use std::borrow::Cow;

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
///
/// # Examples
///
/// ```no_run
/// use opentelemetry::global;
/// use opentelemetry::trace::SpanKind;
/// use traces::helpers;
///
/// fn process_request() {
///     let tracer = global::tracer("my_service");
///     let ctx = helpers::ctx(&tracer, SpanKind::Server, "process_request");
///     // Use the context for the operation
/// }
/// ```
pub fn ctx(tracer: &BoxedTracer, kind: SpanKind, name: &str) -> Context {
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
///
/// # Examples
///
/// ```no_run
/// use opentelemetry::global;
/// use opentelemetry::trace::SpanKind;
/// use traces::helpers;
///
/// fn log_trace_id() {
///     let tracer = global::tracer("my_service");
///     let ctx = helpers::ctx(&tracer, SpanKind::Server, "my_operation");
///     let trace_id = helpers::trace_id(&ctx);
///     println!("Current trace ID: {}", trace_id);
/// }
/// ```
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
/// Extracts the span ID from a Context.
///
/// # Arguments
///
/// * `ctx` - The Context containing the span
///
/// # Returns
///
/// A string representation of the span ID, or an empty string if the span is not recording
///
/// # Examples
///
/// ```no_run
/// use opentelemetry::global;
/// use opentelemetry::trace::SpanKind;
/// use traces::helpers;
///
/// fn log_span_info() {
///     let tracer = global::tracer("my_service");
///     let ctx = helpers::ctx(&tracer, SpanKind::Server, "my_operation");
///     let span_id = helpers::span_id(&ctx);
///     println!("Current span ID: {}", span_id);
/// }
/// ```
pub fn span_id(ctx: &Context) -> String {
    let span = ctx.span();

    if span.is_recording() {
        let span_ctx = span.span_context();

        return span_ctx.span_id().to_string();
    }

    String::new()
}
