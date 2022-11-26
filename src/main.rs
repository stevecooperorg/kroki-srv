mod askama_axum;
mod diags;
mod routes;
mod server;
mod util;

//use crate::askama_axum::into_response;
use crate::server::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = App::new();

    app.start(7999).await.expect("could not run server");
}
