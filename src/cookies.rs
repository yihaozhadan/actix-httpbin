use actix_web::{get, HttpRequest, HttpResponse, Responder, HttpResponseBuilder, http::StatusCode, cookie::Cookie};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct Cookies {
    cookies: BTreeMap<String, String>,
}

/**
 * Cookies
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

#[get("/cookies/set")]
pub async fn set_cookies(req: HttpRequest) -> impl Responder {
    let querie_vec = serde_urlencoded::from_str::<Vec<(String, String)>>(req.query_string()); 
    //TODO: Add Error Handling
    let req_cookies = req.cookies().unwrap();
    let mut cookie_hashmap: BTreeMap<String, String> = BTreeMap::new();
    match querie_vec {
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
