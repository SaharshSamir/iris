use crate::{utils::Ctx, JwtPayload, LoginData};
use bcrypt::{hash, verify};
use iris_core::prisma::{user, DeviceType};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rspc::{Error, ErrorCode, RouterBuilder, Type};
use serde::Deserialize;

const JWT_SECRET: &str =
    "asdoreojfdifjdjfoijfacxvcnvmxcnbv045&&fjdkasldjfkljlaadsfuiordlureubvubhjg";

#[derive(Debug, Deserialize, Type)]
struct DeviceInfo {
    name: String,
    user_id: String,
    device_type: String,
}

pub fn mount() -> RouterBuilder<Ctx> {
    return RouterBuilder::<Ctx>::new()
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
        });
}
