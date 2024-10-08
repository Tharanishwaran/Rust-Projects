use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Respond with "Hello, World!" for any request
    Ok(Response::new(Body::from("Hello, World!")))
}

#[tokio::main]
async fn main() {
    // Define the address for the server to listen on (127.0.0.1:3000)
    let addr = ([127, 0, 0, 1], 3000).into();

    // Create a service handler
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    // Create the server
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
