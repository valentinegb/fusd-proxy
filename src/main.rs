use std::net::SocketAddr;

use env_logger::Env;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Client, Error, Request, Response, Server,
};
use log::{debug, error, info};

async fn proxy(req: Request<Body>) -> Result<Response<Body>, Error> {
    info!("received request: {} {}", req.method(), req.uri());
    debug!("{req:#?}");

    let client = Client::new();
    let res = client.request(req).await?;

    info!("received response: {}", res.status());
    debug!("{res:#?}");

    Ok(res)
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    const LOG_LEVEL: &str = "debug";
    #[cfg(not(debug_assertions))]
    const LOG_LEVEL: &str = "info";

    env_logger::init_from_env(Env::default().default_filter_or(LOG_LEVEL));
    info!("`env_logger` initialized");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("`addr` is {addr:#?}");

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Error>(service_fn(proxy)) });
    let server = Server::bind(&addr).serve(make_svc);

    info!("`server` binded and served");

    if let Err(e) = server.await {
        error!("server error: {e}");
    }
}
