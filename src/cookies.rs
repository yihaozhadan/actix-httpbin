use actix_web::{get, HttpRequest, HttpResponse, Responder, HttpResponseBuilder, http::StatusCode, cookie::Cookie, web};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct Cookies {
    cookies: BTreeMap<String, String>,
}

/**
 * Cookies
 * Creates, reads, and deletes Cookies
 */
#[get("/cookies")]
pub async fn get_cookies(req: HttpRequest) -> impl Responder {
    match req.cookies() {
        Ok(cookies) => {
            let mut cookie_hashmap = BTreeMap::new();
            for k in &*cookies {
                cookie_hashmap.insert(k.name().to_owned(), k.value().to_owned());
            }
            let req_cookies = Cookies {
                cookies: cookie_hashmap,
            };
            HttpResponse::Ok().json(req_cookies)
        },
        Err(error) => HttpResponse::BadRequest().body(error.to_string())
    }
}

#[get("cookies/delete")]
pub async fn delete_cookies(req: HttpRequest) -> impl Responder {
    let req_cookies = req.cookies().unwrap();
    let mut cookie_hashmap: BTreeMap<String, String> = BTreeMap::new();
    for req_cookie in &*req_cookies {
        cookie_hashmap.insert(req_cookie.name().to_owned(), req_cookie.value().to_owned());
    }

    let mut res = HttpResponseBuilder::new(StatusCode::OK);
    let query_vec = serde_urlencoded::from_str::<Vec<(String, String)>>(req.query_string()).unwrap(); 
    for query in query_vec {
        let query_k = query.0;
        if cookie_hashmap.contains_key(&query_k) {
            cookie_hashmap.remove(&query_k);
        }
        let mut c = Cookie::new(query_k.to_owned(), query.1);
        c.make_removal();
        res.cookie(c);
    }
    res.json(Cookies { cookies: cookie_hashmap })
}

#[get("/cookies/set")]
pub async fn set_cookies(req: HttpRequest) -> impl Responder {
    let query_vec = serde_urlencoded::from_str::<Vec<(String, String)>>(req.query_string()); 
    //TODO: Add Error Handling
    let req_cookies = req.cookies().unwrap();
    let mut cookie_hashmap: BTreeMap<String, String> = BTreeMap::new();
    match query_vec {
        Ok(queries) => {
            for k in &*req_cookies {
                cookie_hashmap.insert(k.name().to_owned(), k.value().to_owned());
            }
            let mut res = HttpResponseBuilder::new(StatusCode::OK);
            for q in queries {
                res.cookie(Cookie::build(&q.0, &q.1).finish());
                cookie_hashmap.insert(q.0, q.1);
            }
            let res_cookies = Cookies {
                cookies: cookie_hashmap,
            };
            res.json(res_cookies)
        },
        Err(error) => HttpResponse::BadRequest().body(error.to_string())
    }
}

#[get("/cookies/set/{name}/{value}")]
pub async fn set_cookie(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
    //TODO: Fix the cookie overwrite bug
    match req.cookies() {
        Ok(cookies) => {
            let mut res = HttpResponseBuilder::new(StatusCode::OK);
            let mut cookie_hashmap = BTreeMap::new();
            for k in &*cookies {
                cookie_hashmap.insert(k.name().to_owned(), k.value().to_owned());
                res.cookie(Cookie::new(k.name().to_owned(), k.value().to_owned()));
            }
            let (name, value) = path.into_inner();
            cookie_hashmap.insert(name.to_owned(), value.to_owned());
            let req_cookies = Cookies {
                cookies: cookie_hashmap,
            };
            res.cookie(Cookie::new(name, value));
            res.json(req_cookies)
        },
        Err(error) => HttpResponse::BadRequest().body(error.to_string())
    }
}
