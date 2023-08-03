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
    let state = scope.get_slot_mut::<FetchEventState>();
    let sender = match state {
        Some(state) => state.sender.take().unwrap(),
        None => {
            println!("No state found!!");
            return;
        }
    };

    let mut body = args.get(0);
    println!("body: {}", inspect_v8_value(body, scope));

    // Check if the value is a Promise
    if body.is_promise() {
        println!("respondWith callback is a promise! {:?}", body);

        let promise = v8::Local::<v8::Promise>::try_from(body).unwrap();

        println!("respondWith callback is clearly a promise! {:?}", promise);

        let mut wait = 10;
        while promise.state() == v8::PromiseState::Pending {
            println!("Promise is {:?} {:?}", promise.state(), wait);
            println!("pending_tasks? {:?}", scope.has_pending_background_tasks());
            scope.perform_microtask_checkpoint();
            std::thread::sleep(std::time::Duration::from_millis(10));
            wait -= 1;

            // If the promise is not pending anymore, break the loop
            if promise.state() != v8::PromiseState::Pending {
                println!("Promise is {:?} {:?}", promise.state(), wait);
                break;
            }

            // If the promise is still pending after 10 iterations, timeout
            if wait == 0 {
                println!("Promise is still pending, timeout!");
                return sender.send(JsResponse::new(504)).unwrap();
            }
        }

        match promise.state() {
            v8::PromiseState::Fulfilled => {
              println!("Promise is fulfilled!");
              body = promise.result(scope);
            },
            v8::PromiseState::Rejected => {
              println!("Promise is rejected!");
              return sender.send(JsResponse::new(500)).unwrap();
            },
            v8::PromiseState::Pending => panic!("Promise is pending!"), // Should not happen (see loop above)
        }
    }

    let response = JsResponse::from_v8_value(scope, body);

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

    scope.set_slot(FetchEventState {
        sender: Some(sender),
    });

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
