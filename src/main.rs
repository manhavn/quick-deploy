mod env;
mod handlers;
mod routes;

use crate::env::app::ENV;
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let app = routes::root::routes().merge(routes::api::routes());

    let ip_str = &ENV.rust_app_host;
    let ip: Ipv4Addr = ip_str.parse().unwrap();
    let octets: [u8; 4] = ip.octets();
    let addr = SocketAddr::from((octets, ENV.rust_app_port));

    println!("Server running on http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
