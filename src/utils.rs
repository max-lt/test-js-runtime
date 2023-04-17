use std::sync::Once;

fn _initialize_v8() {
    // Initialize V8 runtime
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

pub fn initialize_v8() {
    static V8_INIT: Once = Once::new();
    V8_INIT.call_once(_initialize_v8);
}

pub fn load_script<'s>(
    scope: &mut v8::HandleScope<'s>,
    name: &str,
    source: &str
) -> v8::Local<'s, v8::Script> {
    let name = v8::String::new(scope, name).unwrap();
    let source = v8::String::new(scope, source).unwrap();
    let source_map_url = v8::String::empty(scope);
    let origin = v8::ScriptOrigin::new(
      scope,
      name.into(),
      0,
      0,
      false,
      123,
      source_map_url.into(),
      true,
      false,
      false,
    );

    v8::Script::compile(scope, source, Some(&origin)).unwrap()
}
