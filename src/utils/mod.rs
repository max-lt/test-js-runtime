pub mod file;
pub mod init;
pub mod inspect;
pub mod iterator;

pub use inspect::inspect_v8_value;

pub fn assign<'a>(
    scope: &mut v8::HandleScope<'a>,
    object: v8::Local<'a, v8::Object>,
    key: &str,
    value: v8::Local<'a, v8::Value>,
) {
    let key = v8::String::new(scope, &key).unwrap();
    object.set(scope, key.into(), value);
}

pub fn get<'a>(scope: &mut v8::HandleScope<'a>, object: v8::Local<'a, v8::Object>, key: &str) -> v8::Local<'a, v8::Value> {
    let key = v8::String::new(scope, &key).unwrap();
    object.get(scope, key.into()).unwrap()
}

pub fn assign_string<'a>(
    scope: &mut v8::HandleScope<'a>,
    object: v8::Local<'a, v8::Object>,
    key: &str,
    value: String,
) {
    let key = v8::String::new(scope, &key).unwrap();
    let value = v8::String::new(scope, &value).unwrap();
    object.set(scope, key.into(), value.into());
}

pub fn throw_type_error<'a>(
    scope: &mut v8::HandleScope<'a>,
    message: &str,
) -> v8::Local<'a, v8::Value> {
    let message = v8::String::new(scope, message).unwrap();
    let exception = v8::Exception::type_error(scope, message);
    scope.throw_exception(exception)
}

pub fn throw_error<'a>(scope: &mut v8::HandleScope<'a>, message: &str) -> v8::Local<'a, v8::Value> {
    let message = v8::String::new(scope, message).unwrap();
    let exception = v8::Exception::error(scope, message);
    scope.throw_exception(exception)
}

// Creates a v8::String from a `&'static [u8]`,
// must be Latin-1 or ASCII, not UTF-8 !
macro_rules! v8_str_static {
    ($scope:expr, $s:expr) => {
        v8::String::new_external_onebyte_static($scope, $s).unwrap()
    };
}

pub(crate) use v8_str_static;
