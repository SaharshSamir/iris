use bcrypt::{hash, verify};
use iris_core::prisma::user;
use jsonwebtoken::{encode, EncodingKey, Header};
use rspc::{Error, ErrorCode, RouterBuilder};

use crate::{utils::Ctx, JwtPayload, LoginData};

const JWT_SECRET: &str =
    "asdoreojfdifjdjfoijfacxvcnvmxcnbv045&&fjdkasldjfkljlaadsfuiordlureubvubhjg";

pub fn mount() -> RouterBuilder<Ctx> {
    return RouterBuilder::<Ctx>::new()
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
                        String::from("email already exists"),
                    ));
                }

                //try to find user of the same email, if found, return with "User already exists" error.
                let user: Option<user::Data> = ctx
                    .db
                    .user()
                    .find_first(vec![user::username::equals(Some(data.username.clone()))])
                    .exec()
                    .await?;
                if user.is_some() {
                    return Err(Error::new(
                        ErrorCode::BadRequest,
                        String::from("username already exists"),
                    ));
                }

                //add user to database
                let result: user::Data = ctx
                    .db
                    .user()
                    .create(
                        data.email,
                        hashedPassword,
                        vec![user::username::set(Some(data.username))],
                    )
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
        });
}
