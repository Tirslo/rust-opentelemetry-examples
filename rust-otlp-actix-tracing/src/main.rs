use std::io;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use opentelemetry::{
    global::{self, shutdown_tracer_provider},
    runtime,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    trace::TraceError,
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use rand::Rng;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

fn init_tracer() -> Result<trace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://192.168.0.159:4317"),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "rust-otlp-actix-tracing",
            )])),
        )
        .install_batch(runtime::Tokio)
}

#[tracing::instrument]
async fn entry() -> HttpResponse {
    gen_number();
    HttpResponse::Ok().body("Hello")
}
#[tracing::instrument]
fn gen_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = init_tracer().unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .route("/", web::get().to(entry))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    shutdown_tracer_provider();
    Ok(())
}
