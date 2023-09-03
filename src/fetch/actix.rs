use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;

use super::JsRequest;
use super::JsResponse;

const DEFAULT_CONTENT_TYPE: &str = "text/plain; charset=utf-8";

/// Convert an Actix request into a JS request
impl Into<JsRequest> for HttpRequest {
    fn into(self) -> JsRequest {
        JsRequest {
            url: self.uri().to_string(),
            method: self.method().to_string(),
            // headers: vec![],
            // body: None,
        }
    }
}

/// Convert a JsResponse into an Actix response
impl Into<HttpResponse> for JsResponse {
    fn into(self) -> HttpResponse {
        let ct = self
            .headers
            .get("Content-Type")
            .unwrap_or(&DEFAULT_CONTENT_TYPE.to_string())
            .to_string();

        let status = self.status;
        let status = StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = self.body.unwrap_or_default();

        HttpResponse::build(status)
            .content_type(ct)
            .body(body)
    }
}
