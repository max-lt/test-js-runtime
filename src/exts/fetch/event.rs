use actix_web::HttpRequest;
use std::string::String;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use v8::ContextScope;
use v8::Function;
use v8::FunctionCallbackArguments;
use v8::HandleScope;
use v8::Local;
use v8::Object;
use v8::ReturnValue;

use super::request::create_request;

pub struct JsFetchEvent<'a> {
    pub event: Local<'a, Object>,
    pub response_receiver: Receiver<String>,
}

struct FetchEventState {
    response_sender: Sender<String>,
}

/// Callback for event.respondWith
fn respond_with_callback(
  scope: &mut HandleScope,
  args: FunctionCallbackArguments,
  _ret: ReturnValue,
) {
  let body = args.get(0).to_rust_string_lossy(scope);
  println!("body: {}", body);
  let response_future = body;

  let state = scope.get_slot_mut::<FetchEventState>();

  match state {
      Some(state) => {
          println!("State found, setting response future");
          state.response_sender.send(response_future).unwrap()
      }
      None => {
          println!("No state found!!");
      }
  }
}

pub fn create_event<'a>(
  scope: &mut ContextScope<'_, HandleScope<'a>>,
  req: HttpRequest,
) -> JsFetchEvent<'a> {
  let event = Object::new(scope);

  let (response_sender, response_receiver) = mpsc::channel();

  let js_event = JsFetchEvent {
      event,
      response_receiver,
  };

  scope.set_slot(FetchEventState { response_sender });

  // Request
  let request = create_request(scope, req);
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
