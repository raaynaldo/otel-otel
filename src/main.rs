use opentelemetry::global;
use opentelemetry::metrics::Histogram;
use opentelemetry::KeyValue;
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::{runtime, Resource};
use rand::Rng;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

async fn tokenize_counter() -> Result<impl Reply, Rejection> {
    let meter = global::meter("tokenize_metric");

    let success_counter = meter.u64_counter("success_counter").init();
    let error_counter = meter.u64_counter("error_counter").init();

    let is_successful = rand::thread_rng().gen_bool(0.5);
    if is_successful {
        success_counter.add(1, &[KeyValue::new("type", "success_tokenize")]);
    } else {
        error_counter.add(1, &[KeyValue::new("type", "error_tokenize")]);
    }

    Ok(warp::reply::json(&"Ok"))
}

async fn tokenize_histogram() -> Result<impl Reply, Rejection> {
    let meter = global::meter("tokenize_histogram");

    let success_histogram = meter.f64_histogram("success_histogram").init();
    let error_histogram = meter.f64_histogram("error_histogram").init();

    let is_successful = rand::thread_rng().gen_bool(0.5);
    if is_successful {
        success_histogram.record(1.0, &[KeyValue::new("type", "success_tokenize")]);
    } else {
        error_histogram.record(1.0, &[KeyValue::new("type", "error_tokenize")]);
    }

    Ok(warp::reply::json(&"Ok"))
}

async fn tokenize_gauge() -> Result<impl Reply, Rejection> {
    let meter = global::meter("tokenize_gauge");

    let success_gauge = meter.f64_gauge("success_gauge").init();
    let error_gauge = meter.f64_gauge("error_gauge").init();

    let is_successful = rand::thread_rng().gen_bool(0.5);
    if is_successful {
        success_gauge.record(1.0, &[KeyValue::new("type", "success_tokenize")]);
    } else {
        error_gauge.record(1.0, &[KeyValue::new("type", "error_tokenize")]);
    }

    Ok(warp::reply::json(&"Ok"))
}

#[tokio::main]
async fn main() {
    let meter_provider = init_meter_provider();

    let tokenize_counter = warp::path("tokenize_counter")
        .and(warp::path::end())
        .and_then(tokenize_counter);

    let tokenize_histogram = warp::path("tokenize_histogram")
        .and(warp::path::end())
        .and_then(tokenize_histogram);

    let tokenize_gauge = warp::path("tokenize_gauge")
        .and(warp::path::end())
        .and_then(tokenize_gauge);

    let routes = warp::get()
        .and(tokenize_counter)
        .or(tokenize_histogram)
        .or(tokenize_gauge);

    // let count_metric_second = warp::path("count-metric-second")
    //     .and(warp::path::end())
    //     .and_then(count_metric_second);

    // let observable_gauge = warp::path("observable_gauge")
    //     .and(warp::path::end())
    //     .and_then(observable_gauge);

    // .or(count_metric_second)
    // .or(observable_gauge);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    meter_provider.shutdown().unwrap();
}

fn init_meter_provider() -> SdkMeterProvider {
    let exporter = opentelemetry_stdout::MetricsExporterBuilder::default()
        .with_encoder(|writer, data| Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
        .build();

    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        .with_interval(Duration::from_secs(5))
        .build();
    let provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::new([KeyValue::new(
            "service.name",
            "open-telemetry",
        )]))
        .build();

    global::set_meter_provider(provider.clone());
    provider
}

// async fn gauge() -> Result<impl Reply, Rejection> {
//     Ok(warp::reply::json(&"Ok"))
// }

// async fn observable_gauge() -> Result<impl Reply, Rejection> {
//     let meter = global::meter("observable_gauge");

//     let _observable_gauge = meter
//         .f64_observable_gauge("observable_gauge")
//         .with_callback(move |observer| {
//             println!("with callback");

//             let val = futures_executor::block_on(async_function());
//             println!("val: {}", val);
//             observer.observe(val, &[]);
//         })
//         .init();

//     Ok(warp::reply::json(&"Ok"))
// }

// async fn async_function() -> f64 {
//     println!("Task started... ");
//     sleep(Duration::from_secs(2)).await;
//     println!("Task completed after 3 seconds! ");

//     10.0
// }
