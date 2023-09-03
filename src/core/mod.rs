mod runtime;
mod message;

pub use message::RuntimeBasicMessage;
pub use message::RuntimeMessage;
pub use runtime::JsRuntime;

pub struct JsState {
    pub handler: Option<v8::Global<v8::Function>>
}

pub type JsStateRef = std::rc::Rc<std::cell::RefCell<JsState>>;

#[cfg(test)]
mod tests {
    use crate::core::runtime::EvalError;
    use crate::core::runtime::JsRuntime;

    /// eval should not panic when js exception is thrown
    #[test]
    fn rt_should_not_panic_on_runtime_error() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("throw new Error('test')");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EvalError::RuntimeError);
    }

    /// eval should not panic when js exception is thrown
    #[test]
    fn rt_should_not_panic_on_compile_error() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("}");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EvalError::CompileError);
    }

    #[test]
    fn rt_should_not_panic_on_dynamic_import() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("import('moduleName')").unwrap();

        assert_eq!(result, String::from("[object Promise]"));

        // TODO: promise should have been rejected
    }

    #[test]
    fn rt_should_not_have_import() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("typeof import");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EvalError::CompileError);
    }


    #[test]
    fn rt_should_have_eval() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("typeof dispatchEvent").unwrap();

        assert_eq!(result, String::from("function"));
    }

    #[test]
    fn rt_should_have_dispatch_event() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("typeof dispatchEvent").unwrap();

        assert_eq!(result, String::from("function"));
    }

    #[test]
    fn rt_should_have_post_message() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("typeof postMessage").unwrap();

        assert_eq!(result, String::from("function"));
    }

    #[test]
    fn rt_should_have_this() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("this === globalThis").unwrap();

        assert_eq!(result, String::from("true"));
    }

    #[test]
    fn rt_should_have_self() {
        let mut rt = JsRuntime::create_init();

        let result = rt.eval("self === globalThis").unwrap();

        assert_eq!(result, String::from("true"));
    }
}
