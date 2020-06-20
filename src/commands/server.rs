
use std::convert::Infallible;

use clap::ArgMatches;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::rt;
// use futures::executor::block_on;


async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}



// See https://github.com/brson/basic-http-server/blob/master/src/main.rs
fn serve() -> Result<(), String>{
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    
    match rt.run(server) {
        Ok(()) => {
            return Ok(())
        }
        Err(e) => {
            return Err(format!("server returned an error:{}", e))
        }
    }
}


pub fn run_server(_: &ArgMatches) -> Result<(), String> {
    env_logger::init();

    serve()
}
 