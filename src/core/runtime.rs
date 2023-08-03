use v8::Context;
use v8::ContextScope;
use v8::Global;
use v8::HandleScope;
use v8::Isolate;
use v8::Local;
use v8::Value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::task::Poll;

use crate::utils::init::initialize_v8;
use crate::utils::inspect::inspect_v8_value;

use super::base::JsRuntime;
use super::event::JsEventTrait;
use super::timers::Timers;
use super::JsRuntimeMod;
use super::JsState;
use super::JsStateRef;

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

impl JsRuntime {
    /// Create a new runtime
    pub fn create() -> Self {
        initialize_v8();

        let mut isolate = Isolate::new(Default::default());

        isolate.set_capture_stack_trace_for_uncaught_exceptions(false, 0);
        isolate.set_promise_reject_callback(promise_reject_callback);
        isolate.add_message_listener(message_callback);

        let context = {
            let scope = &mut HandleScope::new(&mut isolate);

            let context = Context::new(scope);

            let scope = &mut ContextScope::new(scope, context);

            // Remove default console
            {
                let global = context.global(scope);
                let console_key = v8::String::new(scope, "console").unwrap();
                global.delete(scope, console_key.into());
            }

            scope.set_slot(Rc::new(RefCell::new(JsState {
                handlers: HashMap::new(),
                timers: Timers::new(),
            })));

            let context = Global::new(scope, context);

            context
        };

        let mut rt = JsRuntime { isolate, context };

        rt.register(&super::console::ConsoleExt);
        rt.register(&super::event_listener::EventListerExt);
        rt.register(&super::timers::TimersExt);

        rt
    }

    /// Register a new extension
    fn register<Mod: JsRuntimeMod>(&mut self, ext: &Mod) {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        ext.bind(scope);
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

    pub async fn run_event_loop<'a>(&mut self) {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        tokio::macros::support::poll_fn(|cx| Self::poll_timers(cx, scope)).await
    }

    fn poll_timers(cx: &mut std::task::Context, scope: &mut ContextScope<HandleScope>) -> Poll<()> {
        super::timers::poll_timers(cx, scope)
    }

    pub fn dispatch_event<E: JsEventTrait>(&mut self, event: E) -> Option<Local<Value>> {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        // Get handler - State must be dropped before the handler is called
        let handler = {
            let state = scope.get_slot::<JsStateRef>().expect("No state found");
            let state = state.borrow();
            match state.handlers.get(&event.event_type()) {
                Some(handler) => handler.clone(),
                None => {
                    println!("No handler registered");
                    return None;
                }
            }
        };

        // Prepare handler call
        let handler = v8::Local::new(scope, handler);
        let undefined = v8::undefined(scope).into();

        // Call handler
        // let result = match event_data {
        //     Some(event_data) => handler.call(scope, undefined, &[event_data]),
        //     None => handler.call(scope, undefined, &[]),
        // };
        let result = handler.call(scope, undefined, &[]);

        println!("Event result: {:?}", result);

        result
    }
}
