use actix_web::{http::StatusCode, web, middleware, App, HttpResponse, HttpServer, Responder};

async fn status_codes(path: web::Path<u16>) -> impl Responder {
    let status = StatusCode::from_u16(path.into_inner());
    if status.is_err() {
        return HttpResponse::BadRequest().body(status.unwrap_err().to_string());
    }
    HttpResponse::Ok().body(status.unwrap().canonical_reason().unwrap_or("Not a valid status code"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(// prefixes all resources and routes attached to it...
                web::scope("/status")
                    .route("/{status_code}", web::get().to(status_codes))
                    .route("/{status_code}", web::post().to(status_codes))
                    .route("/{status_code}", web::put().to(status_codes))
                    .route("/{status_code}", web::patch().to(status_codes))
                    .route("/{status_code}", web::delete().to(status_codes))
                )
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
