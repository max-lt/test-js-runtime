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
            crate::console::bind_console(scope, context);
            crate::base64::bind_base64(scope, context);

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

        let result = script.run(scope).unwrap();
        inspect_v8_value(result, scope);
        let result = result.to_string(scope).unwrap();

        Some(result.to_rust_string_lossy(scope))
    }
}
