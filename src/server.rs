use anyhow::{Context, Result};
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

pub struct App {
    router: Router,
}

impl App {
    pub fn new() -> Self {
        let router = Router::new().route("/", get(crate::routes::home));
        Self { router }
    }

    pub async fn start(self, port: u16) -> Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(self.router.into_make_service())
            .await
            .context("running web server")
    }
}
