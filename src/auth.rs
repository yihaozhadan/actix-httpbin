use actix_web::{HttpRequest, Responder, HttpResponse, web, get};
use base64::{prelude::*, engine::general_purpose};
use serde::Serialize;

/**
 * Auth
 * Auth methods
 */
#[get("/basic-auth/{user}/{passwd}")]
pub async fn basic_auth(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
    #[derive(Serialize)]
    struct BasicAuthResponse {
        authenticated: bool,
        user: String,
    }
    let headers = req.headers();
    let (user, passwd) = path.into_inner();
    match headers.get("Authorization") {
        Some(auth_header) => {
            let basic_auth_str = String::from_utf8_lossy(auth_header.as_bytes()).into_owned();
            let target_auth_str = general_purpose::STANDARD.encode(format!("{user}:{passwd}"));
            if basic_auth_str == format!("Basic {target_auth_str}") {
                HttpResponse::Ok().json(BasicAuthResponse{ authenticated: true, user:user })
            } else {
                HttpResponse::Unauthorized().json(BasicAuthResponse{ authenticated: false, user:user })
            }
        },
        None => HttpResponse::Unauthorized().json(BasicAuthResponse{ authenticated: false, user:user })
    }
}

#[get("/bearer")]
pub async fn bearer(req: HttpRequest) -> impl Responder {
    #[derive(Serialize)]
    struct BearerAuthResponse {
        authenticated: bool,
        token: String,
    }
    let headers = req.headers();
    match headers.get("Authorization") {
        Some(auth_header) => {
            let bearer_auth_str = String::from_utf8_lossy(auth_header.as_bytes()).into_owned().trim().to_string();
            if bearer_auth_str.len() < 7 {
                return HttpResponse::Unauthorized().json(BearerAuthResponse{ authenticated: false, token: String::from("") })
            }
            let auth_str = &bearer_auth_str[7..];
            if bearer_auth_str.starts_with("Bearer ") {
                HttpResponse::Ok().json(BearerAuthResponse{ authenticated: true, token: auth_str.to_string() })
            } else {
                HttpResponse::Unauthorized().json(BearerAuthResponse{ authenticated: false, token: String::from("") })
            }
        },
        None => HttpResponse::Unauthorized().json(BearerAuthResponse{ authenticated: false, token: String::from("") })
    }
}
