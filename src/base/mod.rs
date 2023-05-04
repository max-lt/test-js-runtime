mod runtime;

pub use crate::base::runtime::JsRuntime;
pub use crate::exts::timers::Timers;

pub trait JsExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>);
}

pub struct JsState {
    pub handlers: std::collections::HashMap<String, v8::Global<v8::Function>>,
    pub timers: Timers
}

pub type JsStateRef = std::rc::Rc<std::cell::RefCell<JsState>>;

#[cfg(test)]
mod tests {
    use crate::base::runtime::EvalError;
    use crate::base::runtime::JsRuntime;

    /// The default runtime should have default console removed
    #[test]
    fn console_should_not_be_defined() {
        let mut rt = JsRuntime::create();

        let result = rt.eval("typeof console").unwrap();

        assert_eq!(result, String::from("undefined"));
    }

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
}
