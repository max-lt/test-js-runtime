use actix_web::HttpRequest;
use v8::ContextScope;
use v8::HandleScope;
use v8::Local;
use v8::Object;
use v8::Value;

use crate::utils;

pub(super) struct JsRequest<'a>(Local<'a, Object>);

impl<'a> From<JsRequest<'a>> for Local<'a, Object> {
    fn from(value: JsRequest<'a>) -> Self {
        value.0
    }
}

impl<'a> From<JsRequest<'a>> for Local<'a, Value> {
  fn from(value: JsRequest<'a>) -> Self {
    value.0.into()
  }
}

impl<'a> JsRequest<'a> {
    pub(super) fn from_http_request(
        scope: &mut ContextScope<'_, HandleScope<'a>>,
        req: HttpRequest,
    ) -> Self {
        let request = Object::new(scope);

        utils::assign_string(scope, request, "url", req.uri().to_string());
        utils::assign_string(scope, request, "method", req.method().to_string());

        // Headers
        {
            let headers = Object::new(scope);
            for (key, value) in req.headers() {
                utils::assign_string(
                    scope,
                    headers,
                    key.as_str(),
                    value.to_str().unwrap().to_string(),
                );
            }

            utils::assign(scope, request, "headers", headers.into());
        }

        // let data = actix_web::web::Payload::extract(&req);
        // let body = v8::ArrayBuffer::new(scope, 64);
        // let body = v8::Uint8Array::new(scope, body, 0, body.byte_length()).unwrap();

        JsRequest(request)
    }
}