# Rust OpenTelemetry Examples
---
## Overview
 - Simple examples to get started with Rust and OpenTelemetry. 
 - Projects use the core opentelemetry libraries and Tokio Tracing API 
 - Each example is using the latest available dependencies. [17/06/2023] 

## Examples
- rust-otlp-basic
    - Tracing with the standard OpenTelemetry library
    - Export tracers using OTLP to Jaegar with Tonic
- rust-otlp-tracing
    - Tracing with Tracing API
    - Creates traces using Tracing API Macros
    - Export traces using OTLP to Jaegar with Tonic
- rust-otlp-actix
    - Tracing with the standard OpenTelemetry library + actix-web-opentelemetry
    - v0.18.0 of OpenTelemetry to support actix-web-opentelemetry
    - Export tracers using OTLP to Jaegar with Tonic
- rust-otlp-actix-tracing
    - Tracing with Tracing API + actix-web-opentelemetry
    - v0.18.0 of tracing to support actix-web-opentelemetry
    - Export tracers using OTLP to Jaegar with Tonic
- rust-otlp-newrelic
    - Tracing with the standard OpenTelemetry library
    - Export tracers using OTLP to New Relic with http and reqwest
- rust-otlp-newrelic-tracing
    - Tracing with Tracing API
    - Creates traces using Tracing API Macros
    - Export traces using OTLP to New Relic with http and reqwest
