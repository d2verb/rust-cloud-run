#[tokio::main]
async fn main() {
    const DEFAULT_PORT: u16 = 8080;

    // Cloud Runでリッスンする際のポートに関しては以下を参照
    // https://cloud.google.com/run/docs/container-contract?hl=ja#port
    let port = match std::env::var("PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => DEFAULT_PORT,
        },
        Err(_) => DEFAULT_PORT,
    };

    let app = axum::Router::new().route("/", axum::routing::get(root));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
