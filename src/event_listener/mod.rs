use v8::{Context, Function, FunctionCallbackArguments, Global, HandleScope, Local};

use crate::base::JsState;

fn add_event_listener(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut ret: v8::ReturnValue,
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
            let listener = v8::Global::new(scope, listener);

            let state = scope
                .get_slot_mut::<JsState>()
                .expect("Missing runtime data in V8 context");

            println!("listener: {:?}", listener);

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

pub fn bind_event_listener(scope: &mut HandleScope, context: Local<Context>) {
    let global = context.global(scope);

    {
        let key = v8::String::new_external_onebyte_static(scope, b"addEventListener").unwrap();
        let function_template = v8::FunctionTemplate::new(scope, add_event_listener);
        let function = function_template.get_function(scope).unwrap();
        global.set(scope, key.into(), function.into());
    }
}

#[cfg(test)]
mod tests {}
