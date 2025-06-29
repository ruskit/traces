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
//! ```no_run
//! use traces::provider;
//!
//! fn main() {
//!     // Initialize tracing
//!     let tracer_provider = provider::install().expect("Failed to initialize tracing");
//! }
//! ```

pub mod errors;
pub mod exporters;
pub mod extractors;
pub mod helpers;
pub mod injectors;
pub mod provider;
