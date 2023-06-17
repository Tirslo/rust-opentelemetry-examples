use opentelemetry::{
    global, runtime,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    trace::TraceError,
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use rand::Rng;
use tracing::{error, event, Level};
use tracing_subscriber::prelude::*;

fn init_tracer() -> Result<trace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "rust-otel-tracing",
            )])),
        )
        .install_batch(runtime::Tokio)
}

#[tracing::instrument]
fn gen_number() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..=100);
    let value = multiply_number(number);
    event!(
        Level::INFO,
        answer = value,
        "otel.status_message" = "multiplied",
        "otel.status_code" = 200
    );
}

#[tracing::instrument]
fn multiply_number(number: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let multiplier: u32 = rng.gen_range(0..=5);
    event!(
        Level::INFO,
        multiplier = multiplier,
        "otel.status_message" = "multiplying",
        "otel.status_code" = 200
    );
    fail();
    number * multiplier
}

#[tracing::instrument]
fn fail() {
    let fail_vec = vec![8];
    error!("Fail: {fail_vec:?}");
}

#[tokio::main]
async fn main() -> Result<(), TraceError> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = init_tracer().unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    gen_number();

    global::shutdown_tracer_provider();
    Ok(())
}
