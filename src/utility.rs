use actix_web::{http::{header::HeaderMap, StatusCode}, HttpRequest, Responder, HttpResponse, route, web, routes, get};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct HttpInfo {
    
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

/**
 * HTTP Methods
 */
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

/**
 * Status codes
 */
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

/**
 * Request Inspection
 */
#[get("/headers")]
pub async fn get_request_headers(req : HttpRequest) -> impl Responder {
    #[derive(Serialize)]
    struct RequestHeaders {
        headers: BTreeMap<String, String>,
    }
    let headers = req.headers();
    let req_headers = RequestHeaders {
        headers: convert_headers(headers),
    };
    HttpResponse::Ok().json(req_headers)
}

#[get("/ip")]
pub async fn get_ip(req : HttpRequest) -> impl Responder {
    #[derive(Serialize)]
    struct Ip {
        origin: String,
    }
    let conn = req.connection_info();
    let addr = conn.peer_addr();
    let ip = Ip {
        origin: addr.unwrap().to_string(),
    };
    HttpResponse::Ok().json(ip)
}

#[get("/user-agent")]
pub async fn get_user_agent(req : HttpRequest) -> impl Responder {
    #[derive(Serialize)]
    struct UserAgent {
        #[serde(rename="user-agent")]
        user_agent: String,
    }
    let headers = req.headers();
    let ip = UserAgent {
        user_agent: headers.get("user-agent").unwrap().to_str().unwrap_or_default().to_string(),
    };
    HttpResponse::Ok().json(ip)
}


/**
 * Response Inspection
 */

#[get("/cache")]
pub async fn cache(_req: HttpRequest, text: String) -> impl Responder {
    let headers = _req.headers();
    let if_modified_since = headers.get("If-Modified-Since");
    let if_none_match = headers.get("If-None-Match");
    if if_modified_since.is_some() || if_none_match.is_some() {
        HttpResponse::NotModified().finish()
    } else {
        let conn = _req.connection_info();
        let headers = _req.headers();
        let host = conn.host();
        let path = _req.path();
        let query = _req.query_string();
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
}

#[get("/cache/{value}")]
pub async fn set_cache(_req: HttpRequest, value: web::Path<u64>) -> impl Responder {
    let seconds = value.into_inner();
    let cache_control = format!("max-age={}", seconds);
    HttpResponse::Ok()
        .append_header(("Cache-Control", cache_control))
        .finish()
}

#[get("/etag/{etag}")]
pub async fn set_etag(_req: HttpRequest, etag: web::Path<String>) -> impl Responder {
    let etag = etag.into_inner();
    HttpResponse::Ok()
        .append_header(("Etag", etag))
        .finish()
}

#[routes]
#[get("/response-headers")]
#[post("/response-headers")]
pub async fn set_response_headers(_req: HttpRequest) -> impl Responder {
    let mut response = HttpResponse::Ok();
    let query_str = _req.query_string();
    for pair in query_str.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            response.append_header((key, value));
        }
    }
    response.finish()
}

/**
 * Anything
 */
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
