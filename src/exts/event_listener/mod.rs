use v8::{Function, FunctionCallbackArguments, Global, HandleScope, Local};

use crate::base::JsExt;
use crate::base::JsState;

pub mod request;

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

    // First argument must be a string equal to "fetch"
    {
        let event_type = args.get(0).to_rust_string_lossy(scope);
        if event_type != "fetch" {
            let exception_str = v8::String::new(scope, "Unsupported event type").unwrap();
            let exception = v8::Exception::error(scope, exception_str);
            scope.throw_exception(exception);
            return;
        }
    }

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

    let listener: Result<Local<Function>, _> = listener.try_into();
    match listener {
        Ok(listener) => {
            let listener = Global::new(scope, listener);

            let state = scope
                .get_slot_mut::<JsState>()
                .expect("Missing runtime data in V8 context");

            // Ensure that the listener is not already registered
            if state.handler.is_some() {
                let exception_str = v8::String::new(scope, "Listener already registered").unwrap();
                let exception = v8::Exception::error(scope, exception_str);
                scope.throw_exception(exception);
                return;
            }

            state.handler = Some(listener);
        }
        Err(_) => {
            let exception_str =
                v8::String::new(scope, "Expected a function as the second argument").unwrap();
            let exception = v8::Exception::error(scope, exception_str);
            scope.throw_exception(exception);
        }
    }
}

fn bind_event_listener(scope: &mut HandleScope) {
    let global = scope.get_current_context().global(scope);

    {
        let key = v8::String::new_external_onebyte_static(scope, b"addEventListener").unwrap();
        let function_template = v8::FunctionTemplate::new(scope, add_event_listener);
        let function = function_template.get_function(scope).unwrap();
        global.set(scope, key.into(), function.into());
    }
}

pub struct EventListerExt;

impl JsExt for EventListerExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>) {
        bind_event_listener(scope);
    }
}

#[cfg(test)]
mod tests {
    use crate::exts::event_listener::EventListerExt;
    use crate::base::JsContext;

    fn prepare_context() -> JsContext {
        let mut ctx = JsContext::create();

        ctx.register(&EventListerExt);

        ctx
    }

    #[test]
    fn test_add_event_listener() {
        let mut ctx = prepare_context();

        let result = ctx.eval("typeof addEventListener").unwrap();

        assert_eq!(result, "function");
    }
}
