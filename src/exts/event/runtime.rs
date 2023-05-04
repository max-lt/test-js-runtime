use crate::base::JsRuntime;
use crate::base::JsStateRef;

use v8::ContextScope;
use v8::HandleScope;
use v8::Local;
use v8::Value;

pub fn trigger_event<'a>(
    event: &str,
    scope: &mut v8::ContextScope<'_, v8::HandleScope<'a>>,
    event_data: Option<v8::Local<v8::Value>>,
) -> Option<v8::Local<'a, v8::Value>> {
    // Get handler - State must be dropped before the handler is called
    let handler = {
        let state = scope.get_slot::<JsStateRef>().expect("No state found");
        let state = state.borrow();
        match state.handlers.get(event) {
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
    let result = match event_data {
        Some(event_data) => handler.call(scope, undefined, &[event_data]),
        None => handler.call(scope, undefined, &[]),
    };

    println!("Event result: {:?}", result);

    result
}

pub trait EventListener {
    fn dispatch_event(&mut self, event: &str) -> Option<Local<Value>>;
}

impl EventListener for JsRuntime {
    fn dispatch_event(&mut self, event: &str) -> Option<Local<Value>> {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        trigger_event(event, scope, None)
    }
}
