use v8::Context;
use v8::ContextScope;
use v8::Global;
use v8::HandleScope;
use v8::Isolate;
use v8::Local;

use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;

use crate::utils;
use crate::utils::init::initialize_v8;
use crate::utils::inspect::inspect_v8_value;

use super::JsState;
use super::JsStateRef;
use super::RuntimeBasicMessage;
use super::RuntimeMessage;
use super::TaskHandler;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    CompileError,
    RuntimeError,
    ConversionError,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EvalError {}

pub struct JsRuntime {
    pub(crate) isolate: v8::OwnedIsolate,
    pub(crate) context: Global<Context>,
}

async fn async_op() -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    "Hello from async".to_string()
}

extern "C" fn promise_reject_callback(message: v8::PromiseRejectMessage) {
    let scope = &mut unsafe { v8::CallbackScope::new(&message) };

    print!("Promise rejected {:?}", message.get_event());

    match message.get_value() {
        None => print!(" value=None"),
        Some(value) => print!(" value=Some({})", value.to_rust_string_lossy(scope)),
    }

    println!(" {:?}", message.get_promise());
}

extern "C" fn message_callback(message: v8::Local<v8::Message>, value: v8::Local<v8::Value>) {
    let scope = &mut unsafe { v8::CallbackScope::new(message) };
    let scope = &mut v8::HandleScope::new(scope);
    let message_str = message.get(scope);

    println!(
        "Message callback {:?} {:?}",
        message_str.to_rust_string_lossy(scope),
        inspect_v8_value(value, scope)
    );
}

fn message_from_worker(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _ret: v8::ReturnValue,
) {
    if !args.length() == 1 {
        println!("postMessage expects 1 argument, got {}", args.length());
        utils::throw_type_error(scope, "postMessage expects 1 argument");
        return;
    }

    let message = args.get(0);
    let message = message.to_object(scope).unwrap();

    let kind = utils::get(scope, message, "kind").to_rust_string_lossy(scope);

    match kind.as_str() {
        "console" => {
            let mut output = String::new();

            let level = utils::get(scope, message, "level").to_rust_string_lossy(scope);

            let date = utils::get(scope, message, "date")
                .integer_value(scope)
                .unwrap_or(0);
            let date = chrono::NaiveDateTime::from_timestamp_millis(date).unwrap();

            let args = utils::get(scope, message, "args");

            let args: Local<'_, v8::Array> = args.try_into().unwrap();

            for i in 0..args.length() {
                let arg = args.get_index(scope, i).unwrap();
                write!(output, " {}", arg.to_rust_string_lossy(scope)).unwrap();
            }

            println!("[{:?}] console.{}:{}", date, level, output);
        }
        "timer" => {
            let delay = utils::get(scope, message, "delay");

            let time = delay.integer_value(scope).unwrap_or(0);

            let state = scope.get_slot::<JsStateRef>().unwrap();
            let mut state = state.borrow_mut();

            if delay.is_null_or_undefined() {
                println!("Clear timer");
                state.timer = None;
            } else {
                println!("Set timer: {:?}", time);

                let delay = std::time::Duration::from_millis(time as u64);

                state.timer = Some(std::time::Instant::now() + delay);
            }
        }
        "fetch" => {
            let request = utils::get(scope, message, "request");
            println!("request: {:?}", inspect_v8_value(request, scope));
            let request = request.to_object(scope).unwrap();
            let url = utils::get(scope, request, "url").to_rust_string_lossy(scope);

            let callback = utils::get(scope, message, "sendResponse");
            let callback = match v8::Local::<v8::Function>::try_from(callback) {
                Ok(callback) => callback,
                Err(_) => {
                    utils::throw_type_error(scope, "sendResponse is not a function");
                    return;
                }
            };

            // Convert callback into a global handle
            // let callback = v8::Global::new(scope, callback);
            // let state = scope.get_slot::<JsStateRef>().unwrap();

            let response = {
                let response = v8::Object::new(scope);

                let body = v8::String::new(scope, &format!("Response for {}", url)).unwrap();

                let options = v8::Object::new(scope);

                let status = v8::Integer::new(scope, 200);
                utils::assign(scope, options, "status", status.into());

                let status_text = v8::String::new(scope, "OK").unwrap();
                utils::assign(scope, options, "statusText", status_text.into());

                let headers = {
                    let headers = v8::Object::new(scope);

                    let content_type = v8::String::new(scope, "text/plain").unwrap();
                    utils::assign(scope, headers, "content-type", content_type.into());

                    headers
                };
                utils::assign(scope, options, "headers", headers.into());

                utils::assign(scope, response, "body", body.into());

                response
            };

            let undefined = v8::undefined(scope).into();

            callback.call(scope, undefined, &[response.into()]);
        }
        _ => {
            println!("Unknown message kind: {}", kind);
        }
    }
}

/// Register callback for onMessage
fn register_message_handler(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _ret: v8::ReturnValue,
) {
    println!("onMessage called {}", args.length());

    let callback = args.get(0);

    if !callback.is_function() {
        utils::throw_type_error(scope, "Arg 0 is not a function");
        return;
    }

    let callback: Local<v8::Function> = match callback.try_into() {
        Ok(callback) => callback,
        Err(_) => {
            utils::throw_type_error(scope, "Arg 0 is not a function");
            return;
        }
    };

    let callback = Global::new(scope, callback);

    let state = scope.get_slot::<JsStateRef>().unwrap();
    let mut state = state.borrow_mut();

    match state.handler.as_mut() {
        Some(_) => {
            println!("Handler already registered");

            drop(state);

            utils::throw_error(scope, "Handler already registered");
        }
        None => {
            println!("Registering handler");
            state.handler = Some(callback);
            return;
        }
    };
}

fn eval(scope: &mut HandleScope, code: &str) {
    let source = v8::String::new(scope, code).unwrap();
    let script = v8::Script::compile(scope, source, None).unwrap();
    script.run(scope);
}

fn eval_runtime(scope: &mut ContextScope<HandleScope>) {
    eval(scope, include_str!("../runtime/init.js"));
    eval(scope, include_str!("../runtime/atob.js"));
    eval(scope, include_str!("../runtime/btoa.js"));
    eval(scope, include_str!("../runtime/console.js"));
    eval(scope, include_str!("../runtime/navigator.js"));
    eval(scope, include_str!("../runtime/events.js"));
    eval(scope, include_str!("../runtime/timers.js"));
    eval(scope, include_str!("../runtime/fetch/headers.js"));
    eval(scope, include_str!("../runtime/fetch/response.js"));
    eval(scope, include_str!("../runtime/fetch/request.js"));
    eval(scope, include_str!("../runtime/fetch/fetch-event.js"));
    eval(scope, include_str!("../runtime/fetch/fetch.js"));
}

impl JsRuntime {
    pub fn create_snapshot() {
        initialize_v8();

        let mut isolate = Isolate::snapshot_creator(None);

        {
            let scope = &mut HandleScope::new(&mut isolate);

            let context = Context::new(scope);

            let scope = &mut ContextScope::new(scope, context);

            eval_runtime(scope);

            scope.set_default_context(context);
        }

        // Snapshot
        let snapshot = isolate.create_blob(v8::FunctionCodeHandling::Keep).unwrap();

        std::fs::write("snapshot.bin", snapshot).unwrap();
    }

    /// Create a new context with default extensions
    pub fn create_init(snapshot: Option<Vec<u8>>) -> Self {
        initialize_v8();

        let time = std::time::Instant::now();

        let from_snapshot = snapshot.is_some();

        let mut rt = {
            let mut isolate = match snapshot {
                Some(snapshot) => {
                    let params = v8::Isolate::create_params().snapshot_blob(snapshot);
                    Isolate::new(params)
                }
                None => Isolate::new(Default::default()),
            };

            isolate.set_capture_stack_trace_for_uncaught_exceptions(false, 0);
            isolate.set_promise_reject_callback(promise_reject_callback);
            isolate.add_message_listener(message_callback);

            let context = {
                let scope = &mut HandleScope::new(&mut isolate);

                let context = Context::new(scope);

                let scope = &mut ContextScope::new(scope, context);

                scope.set_slot(Rc::new(RefCell::new(JsState::default())));

                let context = Global::new(scope, context);

                context
            };

            JsRuntime { isolate, context }
        };

        if !from_snapshot {
            let scope = &mut HandleScope::new(&mut rt.isolate);
            let context = Local::new(scope, &rt.context);
            let scope = &mut ContextScope::new(scope, context);
            eval_runtime(scope);
        }

        let time = time.elapsed().as_micros();
        println!("Runtime init took {}μs (snapshot: {})", time, from_snapshot);

        // Set postMessage handler
        {
            let scope = &mut HandleScope::new(&mut rt.isolate);
            let context = Local::new(scope, &rt.context);
            let global = context.global(scope);
            let scope = &mut ContextScope::new(scope, context);

            let post_message = v8::FunctionTemplate::new(scope, message_from_worker);
            let post_message = post_message.get_function(scope).unwrap();

            let name = v8::String::new(scope, "postMessage").unwrap();
            global.set(scope, name.into(), post_message.into());
        }

        // Set onMessage handler
        {
            let scope = &mut HandleScope::new(&mut rt.isolate);
            let context = Local::new(scope, &rt.context);
            let global = context.global(scope);
            let scope = &mut ContextScope::new(scope, context);

            let on_message = v8::FunctionTemplate::new(scope, register_message_handler);
            let on_message = on_message.get_function(scope).unwrap();

            let name = v8::String::new(scope, "onMessage").unwrap();
            global.set(scope, name.into(), on_message.into());
        }

        // Runtime message handler
        rt.eval(include_str!("../runtime/message.js")).unwrap();

        rt
    }

    /// Evaluate a script
    pub fn eval(&mut self, script: &str) -> Result<String, EvalError> {
        let scope = &mut HandleScope::new(&mut self.isolate);

        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        let code = v8::String::new(scope, script).ok_or(EvalError::CompileError)?;
        let script = v8::Script::compile(scope, code, None).ok_or(EvalError::CompileError)?;

        // Run script
        let result = script.run(scope).ok_or(EvalError::RuntimeError)?;

        let result = result.to_string(scope).ok_or(EvalError::ConversionError)?;

        Ok(result.to_rust_string_lossy(scope))
    }

    pub fn send_message<E: super::message::RuntimeMessage>(
        &mut self,
        event: &mut E,
    ) -> Option<Local<v8::Value>> {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);

        event.prepare(scope);

        let result = {
            let scope = &mut ContextScope::new(scope, context);

            // Prepare handler call
            let handler = Self::get_handler(scope).unwrap();
            let undefined = v8::undefined(scope).into();

            let event = event.to_value(scope);

            // Call handler
            let result = handler.call(scope, undefined, &[event]);

            println!("Event result: {:?}", result);

            result
        };

        result
    }

    fn get_handler<'a>(scope: &mut HandleScope<'a>) -> Option<Local<'a, v8::Function>> {
        let handler = {
            let state = scope.get_slot::<JsStateRef>().expect("No state found");
            let state = state.borrow_mut();

            match state.handler.clone() {
                Some(handler) => handler,
                None => {
                    println!("No handler registered");
                    return None;
                }
            }
        };

        // Prepare handler call
        let handler = v8::Local::new(scope, handler);

        Some(handler)
    }

    fn poll_timer(
        cx: &mut std::task::Context,
        scope: &mut v8::ContextScope<v8::HandleScope>,
    ) -> std::task::Poll<bool> {
        let state = scope.get_slot::<JsStateRef>().expect("No state found");

        let timer = state.borrow_mut().timer;

        let now = std::time::Instant::now();

        match timer {
            None => {
                println!("Timer [void]");
                std::task::Poll::Ready(true)
            }
            Some(time) => {
                if time > now {
                    let waker = cx.waker().clone();

                    tokio::task::spawn(async move {
                        tokio::time::sleep_until(time.into()).await;
                        println!("Timer waked");
                        waker.wake();
                    });

                    println!("Timer [pending]");

                    std::task::Poll::Pending
                } else {
                    state.borrow_mut().timer = None;
                    println!("Timer fired");

                    // Wait for timer
                    {
                        let cb = v8::Function::new(
                            scope,
                            |scope: &mut v8::HandleScope,
                             _args: v8::FunctionCallbackArguments,
                             _rv: v8::ReturnValue| {
                                // Prepare handler call
                                let handler = Self::get_handler(scope).unwrap();
                                let undefined = v8::undefined(scope).into();

                                let message = RuntimeBasicMessage::new(String::from("timer"));
                                let message = message.to_value(scope);

                                // Call handler
                                handler.call(scope, undefined, &[message]);
                            },
                        )
                        .unwrap();

                        scope.enqueue_microtask(cb)
                    }

                    std::task::Poll::Ready(false)
                }
            }
        }
    }

    pub async fn run_event_loop<'a>(&mut self) {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        loop {
            scope.perform_microtask_checkpoint();

            let timers_task = tokio::macros::support::poll_fn(|cx| Self::poll_timer(cx, scope));

            let empty = timers_task.await;

            // Check if we are done
            if empty {
                break;
            }
        }
    }
}
