use std::sync::Arc;

use axum::http::HeaderValue;
use iris_core::{db::new_client::new_client, prisma::PrismaClient};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rspc::integrations::httpz::Request;

use crate::JwtPayload;

const JWT_SECRET: &str =
    "asdoreojfdifjdjfoijfacxvcnvmxcnbv045&&fjdkasldjfkljlaadsfuiordlureubvubhjg";
//Cloning Request
#[derive(Debug, Clone)]
pub struct Ctx {
    pub db: Arc<PrismaClient>,
    //pub req: Arc<Option<&'a Request>>
    pub jwt: Option<String>,
    pub authenticated: bool,
    pub user_id: Option<String>,
}

impl Ctx {
    //can not return Self for some reason
    pub async fn new() -> Self {
        let db = Arc::new(new_client().await.expect("Unable to create prisma client"));
        return Ctx {
            db,
            jwt: None,
            authenticated: false,
            user_id: None,
        };
    }

    pub fn add_jwt(&mut self, jwt: Option<&HeaderValue>) -> &mut Self {
        if let Some(jwt_string) = jwt {
            self.jwt = Some(jwt_string.to_str().unwrap_or_default().to_string());
        } else {
            self.jwt = None;
        }
        return self;
    }

    pub fn authenticate(&mut self, token: String) -> &mut Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        println!("Token: {}", token);
        let jwt_data = decode::<JwtPayload>(
            &token,
            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
            &validation,
        )
        .unwrap();

        let user_id = jwt_data.claims.user_id;

        self.user_id = Some(user_id);
        self.authenticated = true;

        return self;
    }
}
