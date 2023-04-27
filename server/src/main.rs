#![allow(non_snake_case)]

use rspc::{Config, integrations::httpz::Request,Type};
use axum::{routing::get, extract::State};
use chrono::FixedOffset;
use dotenv::dotenv;
use iris_core::{
    db::new_client::new_client,
    prisma::{example, PrismaClient, DeviceType, self, user},
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc, path::PathBuf};
use tower_http::cors::{Any, CorsLayer};
use crate::utils::Ctx;
use bcrypt::hash;

mod google_oauth;
mod oauth;
mod utils;
mod models;

struct AnotherCtx {}

#[derive(Debug, Deserialize, Type)]
struct LoginData {
    email: String,
    password: String,
    deviceType: String
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!(env!("CARGO_MANIFEST_DIR"));
    let router = rspc::Router::<Ctx>::new()
        .config(Config::new().export_ts_bindings(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../iris_core/bindings.ts")))
        .query("health", |t| t(|_, _: ()| String::from("Aal is good...")))
       // .query("newex", |t| {
       //     t(|ctx, _: ()| async move {
       //         let currentDT =
       //             chrono::Local::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
       //         let d = ctx
       //             .db
       //             .example()
       //             .create(vec![example::created_at::set(currentDT)])
       //             .exec() .await
       //             .expect("Could not create device");
       //         return currentDT;
       //     })
       // })
       .mutation("register", |t| {
           t(|ctx: Ctx, data: LoginData| async move {
               let hashedPassword = hash(data.password, 12).unwrap();
               let result: user::Data= ctx.db
                   .user()
                   .create(data.email, hashedPassword, vec![])
                   .exec()
                   .await?;
               println!("result: {:?}", result);
                return Ok(result); 
           })
       })
        .build()
        .arced();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let context = utils::Ctx::new().await;

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" })) 
        .nest(
            "/rspc",
            router
                .clone()
                .endpoint(|| {
                    return context;           
                })
                //.endpoint(|| {
                //    ()
                //})
                .axum(),
        )
        .merge(oauth::oauth_router())
        .with_state(Ctx::new().await)
        .layer(cors);


    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));

    println!("Listening on http://{}/rspc/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
