use v8::Context;
use v8::Global;
use v8::OwnedIsolate;

use super::timers::Timers;

pub struct JsRuntime {
  pub(crate) isolate: OwnedIsolate,
  pub(crate) context: Global<Context>,
}

pub trait JsRuntimeMod {
  fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>);
}

pub struct JsState {
  pub handlers: std::collections::HashMap<String, v8::Global<v8::Function>>,
  pub timers: Timers,
}

pub type JsStateRef = std::rc::Rc<std::cell::RefCell<JsState>>;
