use v8::HandleScope;

use crate::base::JsExt;
use crate::utils;
use crate::utils::init::load_script;

fn bind_base64_utils(scope: &mut HandleScope) {
    let global = scope.get_current_context().global(scope);

    let atob_key = utils::v8_str_static!(scope, b"atob");
    let atob = {
        let context = v8::Context::new(scope);
        let global = context.global(scope);
        let scope = &mut v8::ContextScope::new(scope, context);
        let script = load_script(scope, "atob.js", include_str!("atob.js"));
        let _result = script.run(scope).unwrap();

        global.get(scope, atob_key.into()).unwrap()
    };
    global.set(scope, atob_key.into(), atob);

    let btoa_key = utils::v8_str_static!(scope, b"btoa");
    let btoa = {
        let context = v8::Context::new(scope);
        let global = context.global(scope);
        let scope = &mut v8::ContextScope::new(scope, context);
        let script = load_script(scope, "btoa.js", include_str!("btoa.js"));
        let _result = script.run(scope).unwrap();
        global.get(scope, btoa_key.into()).unwrap()
    };
    global.set(scope, btoa_key.into(), btoa);
}

pub struct Base64UtilsExt;

impl JsExt for Base64UtilsExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>) {
        bind_base64_utils(scope);
    }
}

#[cfg(test)]
mod tests {
    use crate::base::JsContext;
    use crate::exts::base64_utils::Base64UtilsExt;

    fn prepare_context() -> JsContext {
        let mut ctx = JsContext::create();

        ctx.register(&Base64UtilsExt);

        ctx
    }

    #[test]
    fn ext_should_set_atob() {
        let mut ctx = prepare_context();

        let result = ctx.eval("typeof atob === 'function'").unwrap();

        assert_eq!(result, "true");
    }

    #[test]
    fn ext_should_set_btoa() {
        let mut ctx = prepare_context();

        let result = ctx.eval("typeof btoa === 'function'").unwrap();

        assert_eq!(result, "true");
    }

    #[test]
    fn atob_decodes_base64_string() {
        let mut ctx = prepare_context();

        let result = ctx.eval("atob('SGVsbG8sIFdvcmxkIQ==')").unwrap();

        assert_eq!(result, "Hello, World!");
    }

    #[test]
    #[should_panic]
    fn atob_handles_invalid_characters() {
        let mut ctx = prepare_context();

        let result = ctx.eval("atob('SGVsbG8sIFdvcmxkI$Q=')").unwrap();

        assert_eq!(result, "InvalidCharacterError");
    }

    #[test]
    #[should_panic]
    fn btoa_handles_non_latin1_characters() {
        let mut ctx = prepare_context();

        let result = ctx.eval("btoa('Hello, 世界!')").unwrap();

        assert_eq!(result, "InvalidCharacterError");
    }

    #[test]
    fn atob_should_handle_padding() {
        let mut ctx = prepare_context();

        let expect = "{}";

        assert_eq!(expect, ctx.eval("atob('e30')").unwrap());
        assert_eq!(expect, ctx.eval("atob('e30=')").unwrap());
    }
}
