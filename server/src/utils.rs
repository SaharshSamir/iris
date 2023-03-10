use std::sync::Arc;
use iris_core::{prisma::PrismaClient, db::new_client::new_client};


#[derive(Debug, Clone)]
pub struct Ctx {
    pub db: Arc<PrismaClient>,
}

impl Ctx {
    pub async fn new() -> self::Ctx {
        let db = Arc::new(new_client().await.expect("Unable to create prisma client"));
        return Ctx{db};
    }
}
