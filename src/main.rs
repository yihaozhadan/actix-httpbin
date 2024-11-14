mod utility;
mod auth;
mod cookies;

use crate::utility::*;
use crate::auth::*;
use crate::cookies::*;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(basic_auth)
            .service(bearer)
            .service(status_codes)
            .service(get_request_headers)
            .service(get_ip)
            .service(get_user_agent)
            .service(cache)
            .service(set_cache)
            .service(set_etag)
            .service(set_response_headers)
            .service(get_cookies)
            .service(delete_cookies)
            .service(set_cookies)
            .service(set_cookie)
            .service(anything)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
