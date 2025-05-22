// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Trace context extractors.
//!
//! This module provides extractors for retrieving trace context information
//! from various transport protocols and formats.

#[cfg(feature = "otlp")]
pub mod grpc;
