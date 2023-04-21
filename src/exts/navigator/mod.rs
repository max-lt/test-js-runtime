use v8::HandleScope;
use v8::Local;
use v8::Name;
use v8::PropertyCallbackArguments;
use v8::ReturnValue;

use crate::base::JsExt;

static USER_AGENT: &str = "OpenWorkers/dev";

fn get_user_agent(
    scope: &mut HandleScope,
    _key: Local<Name>,
    _args: PropertyCallbackArguments,
    mut ret: ReturnValue,
) {
    let user_agent = v8::String::new(scope, USER_AGENT).unwrap();

    ret.set(user_agent.into());
}

fn get_navigator(
    scope: &mut HandleScope,
    _key: Local<Name>,
    _args: PropertyCallbackArguments,
    mut ret: ReturnValue,
) {
    let navigator = v8::Object::new(scope);

    let key = v8::String::new(scope, "userAgent").unwrap();

    navigator.set_accessor(scope, key.into(), get_user_agent);

    ret.set(navigator.into());
}

fn bind_navigator(scope: &mut v8::HandleScope) {
    let global = scope.get_current_context().global(scope);

    let key = v8::String::new(scope, "navigator").unwrap();

    global.set_accessor(scope, key.into(), get_navigator);
}

pub struct NavigatorExt;

impl JsExt for NavigatorExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>) {
        bind_navigator(scope);
    }
}

#[cfg(test)]
mod tests {
    use crate::base::JsContext;
    use crate::exts::navigator::NavigatorExt;

    fn prepare_context() -> JsContext {
        let mut ctx = JsContext::create();

        ctx.register(&NavigatorExt);

        ctx
    }

    #[test]
    fn has_navigator_property() {
        let mut ctx = prepare_context();

        let result = ctx.eval("typeof navigator").unwrap();

        assert_eq!(result, String::from("object"));
    }

    #[test]
    fn test_has_navigator_user_agent_property() {
        let mut ctx = prepare_context();

        let result = ctx.eval("typeof navigator.userAgent").unwrap();

        assert_eq!(result, String::from("string"));
    }

    #[test]
    fn test_navigator_user_agent_cannot_be_overwritten() {
        let mut ctx = prepare_context();

        let result = ctx.eval("navigator.userAgent = 'test'").unwrap();

        assert_eq!(result, String::from("test"));

        let result = ctx.eval("navigator.userAgent").unwrap();

        assert_eq!(result, String::from(crate::exts::navigator::USER_AGENT));
    }

    #[test]
    fn test_navigator_can_be_overwritten() {
        let mut ctx = prepare_context();

        let result = ctx.eval("navigator = 'test'").unwrap();

        assert_eq!(result, String::from("test"));

        let result = ctx.eval("typeof navigator").unwrap();

        assert_eq!(result, String::from("object"));
    }
}
