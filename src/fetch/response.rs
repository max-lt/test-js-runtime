use std::collections::HashMap;

use v8::HandleScope;
use v8::Local;
use v8::Object;
use v8::Value;

use crate::utils;
use crate::utils::inspect::inspect_v8_value;

#[derive(Debug)]
pub struct JsResponse {
    pub status: u16,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

impl<'a> JsResponse {
    pub fn new(status: u16) -> Self {
        JsResponse {
            status,
            body: None,
            headers: HashMap::new(),
        }
    }

    pub(super) fn from_v8_value(
        scope: &mut HandleScope<'a>,
        response: Local<'a, Value>,
    ) -> Option<Self> {
        println!("from_v8_value {}", inspect_v8_value(response, scope));

        let mut res = JsResponse {
            status: 200,
            body: None,
            headers: HashMap::new(),
        };

        let response: Local<Object> = response.to_object(scope).unwrap();

        // Status
        {
            let status_key = utils::v8_str_static!(scope, b"status");
            res.status = match response.get(scope, status_key.into()) {
                Some(status) if status.is_number() => {
                    status.to_uint32(scope)?.value().try_into().ok()?
                }
                _ => return None,
            };
        }

        // Body
        {
            let body_key = utils::v8_str_static!(scope, b"body");
            let body = response
                .get(scope, body_key.into())
                .unwrap()
                .to_rust_string_lossy(scope);

            res.body = Some(body);
        }

        // Headers
        {
            let headers_key = utils::v8_str_static!(scope, b"headers");
            let headers = match response.get(scope, headers_key.into()) {
                Some(headers) if headers.is_object() => headers.to_object(scope)?,
                _ => return None,
            };

            let headers_keys = headers
                .get_own_property_names(scope, v8::GetPropertyNamesArgs::default())
                .unwrap();

            for key in 0..headers_keys.length() {
                let key = headers_keys.get_index(scope, key)?;
                let val = headers.get(scope, key.into())?;
                let key = key.to_rust_string_lossy(scope);
                let val = val.to_rust_string_lossy(scope);

                res.headers.insert(key, val);
            }
        }

        Some(res)
    }
}
