use crate::base::JsExt;

pub mod request;
pub mod response;
pub mod fetch_event;

mod event_listener;

pub struct FetchExt;

impl JsExt for FetchExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>) {
        event_listener::bind_event_listener(scope);

        response::bind_response_constructor(scope);
    }
}

#[cfg(test)]
mod tests {
    use crate::base::JsRuntime;
    use crate::exts::fetch::FetchExt;

    fn prepare_runtime() -> JsRuntime {
        let mut rt = JsRuntime::create();

        rt.register(&FetchExt);

        rt
    }

    #[test]
    fn add_event_listener_should_be_defined() {
        let mut rt = prepare_runtime();

        let result = rt.eval("typeof addEventListener === 'function'").unwrap();

        assert_eq!(result, "true");
    }

    #[test]
    fn response_should_be_defined() {
        let mut rt = prepare_runtime();

        let result = rt.eval("typeof Response === 'function'").unwrap();

        assert_eq!(result, "true");
    }
}
