use actix_web::{http::{header::HeaderMap, StatusCode}, HttpRequest, Responder, HttpResponse, route, web, routes};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct HttpInfo {
    data: String,
    headers: BTreeMap<String, String>,
    json: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<String>,
    origin: String,
    url: String,
}

fn convert_headers(headers: &HeaderMap) -> BTreeMap<String, String> {
    let mut header_hashmap = BTreeMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(String::new).push_str(&v)
    }
    header_hashmap
}

#[routes]
#[delete("/delete")]
#[get("/get")]
#[patch("/patch")]
#[post("/post")]
#[put("/put")]
pub async fn index(req : HttpRequest, text: String) -> impl Responder {
    let conn = req.connection_info();
    let headers = req.headers();
    let host = conn.host();
    let path = req.path();
    let query = req.query_string();
    let addr = conn.peer_addr();
    let info = HttpInfo {
        data: text.clone(), 
        headers: convert_headers(headers),
        json: serde_json::from_str(text.as_str()).unwrap_or_default(),
        method: None,
        origin: addr.unwrap().to_string(),
        url: if query.is_empty() { format!("http://{}{}", host, path) } else { format!("http://{}{}?{}", host, path, query) },
    };
    HttpResponse::Ok().json(info)
}

#[route("/status/{status_code}", method="DELETE", method="GET", method="PATCH", method="POST", method="PUT")]
pub async fn status_codes(path: web::Path<u16>) -> impl Responder {
    let status = StatusCode::from_u16(path.into_inner());
    if status.is_err() {
        return HttpResponse::BadRequest().body(status.unwrap_err().to_string());
    }
    let status_value = status.unwrap();
    if status_value.as_u16() > 199 {
        HttpResponse::build(status_value).body(status_value.canonical_reason().unwrap_or("Unknown status code"))
    } else {
        HttpResponse::Ok().body(status_value.canonical_reason().unwrap_or("Unknown status code"))
    }
}

#[routes]
#[delete("/anything")]
#[get("/anything")]
#[patch("/anything")]
#[post("/anything")]
#[put("/anything")]
#[delete("/anything/{anything}")]
#[get("/anything/{anything}")]
#[patch("/anything/{anything}")]
#[post("/anything/{anything}")]
#[put("/anything/{anything}")]
pub async fn anything(req : HttpRequest, text: String) -> impl Responder {
    let conn = req.connection_info();
    let headers = req.headers();
    let method = req.method().as_str();
    let host = conn.host();
    let path = req.path();
    let query = req.query_string();
    let addr = conn.peer_addr();
    let info = HttpInfo { 
        data: text.clone(),
        headers: convert_headers(headers),
        json: serde_json::from_str(text.as_str()).unwrap_or_default(),
        method: Some(method.to_string()),
        origin: addr.unwrap().to_string(),
        url: if query.is_empty() { format!("http://{}{}", host, path) } else { format!("http://{}{}?{}", host, path, query) },
    };
    HttpResponse::Ok().json(info)
}
