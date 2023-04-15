
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
