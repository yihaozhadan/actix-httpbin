mod utility;

use crate::utility::*;
use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/get", web::get().to(index))
            .route("/post", web::post().to(index))
            .route("/put", web::put().to(index))
            .route("/patch", web::patch().to(index))
            .route("/delete", web::delete().to(index))
            .service(status_codes)
            .service(anything)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
