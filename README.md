# Traces

[![Crate](https://img.shields.io/crates/v/traces.svg)](https://crates.io/crates/traces)
[![Documentation](https://docs.rs/traces/badge.svg)](https://docs.rs/traces)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A Rust library for distributed tracing using OpenTelemetry, providing easy configuration and support for multiple trace exporters.

## Overview

The `traces` crate is part of the Ruskit collection, providing a standardized way to implement distributed tracing in Rust applications. It supports different exporters for sending trace data to various backends, as well as utilities for propagating trace context across service boundaries.

## Features

- OpenTelemetry distributed tracing integration
- Multiple exporter options:
  - OTLP gRPC exporter for production environments (`otlp` feature)
  - Stdout exporter for development and debugging (`stdout` feature)
- Configurable trace sampling based on environment
- Utilities for extracting and injecting trace context in gRPC metadata
- Span creation and management helpers

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
traces = { git = "ssh://git@github.com/ruskit/traces.git", rev = "v0.0.1" }
# Enable optional exporters as needed
features = ["otlp", "stdout"]
```

## Usage

### Initializing Tracing

```rust
use traces::provider;
use configs::Configs;

fn main() {
    // Initialize configuration
    let cfg = Configs::new();
    
    // Initialize tracing
    provider::init(&cfg).expect("Failed to initialize tracing");
    
    // Your application code...
}
```

### Creating Spans

```rust
use opentelemetry::trace::SpanKind;
use traces::span_ctx;

fn perform_operation(tracer: &BoxedTracer) {
    // Create a new span
    let ctx = span_ctx(tracer, SpanKind::Internal, "operation_name");
    
    // Perform work within the span context
    // ...
    
    // Get trace and span IDs if needed
    let trace_id = traces::trace_id(&ctx);
    let span_id = traces::span_id(&ctx);
}
```

### gRPC Context Propagation

#### Server-side (extract context)

```rust
use traces::extractors::grpc;
use tonic::{Request, Response, Status};

async fn grpc_handler(request: Request<MyRequest>) -> Result<Response<MyResponse>, Status> {
    let metadata = request.metadata();
    let (ctx, span) = grpc::span(metadata, &global_tracer);
    
    // Process request with the extracted context
    // ...
    
    Ok(Response::new(MyResponse {}))
}
```

#### Client-side (inject context)

```rust
use traces::injectors::grpc;
use tonic::Request;

fn make_grpc_call(ctx: &Context) {
    let mut request = Request::new(MyRequest {});
    
    // Inject trace context into the request metadata
    grpc::inject(ctx, request.metadata_mut());
    
    // Make the gRPC call
    // ...
}
```

## Configuration

The traces library uses the `configs` crate for configuration. The relevant configuration properties are:

```rust
// Enable or disable tracing
trace.enable = true

// Select exporter type
trace.exporter = TraceExporterKind::OtlpGrpc  // or TraceExporterKind::Stdout

// For OTLP exporter
metric.host = "https://collector.example.com:4317"
metric.export_timeout = 30  // seconds
trace.header_access_key = "api-key"
trace.access_key = "your-api-key"

// Sampling configuration
trace.export_rate_base = 0.1  // Sample 10% of traces in non-local environments
```

## Features

- `otlp`: Enables the OpenTelemetry Protocol (OTLP) exporter over gRPC
- `stdout`: Enables console output for traces

## License

This project is licensed under the [MIT License](LICENSE).