use std::sync::Arc;

use axum::http::HeaderValue;
use iris_core::{prisma::PrismaClient, db::new_client::new_client};
use rspc::integrations::httpz::Request;


//borrowing Request
//#[derive(Debug, Clone)]
//pub struct Ctx<'a> {
//    pub db: Arc<PrismaClient>,
//    //pub req: Arc<Option<&'a Request>>
//    pub req: Option<Request>
//}


//impl<'a> Ctx<'a> {
//    //can not return Self for some reason
//    pub async fn new() -> Ctx<'a> {
//        let db = Arc::new(new_client().await.expect("Unable to create prisma client"));
//        return Ctx{db, req: Arc::new(None)};
//    }
//
//    pub fn addReq(&mut self, req: &'a Request) -> &Self {
//       self.req = Arc::new(Some(req)); 
//       return self;
//    }
//}



//Cloning Request
#[derive(Debug, Clone)]
pub struct Ctx {
    pub db: Arc<PrismaClient>,
    //pub req: Arc<Option<&'a Request>>
    pub jwt: Option<String>
}

impl Ctx {
    //can not return Self for some reason
    pub async fn new() -> Self {
        let db = Arc::new(new_client().await.expect("Unable to create prisma client"));
        return Ctx{db, jwt: None};
    }

    pub fn add_jwt(&mut self, jwt: Option<&HeaderValue>) -> &mut Self {
        if let Some(jwt_string) = jwt {
            self.jwt = Some(jwt_string.to_str().unwrap_or_default().to_string());
        }else {
            self.jwt = None;
        }
        return self;
    }
}
