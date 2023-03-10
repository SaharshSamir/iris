use crate::utils::Ctx;       
use axum::{Router, routing::get, response::IntoResponse, debug_handler, extract::State};

#[debug_handler]
async fn hey(_state: State<Ctx>) -> impl IntoResponse {
    return String::from("Hello from oauth");
}

pub fn oauth_router() -> Router<Ctx>{
    let router = Router::new()
        .route("/oauth/hey", get(hey));

    return router;
}
