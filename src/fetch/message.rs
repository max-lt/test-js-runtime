use v8::HandleScope;
use v8::Local;
use v8::Value;

use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot::Sender;

use super::JsRequest;
use super::JsResponse;
use crate::core::RuntimeMessage;
use crate::utils;

pub struct RuntimeFetchMessage {
    request: JsRequest,
    tx: Option<Sender<JsResponse>>, // Channel to send response (from worker to runtime)
    rx: Option<Receiver<JsResponse>>, // Channel to receive response
}

impl RuntimeFetchMessage {
    pub fn new(request: JsRequest) -> Self {
        let (sender, receiver) = oneshot::channel();

        RuntimeFetchMessage {
            request,
            tx: Some(sender),
            rx: Some(receiver),
        }
    }

    pub async fn get_response(&mut self) -> Option<JsResponse> {
        println!("Waiting for response...");
        let receiver = self.rx.take().unwrap();

        let response = receiver.await.unwrap();

        println!("Got response: {:?}", response);

        Some(response)
    }
}

/// Callback for event.respondWith
fn respond_with_callback<'a>(
    scope: &mut HandleScope<'a>,
    args: v8::FunctionCallbackArguments<'a>,
    _ret: v8::ReturnValue,
) {
    let js_response: Local<'_, Value> = args.get(0);

    println!(
        "Response callback: {}",
        utils::inspect_v8_value(js_response, scope)
    );

    let response = JsResponse::from_v8_value(scope, js_response);

    // Get sender from state
    let state = scope.get_slot_mut::<Option<Sender<JsResponse>>>();
    let sender = match state {
        Some(sender) => sender.take().unwrap(),
        None => {
            println!("No state found!!");
            return;
        }
    };

    // Send response
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

impl RuntimeMessage for RuntimeFetchMessage {
    fn kind(&self) -> String {
        "fetch".to_string()
    }

    fn prepare<'s>(&mut self, scope: &mut HandleScope<'s, ()>) {
        let sender = self.tx.take();

        scope.set_slot(sender);
    }

    fn to_value<'s>(&self, scope: &mut HandleScope<'s>) -> Local<'s, Value> {
        let event = v8::Object::new(scope);

        // FetchEvent.ype
        utils::assign_string(scope, event, "kind", self.kind());

        // FetchEvent.request
        let request = self.request.to_value(scope);
        utils::assign(scope, event, "request", request);

        // FetchEvent.respondWith
        let respond_with = v8::Function::new(scope, respond_with_callback).unwrap();
        utils::assign(scope, event, "sendResponse", respond_with.into());

        event.into()
    }
}
