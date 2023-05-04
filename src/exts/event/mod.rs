use crate::base::JsExt;

mod event_listener;
mod runtime;

pub use runtime::EventListener;
pub use runtime::trigger_event;

pub struct EventListerExt;

impl JsExt for EventListerExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>) {
        event_listener::bind_event_listener(scope);
    }
}

#[cfg(test)]
mod tests {
    use crate::base::JsRuntime;
    use crate::exts::event::EventListerExt;

    fn prepare_runtime() -> JsRuntime {
        let mut rt = JsRuntime::create();

        rt.register(&EventListerExt);

        rt
    }

    #[test]
    fn add_event_listener_should_be_defined() {
        let mut rt = prepare_runtime();

        let result = rt.eval("typeof addEventListener === 'function'").unwrap();

        assert_eq!(result, "true");
    }
}
