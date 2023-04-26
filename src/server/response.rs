use actix_web::HttpResponse;
use std::str::FromStr;

use actix_web::http::header::HeaderName;
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;

use crate::exts::fetch::response::JsResponse;

impl From<JsResponse> for HttpResponse {
    fn from(value: JsResponse) -> Self {
        println!("to_http_response {:?}", value);

        let status = StatusCode::from_u16(value.status).unwrap();

        let body = match value.body {
            Some(body) => body,
            None => String::new(),
        };

        let mut response = HttpResponse::build(status).body(body);
        let headers = response.headers_mut();

        for (key, val) in value.headers {
            let key = HeaderName::from_str(&key).unwrap();
            let val = HeaderValue::from_str(&val).unwrap();
            headers.insert(key, val);
        }

        response
    }
}
