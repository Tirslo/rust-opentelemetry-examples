# Rust OpenTelemetry Examples
---
## Overview
 - Simple examples to get started with Rust and OpenTelemetry. 
 - Projects use the core opentelemetry libraries and Tokio Tracing API 
 - Each example is using the latest available dependencies. [17/06/2023] 
 - Using [Jaeger](https://www.jaegertracing.io/docs/1.46/getting-started/) all in one for most examples.
 - For the New Relic examples [Sign Up](https://newrelic.com/signup) to a free developer account and [copy your license key](https://one.newrelic.com/api-keys)

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

## Install Jaegar All in One
```shell
docker run -d --name jaeger \
  -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 6831:6831/udp \
  -p 6832:6832/udp \
  -p 5778:5778 \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 14250:14250 \
  -p 14268:14268 \
  -p 14269:14269 \
  -p 9411:9411 \
  jaegertracing/all-in-one:1.46
```

| Port  | Protocol | Component | Function                                                                                     |   |
|-------|----------|-----------|----------------------------------------------------------------------------------------------|---|
| 6831  | UDP      | agent     | accept jaeger.thrift over Thrift-compact protocol (used by most SDKs)                        |   |
| 6832  | UDP      | agent     | accept jaeger.thrift over Thrift-binary protocol (used by Node.js SDK)                       |   |
| 5775  | UDP      | agent     | (deprecated) accept zipkin.thrift over compact Thrift protocol (used by legacy clients only) |   |
| 5778  | HTTP     | agent     | serve configs (sampling, etc.)                                                               |   |
|       |          |           |                                                                                              |   |
| 16686 | HTTP     | query     | serve frontend                                                                               |   |
|       |          |           |                                                                                              |   |
| 4317  | HTTP     | collector | accept OpenTelemetry Protocol (OTLP) over gRPC, if enabled                                   |   |
| 4318  | HTTP     | collector | accept OpenTelemetry Protocol (OTLP) over HTTP, if enabled                                   |   |
| 14268 | HTTP     | collector | accept jaeger.thrift directly from clients                                                   |   |
| 14250 | HTTP     | collector | accept model.proto                                                                           |   |
| 9411  | HTTP     | collector | Zipkin compatible endpoint (optional)                                                        |   |