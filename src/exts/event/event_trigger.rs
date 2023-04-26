use crate::base::JsState;

pub fn trigger_event<'a>(
    event: &str,
    scope: &mut v8::ContextScope<'_, v8::HandleScope<'a>>,
    event_data: Option<v8::Local<v8::Value>>,
) -> Option<v8::Local<'a, v8::Value>> {
    // Get state
    let state = scope.get_slot::<JsState>().expect("No state found");
    let handler = match state.handlers.get(event) {
        Some(handler) => Some(handler.clone()),
        None => {
            println!("No handler registered");
            return None;
        }
    };

    let handler = v8::Local::new(scope, handler.unwrap());
    let undefined = v8::undefined(scope).into();

    let result = match event_data {
        Some(event_data) => handler.call(scope, undefined, &[event_data]),
        None => handler.call(scope, undefined, &[]),
    };

    println!("Event result: {:?}", result);

    result
}
