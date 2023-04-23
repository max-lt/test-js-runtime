use v8::HandleScope;
use v8::ReturnValue;
use v8::FunctionCallbackArguments;

use crate::base::JsExt;

pub struct TimersExt;

impl JsExt for TimersExt {
    fn bind<'s>(&self, scope: &mut HandleScope<'s>) {
        bind_timers(scope);
    }
}

fn bind_timers(scope: &mut HandleScope) {
  let global = scope.get_current_context().global(scope);

  // Set up the setTimeout function
  let set_timeout_key = v8::String::new(scope, "setTimeout").unwrap();
  let set_timeout_fn = v8::Function::new(scope, set_timeout).unwrap();
  global.set(scope, set_timeout_key.into(), set_timeout_fn.into());

  // Set up the clearTimeout function
  let clear_timeout_key = v8::String::new(scope, "clearTimeout").unwrap();
  let clear_timeout_fn = v8::Function::new(scope, clear_timeout).unwrap();
  global.set(scope, clear_timeout_key.into(), clear_timeout_fn.into());

  // Set up the setInterval function
  let set_interval_key = v8::String::new(scope, "setInterval").unwrap();
  let set_interval_fn = v8::Function::new(scope, set_interval).unwrap();
  global.set(scope, set_interval_key.into(), set_interval_fn.into());

  // Set up the clearInterval function
  let clear_interval_key = v8::String::new(scope, "clearInterval").unwrap();
  let clear_interval_fn = v8::Function::new(scope, clear_interval).unwrap();
  global.set(scope, clear_interval_key.into(), clear_interval_fn.into());

  // Set up the queueMicrotask function
  let queue_microtask_key = v8::String::new(scope, "queueMicrotask").unwrap();
  let queue_microtask_fn = v8::Function::new(scope, queue_microtask).unwrap();
  global.set(scope, queue_microtask_key.into(), queue_microtask_fn.into());
}

fn set_timeout(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
  println!("set_timeout");
}

fn clear_timeout(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
  println!("clear_timeout");
}

fn set_interval(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
  println!("set_interval");
}

fn clear_interval(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
  println!("clear_interval");
}

fn queue_microtask(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
  println!("queue_microtask");
}
