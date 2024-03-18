use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;

fn init_logs() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let level = std::env::var("LOG").unwrap_or_else(|_| "debug".into());
    if let Err(err) = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(level))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
    {
        eprintln!("unable to register tracing: {err:?}");
    }
}

fn address() -> SocketAddr {
    let host = std::env::var("HOST")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3000);

    SocketAddr::from((host, port))
}

#[tokio::main]
async fn main() {
    init_logs();

    tracing::debug!("binding socket");
    let addr = address();
    let listener = TcpListener::bind(addr).await.unwrap();
    let app = axum::Router::default();

    tracing::info!("server listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
