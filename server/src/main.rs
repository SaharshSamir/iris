use std::net::SocketAddr;

use rspc::Router;
use axum::{routing::get, http::Request};
use tower_http::cors::Cors;
use tower_http::cors::{Any, CorsLayer};

struct Ctx{}

#[tokio::main]
async fn main() {
    let router = Router::<Ctx>::new()
        .query("version", |t| t(|_, _:()| env!("CARGO_PKG_VERSION")))
        .build()
        .arced();
    

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .route(
            "/rspc/:id",
            router
                .clone()
                .endpoint(|| {
                    println!("Hey there"); 
                    Ctx {}
                })
                .axum(),
        )
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));

    println!("Listening on https://{}/rspc/version", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


