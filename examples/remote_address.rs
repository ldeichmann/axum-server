//! Run with `cargo run --example remote_address` command.
//!
//! To connect through browser, navigate to "http://localhost:3000" url.

use axum::{extract::ConnectInfo, routing::get, Router};
use std::net::SocketAddr;
use axum::extract::connect_info::Connected;
use axum_server::IncomingStream;

#[derive(Clone, Debug)]
struct MyConnectInfo {
    local_addr: SocketAddr,
    remote_addr: SocketAddr,
}
impl Connected<IncomingStream> for MyConnectInfo {
    fn connect_info(target: IncomingStream) -> Self {
        MyConnectInfo {
            local_addr: *target
                .local_addr()
                .as_ref()
                .expect("no local address in tcp stream, broken"),
            remote_addr: target.remote_addr(),
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .into_make_service_with_connect_info::<MyConnectInfo>();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum_server::bind(addr).serve(app).await.unwrap();
}

async fn handler(ConnectInfo(addr): ConnectInfo<MyConnectInfo>) -> String {
    format!(
        "your ip address is: {} - remote address is {}",
        addr.local_addr,
        addr.remote_addr,
    )
}
