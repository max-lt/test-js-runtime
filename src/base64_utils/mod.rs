use v8::HandleScope;

use crate::base::JsExt;

fn bind_base64(scope: &mut HandleScope) {
    let script = crate::utils::load_script(scope, "atob.js", include_str!("atob.js"));
    let _result = script.run(scope).unwrap();

    let script = crate::utils::load_script(scope, "btoa.js", include_str!("btoa.js"));
    let _result = script.run(scope).unwrap();
}

pub struct Base64UtilsExt;

impl JsExt for Base64UtilsExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>) {
        bind_base64(scope);
    }
}

#[cfg(test)]
mod tests {
    use crate::base::JsContext;
    use crate::base64_utils::Base64UtilsExt;

    fn prepare_context() -> JsContext {
        let mut ctx = JsContext::create();

        ctx.register_module(&Base64UtilsExt);

        ctx
    }

    #[test]
    fn ext_should_set_atob() {
        let mut ctx = prepare_context();

        let result = ctx.run_script("typeof atob === 'function'");

        assert_eq!(result, "true");
    }

    #[test]
    fn ext_should_set_btoa() {
        let mut ctx = prepare_context();

        let result = ctx.run_script("typeof btoa === 'function'");

        assert_eq!(result, "true");
    }

    #[test]
    fn atob_decodes_base64_string() {
        let mut ctx = prepare_context();

        let result = ctx.run_script("atob('SGVsbG8sIFdvcmxkIQ==')");

        assert_eq!(result, "Hello, World!");
    }

    #[test]
    #[should_panic]
    fn atob_handles_invalid_characters() {
        let mut ctx = prepare_context();

        let result = ctx.run_script("atob('SGVsbG8sIFdvcmxkI$Q=')");

        assert_eq!(result, "InvalidCharacterError");
    }

    #[test]
    #[should_panic]
    fn btoa_handles_non_latin1_characters() {
        let mut ctx = prepare_context();

        let result = ctx.run_script("btoa('Hello, 世界!')");

        assert_eq!(result, "InvalidCharacterError");
    }

    #[test]
    fn atob_should_handle_padding() {
        let mut ctx = prepare_context();

        let expect = "{}";

        assert_eq!(expect, ctx.run_script("atob('e30')"));
        assert_eq!(expect, ctx.run_script("atob('e30=')"));
    }
}