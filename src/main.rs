mod utility;

use crate::utility::*;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(status_codes)
            .service(anything)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
