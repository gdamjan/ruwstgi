use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let cmd = tokio::process::Command::new("./data/cgi-bin/index.cgi");
    let (response, stderr) = hyper_cgi::do_cgi(req, cmd).await;
    if stderr.len() > 0 {
        let s = String::from_utf8_lossy(&stderr);
        eprintln!("cgi-error: {}", s);
    }
    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);
    eprintln!("php: {}", String::from_utf8_lossy(phper_sys::PHP_VERSION));



    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
