use v8::HandleScope;
use v8::Local;
use v8::Object;
use v8::Value;

use crate::utils;

pub struct JsRequest {
    pub url: String,
    pub method: String,
}

impl JsRequest {
    pub fn new(url: String, method: String) -> JsRequest {
        JsRequest { url, method }
    }

    pub fn to_value<'s>(&self, scope: &mut HandleScope<'s>) -> Local<'s, Value> {
        let request = Object::new(scope);
        let headers = v8::Map::new(scope);

        utils::assign_string(scope, request, "url", self.url.clone());
        utils::assign_string(scope, request, "method", self.method.clone());
        utils::assign(scope, request, "headers", headers.into());

        request.into()
    }
}
