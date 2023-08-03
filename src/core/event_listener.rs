use v8::{Function, FunctionCallbackArguments, Global, HandleScope, Local};

use crate::core::JsRuntimeMod;
use crate::core::JsStateRef;
use crate::utils::{self, v8_str_static};

/// global.addEventListener function
fn add_event_listener(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    _ret: v8::ReturnValue,
) {
    if args.length() < 2 {
        let exception_str =
            v8::String::new(scope, "2 arguments required, but fewer were provided.").unwrap();
        let exception = v8::Exception::error(scope, exception_str);
        scope.throw_exception(exception);
        return;
    }

    // First argument must be a string
    let event = {
        let event = args.get(0);
        if !event.is_string() {
            let exception_str =
                v8::String::new(scope, "Expected a string as the first argument").unwrap();
            let exception = v8::Exception::error(scope, exception_str);
            scope.throw_exception(exception);
            return;
        }
        event.to_rust_string_lossy(scope)
    };

    let listener = args.get(1);
    // Check if the second argument is a function
    {
        if !listener.is_function() {
            let exception_str =
                v8::String::new(scope, "Expected a function as the second argument").unwrap();
            let exception = v8::Exception::error(scope, exception_str);
            scope.throw_exception(exception);
            return;
        }
    }

    println!("listener: {:?}", listener);

    let listener: Local<Function> = match listener.try_into() {
        Ok(listener) => listener,
        Err(_) => {
            let exception_str =
                v8_str_static!(scope, b"Expected a function as the second argument");
            let exception = v8::Exception::error(scope, exception_str);
            scope.throw_exception(exception);
            return;
        }
    };

    let listener = Global::new(scope, listener);

    // Get isolate state
    let state = scope.get_slot_mut::<JsStateRef>().expect("No state found");
    let state = state.clone();
    let mut state = state.borrow_mut();

    match state.handlers.get(&event) {
        Some(_) => {
            // Ensure that the listener is not already registered
            let exception_str = v8_str_static!(scope, b"Listener already registered");
            let exception = v8::Exception::error(scope, exception_str);
            scope.throw_exception(exception);
        }
        None => {
            // Add the listener to the handlers map
            state.handlers.insert(event, listener);
        }
    }
}

pub struct EventListerExt;

impl JsRuntimeMod for EventListerExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>) {
        let global = scope.get_current_context().global(scope);

        let key = utils::v8_str_static!(scope, b"addEventListener");
        let function_template = v8::FunctionTemplate::new(scope, add_event_listener);
        let function = function_template.get_function(scope).unwrap();
        global.set(scope, key.into(), function.into());
    }
}
