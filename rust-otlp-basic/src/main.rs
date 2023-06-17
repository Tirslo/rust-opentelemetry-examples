use opentelemetry::{
    global, runtime,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    trace::{TraceContextExt, TraceError, Tracer},
    Key, KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use rand::Rng;


const RANDOM: Key = Key::from_static_str("random.value");

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
                "rust-otel-basic",
            )])),
        )
        .install_batch(runtime::Tokio)
}

fn gen_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[tokio::main]
async fn main() -> Result<(), TraceError> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = init_tracer().unwrap();

    tracer.in_span("generating number", |cx| {
        let span = cx.span();
        let num = gen_number();
        span.add_event(
            "Generating Number".to_string(),
            vec![Key::new("number").i64(num.into())],
        );

        span.set_attribute(RANDOM.i64(10));

        tracer.in_span("generate another number", |cx| {
            let span = cx.span();
            let num = gen_number();
            span.add_event(
                "Generating Number".to_string(),
                vec![Key::new("number").i64(num.into())],
            )
        })
    });
    global::shutdown_tracer_provider();
    Ok(())
}
