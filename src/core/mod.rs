mod runtime;
mod base;

mod event;
mod console;
mod timers;
mod event_listener;

pub use crate::core::event::JsEvent;

pub use crate::core::base::JsRuntime;
pub use crate::core::base::JsRuntimeMod;
pub use crate::core::base::JsState;
pub use crate::core::base::JsStateRef;

#[cfg(test)]
mod tests {
    use crate::core::runtime::EvalError;
    use crate::core::base::JsRuntime;

    /// The default runtime should have default console removed
    // #[test]
    // fn console_should_not_be_defined() {
    //     let mut rt = JsRuntime::create();

    //     let result = rt.eval("typeof console").unwrap();

    //     assert_eq!(result, String::from("undefined"));
    // }

    /// eval should not panic when js exception is thrown
    #[test]
    fn eval_should_not_panic_on_runtime_error() {
        let mut rt = JsRuntime::create();

        let result = rt.eval("throw new Error('test')");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EvalError::RuntimeError);
    }

    /// eval should not panic when js exception is thrown
    #[test]
    fn eval_should_not_panic_on_compile_error() {
        let mut rt = JsRuntime::create();

        let result = rt.eval("}");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EvalError::CompileError);
    }

    #[test]
    fn eval_should_not_panic_on_dynamic_import() {
        let mut rt = JsRuntime::create();

        let result = rt.eval("import('moduleName')").unwrap();

        assert_eq!(result, String::from("[object Promise]"));

        // TODO: promise should have been rejected
    }

    #[test]
    fn eval_should_not_have() {
        let mut rt = JsRuntime::create();

        let result = rt.eval("typeof import");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EvalError::CompileError);
    }

    #[test]
    fn add_event_listener_should_be_defined() {
        let mut rt = JsRuntime::create();

        let result = rt.eval("typeof addEventListener").unwrap();

        assert_eq!(result, "function");
    }
}
