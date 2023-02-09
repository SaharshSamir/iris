#![allow(non_snake_case)]

use axum::routing::get;
use chrono::FixedOffset;
use dotenv::dotenv;
use iris_core::{
    db::new_client::new_client,
    prisma::{example, PrismaClient},
};
use rspc::Router;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

struct Ctx {
    db: Arc<PrismaClient>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = Arc::new(new_client().await.expect("Unable to create prisma client"));

    let router = Router::<Ctx>::new()
        .query("version", |t| t(|_, _: ()| env!("CARGO_PKG_VERSION")))
        .query("newex", |t| {
            t(|ctx, _: ()| async move {
                let currentDT =
                    chrono::Local::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
                let d = ctx
                    .db
                    .example()
                    .create(vec![example::created_at::set(currentDT)])
                    .exec()
                    .await
                    .expect("Could not create device");
                return currentDT;
            })
        })
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
                // .clone()
                .endpoint(|| {
                    return Ctx { db };
                })
                .axum(),
        )
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));

    println!("Listening on http://{}/rspc/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
