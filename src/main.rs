// use dotenv::dotenv;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger("info");
    let app = app();
    let addr = "127.0.0.1:6969";
    log::info!("listening on http://{}", addr);
    serve(app, addr).await?;
    Ok(())
}

fn init_logger(level: &str) {
    structured_logger::Builder::with_level(level)
        .with_target_writer("*", structured_logger::json::new_writer(std::io::stdout()))
        .init();
}

fn app() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(routes::root::root_get))
}

async fn serve(app: axum::Router, addr: &str) -> anyhow::Result<()> {
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
