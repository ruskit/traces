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
