#![allow(non_snake_case)]

use crate::utils::Ctx;
use axum::routing::get;
use bcrypt::{hash, verify};
use dotenv::dotenv;
use iris_core::prisma::{self, example, user, DeviceType, PrismaClient};
use jsonwebtoken::{encode, EncodingKey, Header};
use rspc::{Config, Error, ErrorCode, Type, integrations::httpz::Request};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

mod google_oauth;
mod models;
//mod oauth;
mod utils;


struct AnotherCtx {}

#[derive(Debug, Serialize, Deserialize)]
struct JwtPayload {
    user_id: String,
}

impl JwtPayload {
    fn new(user_id: String) -> Self {
        return Self { user_id };
    }
}
#[derive(Debug, Deserialize, Type)]
struct LoginData {
    email: String,
    password: String,
}

const JWT_SECRET: &str = "asdoreojfdifjdjfoijfacxvcnvmxcnbv045&&fjdkasldjfkljlaadsfuiordlureubvubhjg";

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!(env!("CARGO_MANIFEST_DIR"));
    let router = rspc::Router::<Ctx>::new()
        .config(Config::new().export_ts_bindings(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../iris_core/bindings.ts"),
        ))
        .query("health", |t| t(|_, _: ()| String::from("Aal is good...")))
        .mutation("register", |t| {
            t(|ctx: Ctx, data: LoginData| async move {
                //hash password
                let hashedPassword = hash(data.password, 12).unwrap();

                //try to find user of the same email, if found, return with "User already exists" error.
                let user: Option<user::Data> = ctx
                    .db
                    .user()
                    .find_first(vec![user::email::equals(data.email.clone())])
                    .exec()
                    .await?;
                if user.is_some() {
                    return Err(Error::new(
                        ErrorCode::BadRequest,
                        String::from("User already exists"),
                    ));
                }

                //add user to database
                let result: user::Data = ctx
                    .db
                    .user()
                    .create(data.email, hashedPassword, vec![])
                    .exec()
                    .await?;
                println!("result: {:?}", result);

                let claim = JwtPayload::new(result.id.clone());
                let token = encode(
                    &Header::default(),
                    &claim,
                    &EncodingKey::from_secret(JWT_SECRET.as_ref()),
                )
                .unwrap();

                return Ok(token);
            })
        })
        .mutation("login", |t| {
            t(|ctx: Ctx, data: LoginData| async move {
                let user: Option<user::Data> = ctx
                    .db
                    .user()
                    .find_first(vec![user::email::equals(data.email.clone())])
                    .exec()
                    .await
                    .unwrap();

                //return Ok("hey".to_string());
                match user {
                    Some(user) => {
                        //passwords do match
                        if verify(data.password, &user.password).unwrap() {
                            let claim = JwtPayload::new(user.id);
                            let token = encode(
                                &Header::default(),
                                &claim,
                                &EncodingKey::from_secret(JWT_SECRET.as_ref()),
                            );

                            return Ok(token.unwrap());
                        } else {
                            return Err(Error::new(
                                ErrorCode::Unauthorized,
                                String::from("Invalid credentials"),
                            )) as Result<String, _>;
                        }
                    }
                    None => {
                        return Err(Error::new(
                            ErrorCode::BadRequest,
                            String::from("The user does not exist"),
                        )) as Result<String, _>;
                    }
                };

            })
        })
        .query("getUser", |t| t(|ctx: Ctx, (): _| async move {
            let token = ctx.jwt;
            match token {
                Some(jwt) => {
                    println!("Token: {}", jwt);
                    return "got token, check console".to_string();
                },
                None => {
                    println!("Token not found");
                    return "Token not found".to_string(); 

                }
            }
        }))
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
                .endpoint(|req: Request|{
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

    println!("Listening on http://{}/rspc/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
