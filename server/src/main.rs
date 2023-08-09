#![allow(non_snake_case)]
use crate::utils::Ctx;
use axum::routing::get;
use bcrypt::{hash, verify};
use chrono::prelude::*;
use dotenv::dotenv;
//use iris_core::prisma::*;
use iris_core::prisma::{self, device, example, user, DeviceType, PrismaClient};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rspc::{integrations::httpz::Request, Config, Error, ErrorCode, Type};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

mod google_oauth;
mod models;
mod routes;
//mod oauth;
mod utils;

struct AnotherCtx {}

#[derive(Debug, Serialize, Deserialize)]
struct JwtPayload {
    user_id: String,
    exp: u32,
}

impl JwtPayload {
    fn new(user_id: String) -> Self {
        return Self {
            user_id,
            exp: Utc::now().timestamp_subsec_millis(),
        };
    }
}
#[derive(Debug, Deserialize, Type)]
struct LoginData {
    email: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Type)]
struct DeviceInfo {
    name: String,
    user_id: String,
    device_type: String,
}

user::include!(user_with_devices { devices });

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!(env!("CARGO_MANIFEST_DIR"));
    let router = rspc::Router::<Ctx>::new()
        .config(Config::new().export_ts_bindings(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../iris_core/bindings.ts"),
        ))
        .query("health", |t| t(|_, _: ()| String::from("Aal is good...")))
        .merge("auth.", routes::auth::mount())
        //middleware to convert jwt to user_id and authenticate the following
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                match &mw.ctx.jwt.clone() {
                    Some(token) => {
                        let mut ctx = mw.ctx.clone();
                        return Ok(mw.with_ctx(ctx.authenticate(token.to_string()).clone()));
                        // return mw.ctx.authenticate(token);
                    }
                    None => {
                        println!("Token not found");
                        return Err(Error::new(
                            ErrorCode::Forbidden,
                            String::from("Who are you?"),
                        ));
                    }
                }
            })
        })
        .merge("user.", routes::user::mount())
        .build()
        .arced();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let mut context = utils::Ctx::new().await;
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest(
            "/rspc",
            router
                .clone()
                .endpoint(|req: Request| {
                    let jwt = req.headers().get("Authorization");
                    context.add_jwt(jwt);
                    return context;
                })
                //.endpoint(|| {
                //    ()
                //})
                .axum(),
        )
        //.merge(oauth::oauth_router())
        //.with_state(Ctx::new().await)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));

    println!(
        "Listening on http://{}/rspc/ \n tunneling frmo http://df39-103-132-28-208.ngrok-free.app",
        addr
    );
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
