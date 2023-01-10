use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let hello_world = warp::path("hello-world")
        .and(warp::post())
        .and_then(hello_world_handler);

    warp::serve(hello_world).run(([127, 0, 0, 1], 8080)).await;
}

async fn hello_world_handler() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&json!({"status": 200, "message": "Hello world, this is a bot response"})))
}