// Here is an example of a simple Rust backend API that responds to HTTP POST requests to the endpoint /hello-world with a JSON object containing a message:

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

// This API uses the warp web framework to define a route for the /hello-world endpoint. The route only matches HTTP POST requests, and uses the hello_world_handler function to generate a JSON response.
// The hello_world_handler function creates a JSON object containing the status code and message, and uses the warp::reply::json function to convert it into a Reply type that can be sent back to the client.
// To use this API, you will need to have the Rust programming language and the warp web framework installed. You can then run the main function to start the API server, which will listen on the 127.0.0.1:8080 address by default. You can then use a tool like curl to send a POST request to the /hello-world endpoint and receive the JSON response.
//
// $ curl -X POST http://127.0.0.1:8080/hello-world
// {"status":200,"message":"Hello world, this is a bot response"}

// This is a very basic example of a Rust backend API, and you can extend and modify it to add more functionality as needed. For more information, you can refer to the warp documentation and the Rust documentation for working with JSON.

// // ##########################################################################

// use std::{
//     collections::HashMap,
//     env,
//     net::{IpAddr, Ipv4Addr, SocketAddr},
// };

// use actix_cors::Cors;
// use actix_web::{web, App, HttpServer, Result};
// use serde::Serialize;

// #[derive(Debug, Serialize)]
// struct Response {
//     status: u16,
//     message: String,
// }

// async fn hello_world(body: web::Json<HashMap<String, String>>) -> Result<web::Json<Response>> {
//     let message = body
//         .get("message")
//         .map(|s| s.to_string())
//         .unwrap_or_else(|| "Hello world, this is a bot response".to_string());

//     Ok(web::Json(Response {
//         status: 200,
//         message,
//     }))
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let ip: IpAddr = env::var("IP")
//         .unwrap_or_else(|_| "127.0.0.1".to_string())
//         .parse()
//         .unwrap();
//     let port: u16 = env::var("PORT")
//         .unwrap_or_else(|_| "8080".to_string())
//         .parse()
//         .unwrap();

//     HttpServer::new(|| {
//         App::new()
//             .wrap(
//                 Cors::new()
//                     .allowed_origin("http://localhost:3000")
//                     .allowed_methods(vec!["POST"])
//                     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
//                     .allowed_header(http::header::CONTENT_TYPE)
//                     .max_age(3600),
//             )
//             .service(web::resource("/hello-world").route(web::post().to(hello_world)))
//     })
//     .bind


// // ##########################################################################
// /*
//  * 
//  * BROKEN?
//  */

// use std::io::Read;
// use std::collections::HashMap;

// use hyper::{Body, Request, Response, Server};
// use hyper::rt::Future;
// use hyper::service::service_fn;
// use serde_json::json;

// type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

// fn hello_world(_req: Request<Body>) -> BoxFut {
//     let json = json!({
//         "status": 200,
//         "message": "Hello world, this is a bot response"
//     });

//     let response = Response::builder()
//         .header("Content-Type", "application/json")
//         .status(200)
//         .body(Body::from(json.to_string()))
//         .unwrap();

//     Box::new(future::ok(response))
// }

// fn router(req: Request<Body>) -> BoxFut {
//     let mut map = HashMap::new();
//     map.insert("/hello-world", hello_world);

//     let parts: Vec<&str> = req.uri().path().split('/').collect();
//     let endpoint = parts.get(1).unwrap_or(&"");
//     let handler = map.get(endpoint).unwrap_or(&not_found);

//     handler(req)
// }

// fn not_found(_req: Request<Body>) -> BoxFut {
//     let json = json!({
//         "status": 404,
//         "message": "Endpoint not found"
//     });

//     let response = Response::builder()
//         .header("Content-Type", "application/json")
//         .status(404)
//         .body(Body::from(json.to_string()))
//         .unwrap();

//     Box::new(future::ok(response))
// }

// fn main() {
//     let addr = ([127, 0, 0, 1], 8080).into();
//     let server = Server::bind(&addr)
//         .serve(|| service_fn(rou