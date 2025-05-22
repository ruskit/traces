# Traces

[![Crate](https://img.shields.io/crates/v/traces.svg)](https://crates.io/crates/traces)
[![Documentation](https://docs.rs/traces/badge.svg)](https://docs.rs/traces)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A Rust library for distributed tracing using OpenTelemetry, providing simple configuration and comprehensive support for multiple trace exporters with a focus on gRPC service instrumentation.

## Overview

The `traces` crate is part of the Ruskit collection, providing a standardized way to implement distributed tracing in Rust applications. It supports different exporters for sending trace data to various backends, as well as utilities for propagating trace context across service boundaries.

## Features

- OpenTelemetry distributed tracing integration
- Multiple exporter options:
  - OTLP gRPC exporter for production environments (`otlp` feature)
  - Stdout exporter for development and debugging (`stdout` feature)
  - No-op exporter when no features are enabled (zero overhead)
- Environment-aware sampling strategies:
  - Always-on sampling for local development environments
  - Configurable ratio-based sampling for other environments
- First-class support for gRPC:
  - Utilities for extracting trace context from incoming gRPC requests
  - Utilities for injecting trace context into outgoing gRPC requests
- Helper functions for span context creation and management

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
traces = { git = "ssh://git@github.com/ruskit/traces.git", rev = "v0.0.1", features = ["otlp"] }
```

Available features:
- `otlp` - Enable the OpenTelemetry Protocol exporter over gRPC (recommended for production)
- `stdout` - Enable console output for traces (recommended for development)

You can enable both features if needed:
```toml
traces = { git = "ssh://git@github.com/ruskit/traces.git", rev = "v0.0.1", features = ["otlp", "stdout"] }
```

## Usage

### Initializing Tracing

```rust
use traces::provider;

fn main() {
    // Initialize tracing - this will automatically load configuration from environment
    let tracer_provider = provider::install().expect("Failed to initialize tracing");
    
    // Your application code...
}
```

### Creating Spans

```rust
use opentelemetry::{global, trace::SpanKind};
use traces::helpers;

fn perform_operation() {
    // Get the global tracer
    let tracer = global::tracer("my_service");
    
    // Create a new span context
    let ctx = helpers::ctx(&tracer, SpanKind::Internal, "operation_name");
    
    // Perform work within the span context
    // ...
    
    // Get trace and span IDs if needed
    let trace_id = helpers::trace_id(&ctx);
    let span_id = helpers::span_id(&ctx);
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

The traces library uses the `configs` crate for configuration. Configuration is automatically loaded from environment variables or configuration files. The relevant configuration properties are:

```rust
/// Application Configuration
struct AppConfigs {
    // Application name (used as service.name in traces)
    name: String,
    
    // Application namespace (used as service.namespace in traces)
    namespace: String,
    
    // Environment (development, staging, production)
    // Local environments use AlwaysOn sampling strategy
    env: Environment,
}

/// OpenTelemetry Configuration
struct OTLPConfigs {
    // OTLP exporter endpoint
    endpoint: String,            // default: http://localhost:4317
    
    // Timeout for exporting spans
    exporter_timeout: Duration,  // default: 30 seconds
    
    // Sample rate for non-local environments (0.0 to 1.0)
    exporter_rate_base: f64,     // default: 0.1 (10% of traces)
}
```

### Feature Flags

The exporter used is determined by feature flags in your `Cargo.toml`:

- When `stdout` feature is enabled, traces are exported to the console
- When `otlp` feature is enabled, traces are exported via OTLP gRPC
- When neither feature is enabled, a no-op tracer is installed

For example:
```toml
[dependencies]
traces = { git = "ssh://git@github.com/ruskit/traces.git", rev = "v0.0.1", features = ["otlp"] }
```

## Advanced Usage

### Error Handling

The library defines common error types for tracing operations:

```rust
enum TracesError {
    // An unexpected internal error occurred
    InternalError,
    
    // The requested exporter requires specific feature flags to be enabled
    InvalidFeaturesError,
    
    // Error occurred during type conversion
    ConversionError,
    
    // Failed to create the OpenTelemetry exporter provider
    ExporterProviderError,
}
```

### Resource Attributes

The tracer automatically sets several resource attributes for each trace:
- `service.name` - The application name from configuration
- `service.namespace` - The application namespace from configuration 
- `environment` - The deployment environment
- `library.language` - Set to "rust"

## License

This project is licensed under the [MIT License](LICENSE).