use v8::Context;
use v8::ContextScope;
use v8::Global;
use v8::HandleScope;
use v8::Isolate;
use v8::Local;
use v8::OwnedIsolate;

use std::collections::HashMap;

use crate::utils::init::initialize_v8;
use crate::utils::inspect::inspect_v8_value;

use super::JsState;
use super::JsExt;

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
  pub(crate) isolate: OwnedIsolate,
  pub(crate) context: Global<Context>,
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

impl JsRuntime {
  /// Create a new runtime
  pub fn create() -> Self {
      initialize_v8();

      let mut isolate = Isolate::new(Default::default());

      println!("Microtasks policy: {:?}", isolate.get_microtasks_policy());
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

          scope.set_slot(JsState {
              handlers: HashMap::new(),
          });

          let context = Global::new(scope, context);

          context
      };

      JsRuntime { isolate, context }
  }

  /// Create a new context with default extensions
  pub fn create_init() -> JsRuntime {
      let mut rt = JsRuntime::create();

      rt.register(&crate::exts::console::ConsoleExt);
      rt.register(&crate::exts::base64_utils::Base64UtilsExt);
      rt.register(&crate::exts::event::EventListerExt);
      rt.register(&crate::exts::fetch::FetchExt);
      rt.register(&crate::exts::navigator::NavigatorExt);
      rt.register(&crate::exts::timers::TimersExt);

      rt
  }

  /// Register a new extension
  pub fn register<E: JsExt>(&mut self, ext: &E) {
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
}
