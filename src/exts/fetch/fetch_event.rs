use actix_web::HttpRequest;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot::Sender;
use v8::ContextScope;
use v8::Function;
use v8::FunctionCallbackArguments;
use v8::HandleScope;
use v8::Local;
use v8::Object;
use v8::ReturnValue;

use crate::exts::fetch::response::JsResponse;
use crate::utils::inspect::inspect_v8_value;

use super::request::JsRequest;

pub struct JsFetchEvent<'a> {
    pub event: Local<'a, Object>,
    pub receiver: Receiver<JsResponse>,
}

struct FetchEventState {
    sender: Option<Sender<JsResponse>>,
}

/// Callback for event.respondWith
fn respond_with_callback<'a>(
    scope: &mut HandleScope<'a>,
    args: FunctionCallbackArguments<'a>,
    _ret: ReturnValue,
) {
    let body = args.get(0);
    println!("body: {}", inspect_v8_value(body, scope));
    let response = JsResponse::from_v8_value(scope, body);

    let state = scope.get_slot_mut::<FetchEventState>();
    let sender = match state {
        Some(state) => state.sender.take().unwrap(),
        None => {
            println!("No state found!!");
            return;
        }
    };

    // Create response
    match response {
        Some(response) => {
            println!("Sending response: {:?}", response);
            sender.send(response).unwrap();
        }
        None => {
            println!("Error creating response: {:?}", response);
            sender.send(JsResponse::new(500)).unwrap();
        }
    }
}

pub fn create_fetch_event<'a>(
    scope: &mut ContextScope<'_, HandleScope<'a>>,
    req: HttpRequest,
) -> JsFetchEvent<'a> {
    let event = Object::new(scope);

    let (sender, receiver) = oneshot::channel();

    let js_event = JsFetchEvent { event, receiver };

    scope.set_slot(FetchEventState { sender: Some(sender) });

    // Request
    let request = JsRequest::from_http_request(scope, req);
    let request_key = v8::String::new(scope, "request").unwrap();
    event
        .set(scope, request_key.into(), request.into())
        .unwrap();

    // RespondWith
    let respond_with = Function::new(scope, respond_with_callback).unwrap();
    let respond_with_key = v8::String::new(scope, "respondWith").unwrap();
    event
        .set(scope, respond_with_key.into(), respond_with.into())
        .unwrap();

    js_event
}
