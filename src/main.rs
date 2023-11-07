use std::net::SocketAddr;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Client, Error, Request, Response, Server,
};

async fn proxy(req: Request<Body>) -> Result<Response<Body>, Error> {
    dbg!(&req);

    let client = Client::new();

    Ok(client.request(req).await?)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Error>(service_fn(proxy)) });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}
