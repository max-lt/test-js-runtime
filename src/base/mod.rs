use v8::Context;
use v8::ContextScope;
use v8::HandleScope;
use v8::Isolate;
use v8::OwnedIsolate;
use v8::{Global, Local};

use crate::inspect::inspect_v8_value;
use crate::utils::initialize_v8;

pub struct JsRuntime {
    isolate: OwnedIsolate,
}

pub struct JsContext<'p> {
    runtime: &'p mut JsRuntime,
    context: Global<Context>,
}

pub struct JsState {
    pub handler: Option<Global<v8::Function>>,
}

impl JsRuntime {
    pub fn new() -> JsRuntime {
        initialize_v8();

        JsRuntime {
            isolate: Isolate::new(Default::default()),
        }
    }

    pub fn create_context<'p>(&'p mut self) -> JsContext<'p> {
        JsContext::new(self)
    }
}

impl<'p> JsContext<'p> {
    fn new(runtime: &'p mut JsRuntime) -> Self {
        let context = {
            // let mut isolate = &runtime.isolate;
            let scope = &mut HandleScope::new(&mut runtime.isolate);

            let context = Context::new(scope);

            let scope = &mut ContextScope::new(scope, context);

            scope.set_slot(JsState { handler: None });

            crate::console::bind_console(scope, context);
            crate::base64::bind_base64(scope, context);
            crate::event_listener::bind_event_listener(scope, context);

            let context = Global::new(scope, context);

            context
        };

        JsContext { runtime, context }
    }

    pub fn run_script(&mut self, script: &str) -> Option<String> {
        let scope = &mut HandleScope::new(&mut self.runtime.isolate);

        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        let code = v8::String::new(scope, script).unwrap();
        let script = v8::Script::compile(scope, code, None).unwrap();

        // Run script
        let result = script.run(scope).unwrap();
        inspect_v8_value(result, scope);
        let result = result.to_string(scope).unwrap();

        // Check if script registered event listeners
        let cloned_handler = {
            let state = scope
                .get_slot::<JsState>()
                .expect("Missing runtime data in V8 context");

            match &state.handler {
                Some(handler) => {
                    println!("Handler found");
                    Some(handler.clone())
                }
                None => {
                    println!("No handler");
                    None
                }
            }
        };

        if let Some(handler) = cloned_handler {
            let handler = Local::new(scope, &handler);

            let args = vec![v8::Integer::new(scope, 1).into()];
            let this = v8::undefined(scope).into();
            let result = handler.call(scope, this, &args).unwrap();
            println!("event result: {:?}", inspect_v8_value(result, scope));
        }

        Some(result.to_rust_string_lossy(scope))
    }
}
