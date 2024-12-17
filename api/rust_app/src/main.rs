use std::{env, net::SocketAddr, sync::Arc};

use axum_server::tls_rustls::RustlsConfig;

mod app_module;
mod config;
mod models;
mod routes;
mod services;
mod usecases;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = routes::create_routes(Arc::new(app_module::AppModule::new()));

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // run our app with hyper, listening globally on port 3000
    if let Err(_) = env::var("AWS_LAMBDA_FUNCTION_NAME") {
        // Run app on local server
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        // let cert = env::var("TLS_CERT")?;
        // let pem = env::var("TLS_PEM")?;

        // let config = RustlsConfig::from_pem_file(cert, pem).await?;
        // axum_server::bind_rustls(addr, config)
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await?;
    } else {
        lambda_http::run(app).await.unwrap();
    }

    Ok(())
}
