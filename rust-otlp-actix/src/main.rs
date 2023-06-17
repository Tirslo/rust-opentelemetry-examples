use std::io;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_opentelemetry::RequestTracing;
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
                "rust-otlp-actix",
            )])),
        )
        .install_batch(runtime::Tokio)
}

async fn entry() -> HttpResponse {
    let tracer = global::tracer("request");
    tracer.in_span("generate number", |cx| {
        let span = cx.span();
        let num = gen_number();
        span.add_event(
            "Generating Number".to_string(),
            vec![Key::new("number").i64(num.into())],
        );
        span.set_attribute(RANDOM.i64(10));
    });
    HttpResponse::Ok().body("Hello")
}

fn gen_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let _tracer = init_tracer().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .route("/", web::get().to(entry))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
