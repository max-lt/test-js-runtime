use v8::FunctionCallbackArguments;
use v8::HandleScope;
use v8::ReturnValue;

use crate::base::JsStateRef;
use crate::utils;

mod timers;
mod runtime;

use crate::base::JsExt;

pub use timers::Timers;
pub use runtime::PollTimers;

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

fn set_timer(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
    repeat: bool,
) -> u32 {
    let callback = args.get(0);
    let callback = v8::Local::<v8::Function>::try_from(callback).unwrap();

    let delay = args.get(1);
    let delay = delay.to_uint32(scope).unwrap().value();

    // Convert callback into a global handle to allow moving it between threads.
    let callback = v8::Global::new(scope, callback);

    let state = scope.get_slot::<JsStateRef>().unwrap();
    let state = state.clone();
    let mut state = state.borrow_mut();

    let id = state.timers.create(callback, delay as u64, repeat);

    rv.set(v8::Integer::new(scope, id as i32).into());

    id
}

fn clear_timer(scope: &mut HandleScope, args: FunctionCallbackArguments, _rv: ReturnValue) -> Option<u32> {
    let id = args.get(0);

    if !id.is_uint32() {
        return None;
    }

    let id = id.to_uint32(scope)?.value();

    let state = scope.get_slot::<JsStateRef>().unwrap();
    let state = state.clone();
    let mut state = state.borrow_mut();

    state.timers.remove(id as u32);

    Some(id as u32)
}

fn set_timeout(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue) {
    let id = set_timer(scope, args, rv, false);
    println!("set_timeout: {}", id);
}

fn clear_timeout(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue) {
    let id = clear_timer(scope, args, rv);
    println!("clear_timeout: {:?}", id);
}

fn set_interval(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue) {
    let id = set_timer(scope, args, rv, true);
    println!("set_timeout: {}", id);
}

fn clear_interval(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue) {
    let id = clear_timer(scope, args, rv);
    println!("clear_timeout: {:?}", id);
}

fn queue_microtask(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    println!("queue_microtask");

    let microtask = args.get(0);
    if !microtask.is_function() {
        let message = "Argument must be a function";
        let message = v8::String::new(scope, message).unwrap();
        let exception = v8::Exception::type_error(scope, message);
        rv.set(scope.throw_exception(exception));
        return;
    }

    match v8::Local::<v8::Function>::try_from(microtask) {
        Ok(microtask) => {
            scope.enqueue_microtask(microtask);
        }
        Err(e) => {
            println!("Failed to convert to function: {:?}", e);
            utils::throw_error(scope, "Failed to convert to function");
        }
    };
}
