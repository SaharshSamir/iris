use crate::utils::Ctx;       
use axum::{Router, routing::get, response::{IntoResponse, Redirect}, debug_handler, extract::State};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::cors::{Any, CorsLayer};

const COOKIE_NAME: &str = "CookieMonster";

#[debug_handler]
async fn hey(_state: State<Ctx>) -> impl IntoResponse {
    return String::from("Hello from oauth");
}

#[debug_handler]
async fn auth_callback() -> impl IntoResponse {
    
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
        .allow_origin(Any);

    let router = Router::new()
        .route("/oauth/", get(hey))
        .route("/auth/google", get(auth_callback))
        .route("/redirect", get(redirect_me))
        .route("/cookietest", get(test_for_cookies))
        .layer(CookieManagerLayer::new())
        .layer(cors);
        

    return router;
}
