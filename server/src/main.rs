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
    password: String,
}

#[derive(Debug, Deserialize, Type)]
struct DeviceInfo {
    name: String,
    user_id: String,
    device_type: String,
}

const JWT_SECRET: &str =
    "asdoreojfdifjdjfoijfacxvcnvmxcnbv045&&fjdkasldjfkljlaadsfuiordlureubvubhjg";

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
        .query("getUser", |t| {
            t(|ctx: Ctx, (): _| async move {
                let token = ctx.jwt;
                match token {
                    Some(jwt) => {
                        //parse token, get user id, fetch user from db, send user
                        let mut validation = Validation::new(Algorithm::HS256);
                        validation.validate_exp = false;
                        println!("Token: {}", jwt);
                        let jwt_data = decode::<JwtPayload>(
                            &jwt,
                            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
                            &validation,
                        )
                        .unwrap();

                        let user_id = jwt_data.claims.user_id;
                        // user::include!({ devices });
                        let user_data = ctx
                            .db
                            .user()
                            .find_unique(user::id::equals(user_id))
                            .include(user::include!({ devices }))
                            .exec()
                            .await
                            .unwrap();

                        match user_data {
                            Some(user) => {
                                return Ok(user);
                            }
                            None => {
                                return Err(Error::new(
                                    ErrorCode::Forbidden,
                                    String::from("You are not who you say you are. Sus."),
                                ))
                            }
                        }
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
        .mutation("addDevice", |t| {
            t(|ctx: Ctx, device_info: DeviceInfo| async move {
                println!("adding device");
                let device_type: DeviceType = match device_info.device_type.as_str() {
                    "Computer" => DeviceType::Desktop,
                    "Phone" => DeviceType::Phone,
                    _ => DeviceType::Desktop,
                };

                let _device = ctx
                    .db
                    .device()
                    .create(
                        device_type,
                        device_info.name,
                        user::id::equals(ctx.user_id.unwrap()),
                        vec![],
                    )
                    .exec()
                    .await?;

                return Ok("device added".to_string());
            })
        })
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

    println!("Listening on http://{}/rspc/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
