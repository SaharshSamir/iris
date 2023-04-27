use std::collections::HashMap;

use crate::{utils::Ctx, 
    models::GoogleAuthCallbackQueryParams,
    google_oauth::get_access_token
};       
use axum::{Router, routing::get, response::{IntoResponse, Redirect}, debug_handler, extract::{State, Query}, Json, http::StatusCode};
use serde::Serialize;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::cors::{Any, CorsLayer};

const COOKIE_NAME: &str = "CookieMonster";

#[derive(Debug, Serialize)]
struct ApiResponse {
    msg: String,
    body: Option<HashMap<String, String>>
}

#[debug_handler]
async fn hey(_state: State<Ctx>) -> impl IntoResponse {
    return String::from("Hello from oauth");
}

#[debug_handler]
async fn auth_callback(Query(query): Query<GoogleAuthCallbackQueryParams>) -> Result<String, StatusCode>{
    let code = &query.code;

    if code.is_empty() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let access_token_res = get_access_token(code).await;

    if access_token_res.is_err() {
        let message = access_token_res.err().unwrap().to_string();
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    } 

    let access_token = access_token_res.unwrap();
    println!("access token: {:?}", access_token);
    //return Ok(Redirect::permanent("http://localhost:6969/testPage"));
    //
    //return Json(ApiResponse { msg: "in progress".to_string(), body: None });
    return Ok("Authed".to_string());
}

#[debug_handler]
async fn redirect_me(cookies: Cookies) -> impl IntoResponse {
    println!("In the cookie part");
    cookies.add(Cookie::new(COOKIE_NAME, "Boo"));
    return "check cookies".to_string();
}

#[debug_handler]
async fn test_for_cookies(cookies: Cookies) -> impl IntoResponse {
    //let cookie = cookies.get(COOKIE_NAME).unwrap_or(Cookie::new("NewCookie", "nothing in here"));
    let cookie = cookies.get(COOKIE_NAME).unwrap();
    println!("Cookie Recvd: {}", cookie.value());
    return format!("Cookie is: {}", cookie.value());
}

pub fn oauth_router() -> Router<Ctx>{
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);


    let router = Router::new()
        .route("/oauth/", get(hey))
        .route("/auth/google", get(auth_callback))
        .route("/redirect", get(redirect_me))
        .route("/cookietest", get(test_for_cookies))
        .layer(CookieManagerLayer::new())
        .layer(cors);
        

    return router;
}
