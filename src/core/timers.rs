use tokio::time::Duration;
use tokio::time::Instant;

use v8::FunctionCallbackArguments;
use v8::HandleScope;
use v8::ReturnValue;

use std::task::Context;
use std::task::Poll;

use crate::core::JsStateRef;
use crate::utils;

use crate::core::JsRuntimeMod;

pub struct Timer {
    pub(super) timestamp: Instant,
    pub(super) interval: Option<Duration>, // Set if the timer is a repeating timer
    pub(super) callback: v8::Global<v8::Function>,
}

pub struct Timers {
    pub(super) timers: std::collections::HashMap<u32, Timer>,
    next_id: u32,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            timers: std::collections::HashMap::new(),
            next_id: 0,
        }
    }

    pub fn create(&mut self, callback: v8::Global<v8::Function>, delay: u64, repeat: bool) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let duration = std::time::Duration::from_millis(delay);
        let timestamp = Instant::now() + duration;

        let interval = match repeat {
            true => Some(std::time::Duration::from_millis(delay)),
            false => None,
        };

        let timer = Timer {
            callback,
            interval,
            timestamp,
        };

        self.timers.insert(id, timer);

        id
    }

    pub fn remove(&mut self, id: u32) {
        self.timers.remove(&id);
    }
}

pub(crate) fn poll_timers(
    cx: &mut Context,
    scope: &mut v8::ContextScope<v8::HandleScope>,
) -> Poll<()> {
    let state = scope.get_slot::<JsStateRef>().expect("No state found");
    let state = state.clone();
    let mut state_ref = state.borrow_mut();

    let timers = &mut state_ref.timers.timers;

    if timers.is_empty() {
        // No timers to process, so the future is considered complete.
        return Poll::Ready(());
    }

    // Find the timer with the earliest timestamp
    let (timer_id, timestamp) = match timers
        .iter_mut()
        .min_by(|(_, a), (_, b)| a.timestamp.cmp(&b.timestamp))
        .map(|(id, timer)| (*id, timer.timestamp))
    {
        Some(id) => id,
        None => return Poll::Ready(()),
    };

    println!("Found at least one timer");

    let now = Instant::now();
    if timestamp <= now {
        println!("Timer {} is ready to be executed", timer_id);

        // Execute the timer's callback.
        {
            println!("Executing timer {}", timer_id);
            let timer = timers.get(&timer_id).unwrap();
            let undefined = v8::undefined(scope);
            let callback = v8::Local::new(scope, &timer.callback);
            drop(state_ref); // Explicitly drop the mutable borrow before calling the callback
            callback.call(scope, undefined.into(), &[]);
        }

        // Update the timer's timestamp.
        {
            let mut state_ref = state.borrow_mut(); // Re-acquire the mutable borrow after the callback
            let timers = &mut state_ref.timers.timers;
            match timers.get_mut(&timer_id) {
                Some(timer) => match timer.interval {
                    Some(duration) => {
                        // The timer is a repeating timer, so we update its timestamp.
                        timer.timestamp += duration;
                    }
                    None => {
                        // The timer is not a repeating timer, so we remove it from the state.
                        timers.remove(&timer_id);
                    }
                },
                None => {
                    // The timer was removed from the state by the callback.
                    println!("Timer {} was removed by the callback", timer_id)
                }
            }
        }

        // Notify the executor to poll again, as more timers may be ready.
        cx.waker().wake_by_ref();

        // Since we executed a timer, we return `Poll::Pending` to indicate that the future is not complete yet.
        Poll::Pending
    } else {
        // No timers are ready to be executed.

        // Calculate the time until the next timer is ready and register the waker to be woken at that time.
        let sleep_until = timestamp;
        let sleep_duration = sleep_until.saturating_duration_since(now);

        println!("No timers are ready to be executed, sleeping for {:?}", sleep_duration);

        let waker = cx.waker().clone();
        tokio::spawn(async move {
            tokio::time::sleep(sleep_duration).await;
            waker.wake();
        });

        // Since no timer is ready to be executed, we return `Poll::Pending` to indicate that the future is not complete yet.
        Poll::Pending
    }
}

pub struct TimersExt;

impl JsRuntimeMod for TimersExt {
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

fn clear_timer(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    _rv: ReturnValue,
) -> Option<u32> {
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
