use actix_web::{http::{StatusCode, header::HeaderMap}, web, middleware, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct HttpInfo {
    headers: BTreeMap<String, String>,
    origin: String,
    url: String,
}

fn convert(headers: &HeaderMap) -> BTreeMap<String, String> {
    let mut header_hashmap = BTreeMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(String::new).push_str(&v)
    }
    header_hashmap
}

async fn index(req : HttpRequest) -> impl Responder {
    let conn = req.connection_info();
    let headers = req.headers();
    let host = conn.host();
    let path = req.path();
    let query = req.query_string();
    let addr = conn.peer_addr();
    let info = HttpInfo { 
        headers: convert(headers),
        origin: addr.unwrap().to_string(),
        url: if query.is_empty() { format!("http://{}{}", host, path) } else { format!("http://{}{}?{}", host, path, query) },
    };
    HttpResponse::Ok().json(info)
}

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
        .wrap(middleware::Logger::default())
            .route("/get", web::get().to(index))
            .route("/post", web::post().to(index))
            .route("/put", web::put().to(index))
            .route("/patch", web::patch().to(index))
            .route("/delete", web::delete().to(index))
            .service(
                web::scope("/status")
                    .route("/{status_code}", web::get().to(status_codes))
                    .route("/{status_code}", web::post().to(status_codes))
                    .route("/{status_code}", web::put().to(status_codes))
                    .route("/{status_code}", web::patch().to(status_codes))
                    .route("/{status_code}", web::delete().to(status_codes))
                )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
