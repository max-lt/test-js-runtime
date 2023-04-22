pub mod init;
pub mod inspect;
pub mod iterator;

pub fn assign<'a>(
    scope: &mut v8::ContextScope<'_, v8::HandleScope<'a>>,
    object: v8::Local<'a, v8::Object>,
    key: &str,
    value: v8::Local<'a, v8::Value>,
) {
    let key = v8::String::new(scope, &key).unwrap();
    object.set(scope, key.into(), value);
}

pub fn assign_string<'a>(
    scope: &mut v8::ContextScope<'_, v8::HandleScope<'a>>,
    object: v8::Local<'a, v8::Object>,
    key: &str,
    value: String,
) {
    let key = v8::String::new(scope, &key).unwrap();
    let value = v8::String::new(scope, &value).unwrap();
    object.set(scope, key.into(), value.into());
}
