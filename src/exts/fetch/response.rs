use std::str::FromStr;

use actix_web::http::header::HeaderName;
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

use v8::Function;
use v8::FunctionCallbackArguments;
use v8::HandleScope;
use v8::Local;
use v8::Object;
use v8::ReturnValue;
use v8::Value;

use crate::utils;
use crate::utils::inspect::inspect_v8_value;

pub(super) struct JsResponse<'a>(Local<'a, Object>);

impl<'a> From<JsResponse<'a>> for Local<'a, Object> {
    fn from(value: JsResponse<'a>) -> Self {
        value.0
    }
}

impl<'a> From<JsResponse<'a>> for Local<'a, Value> {
    fn from(value: JsResponse<'a>) -> Self {
        value.0.into()
    }
}

impl<'a> JsResponse<'a> {
    pub(super) fn from_v8_value(scope: &mut HandleScope<'a>, value: Local<'a, Value>) -> Self {
        JsResponse(value.to_object(scope).unwrap())
    }

    pub(super) fn to_http_response(self, scope: &mut HandleScope<'a>) -> Option<HttpResponse> {
        println!("to_http_response");

        let response: Local<Value> = self.into();

        println!("to_http_response {}", inspect_v8_value(response, scope));

        let response: Local<Object> = response.to_object(scope).unwrap();
        let status_key = utils::v8_str_static!(scope, b"status");
        let status: u16 = match response.get(scope, status_key.into()) {
            Some(status) if status.is_number() => status.to_uint32(scope)?.value().try_into().ok()?,
            _ => return None
        };

        let headers_key = utils::v8_str_static!(scope, b"headers");
        let headers = match response.get(scope, headers_key.into()) {
            Some(headers) if headers.is_object() => headers.to_object(scope)?,
            _ => return None
        };

        let body_key = utils::v8_str_static!(scope, b"body");
        let body = response
            .get(scope, body_key.into())
            .unwrap()
            .to_rust_string_lossy(scope);

        let status = StatusCode::from_u16(status);
        let mut res = HttpResponse::build(status.unwrap()).body(body);

        // res.head_mut().reason = Some("&status_text");

        let headers_keys = headers
            .get_own_property_names(scope, v8::GetPropertyNamesArgs::default())
            .unwrap();

        for key in 0..headers_keys.length() {
            let val = headers.get_index(scope, key)?;
            let key = headers_keys.get_index(scope, key)?;

            let key = key.to_rust_string_lossy(scope);
            let key = HeaderName::from_str(&key).ok()?;
            let val = val.to_rust_string_lossy(scope);
            let val = HeaderValue::from_str(&val).ok()?;
            res.headers_mut().insert(key, val);
        }

        Some(res)
    }
}

/**
 * Response constructor
 */

fn response_constructor_callback<'a>(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut ret: ReturnValue<'a>,
) {
    let this = args.this();

    let argc = args.length();

    if argc > 2 {
        utils::throw_type_error(scope, "Response constructor takes at most 2 arguments");
        return;
    }

    // Process the body argument
    {
        let default_body = utils::v8_str_static!(scope, b"");
        let body_key = utils::v8_str_static!(scope, b"body");
        let body_value = match args.get(0) {
            body if body.is_string() => body.to_string(scope).unwrap_or(default_body),
            _ => default_body,
        };
        this.set(scope, body_key.into(), body_value.into());
    }

    // Process the options argument
    {
        let options_obj = match args.get(1) {
            options if options.is_null_or_undefined() => v8::Object::new(scope),
            options if options.is_object() => options.to_object(scope).unwrap(),
            _ => {
                utils::throw_type_error(scope, "invalid options");
                return;
            }
        };

        // Process the status option
        {
            let default_status = v8::Integer::new(scope, 200);
            let status_key = utils::v8_str_static!(scope, b"status");
            let status = match options_obj.get(scope, status_key.into()) {
                Some(status) if status.is_undefined() => default_status.into(),
                Some(status) if status.is_uint32() => status,
                None => default_status.into(),
                _ => utils::throw_type_error(scope, "invalid status"),
            };

            this.set(scope, status_key.into(), status);
        }

        // Process the statusText option
        {
            let default_status_text = v8::String::new(scope, "OK").unwrap();
            let status_text_key = utils::v8_str_static!(scope, b"statusText");
            let status_text: Local<v8::Value> = match options_obj.get(scope, status_text_key.into())
            {
                Some(status_text) if status_text.is_undefined() => default_status_text.into(),
                Some(status_text) if status_text.is_string() => status_text,
                None => default_status_text.into(),
                _ => utils::throw_type_error(scope, "statusText must be a string"),
            };

            this.set(scope, status_text_key.into(), status_text);
        }

        // Process the headers option
        {
            let default_headers = v8::Object::new(scope);
            let headers_key = utils::v8_str_static!(scope, b"headers");
            let headers = match options_obj.get(scope, headers_key.into()) {
                Some(headers) if headers.is_object() => {
                    headers.to_object(scope).unwrap_or(default_headers)
                }
                _ => default_headers,
            };

            this.set(scope, headers_key.into(), headers.into());
        }
    }

    println!("this is {}", inspect_v8_value(this.into(), scope));

    ret.set(this.into());
}

fn create_response_constructor<'a>(scope: &mut HandleScope<'a>) -> Local<'a, Function> {
    let constructor = Function::new(scope, response_constructor_callback);

    constructor.unwrap()
}

pub(super) fn bind_response_constructor(scope: &mut HandleScope) {
    let global = scope.get_current_context().global(scope);

    let key = utils::v8_str_static!(scope, b"Response");
    let response_constructor = create_response_constructor(scope);

    global.set(scope, key.into(), response_constructor.into());
}
