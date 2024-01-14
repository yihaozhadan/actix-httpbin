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
