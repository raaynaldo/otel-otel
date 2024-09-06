use warp::{Filter, Rejection, Reply};

async fn count_metric() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&"Ok"))
}

#[tokio::main]
async fn main() {
    let count_metric = warp::get()
        .and(warp::path("count-metric"))
        .and(warp::path::end())
        .and_then(count_metric);

    let routes = count_metric;

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
