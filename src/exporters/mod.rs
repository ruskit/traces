// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! OpenTelemetry exporter implementations.
//!
//! This module contains different trace exporters for sending telemetry data
//! to various backends. Each exporter is conditionally compiled based on
//! feature flags.
//!

#[cfg(any(feature = "stdout", feature = "otlp"))]
mod sampler;

#[cfg(feature = "otlp")]
pub mod otlp_grpc;

#[cfg(feature = "stdout")]
pub mod stdout;

pub mod noop;
