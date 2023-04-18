use v8::Context;
use v8::ContextScope;
use v8::HandleScope;
use v8::Isolate;
use v8::OwnedIsolate;
use v8::{Global, Local};

use crate::inspect::inspect_v8_value;
use crate::utils::initialize_v8;

pub trait JsExt {
    //  fn bind(scope: &mut HandleScope, context: Local<Context>) -> ();
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>);
}

pub struct JsContext {
    isolate: OwnedIsolate,
    context: Global<Context>,
}

pub struct JsState {
    pub handler: Option<Global<v8::Function>>,
}

impl JsContext {
    /// Create a new context
    pub fn create() -> Self {
        initialize_v8();

        let mut isolate = Isolate::new(Default::default());

        let context = {
            // let mut isolate = &runtime.isolate;
            let scope = &mut HandleScope::new(&mut isolate);

            let context = Context::new(scope);

            let scope = &mut ContextScope::new(scope, context);

            // Remove default console
            {
                let global = context.global(scope);
                let console_key = v8::String::new(scope, "console").unwrap();
                global.delete(scope, console_key.into());
            }

            scope.set_slot(JsState { handler: None });

            let context = Global::new(scope, context);

            context
        };

        JsContext { isolate, context }
    }

    /// Create a new context with default extensions
    pub fn create_init() -> JsContext {
        let mut context = JsContext::create();

        context.register(&crate::console::ConsoleExt);
        context.register(&crate::base64_utils::Base64UtilsExt);
        context.register(&crate::event_listener::EventListerExt);

        context
    }

    /// Register a new extension
    pub fn register<E: JsExt>(&mut self, ext: &E) {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        ext.bind(scope);
    }

    pub fn last_exception(&mut self) -> Option<String> {
        None // TODO
    }

    /// Evaluate a script
    pub fn eval(&mut self, script: &str) -> String {
        let scope = &mut HandleScope::new(&mut self.isolate);

        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        let code = v8::String::new(scope, script).unwrap();
        let script = v8::Script::compile(scope, code, None).unwrap();

        // Run script
        let result = script.run(scope).unwrap();
        inspect_v8_value(result, scope);
        let result = result.to_string(scope).unwrap();

        result.to_rust_string_lossy(scope)
    }

    /// Call fetch event handler
    pub fn fetch(&mut self) -> Option<String> {
        let scope = &mut HandleScope::new(&mut self.isolate);

        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        // Check if script registered event listeners
        let handler = {
            let state = scope
                .get_slot::<JsState>()
                .expect("Missing runtime data in V8 context");

            match &state.handler {
                Some(handler) => Some(handler.clone()),
                None => {
                    println!("No handler registered");
                    None
                }
            }
        };

        if handler.is_none() {
            return None;
        }

        let handler = Local::new(scope, handler.unwrap());
        let undefined = v8::undefined(scope).into();
        let result = handler.call(scope, undefined, &[undefined]).unwrap();
        println!("event result: {:?}", inspect_v8_value(result, scope));
        Some(result.to_string(scope).unwrap().to_rust_string_lossy(scope))
    }
}

#[cfg(test)]
mod tests {
    use crate::base::JsContext;

    fn prepare_context() -> JsContext {
        JsContext::create()
    }

    /// The default context should have default console removed
    #[test]
    fn console_should_be_defined() {
        let mut ctx = prepare_context();

        assert_eq!(ctx.eval("typeof console"), String::from("undefined"));
    }
}
