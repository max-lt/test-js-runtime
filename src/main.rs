use v8::{HandleScope, Local, Object};

fn console_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    retval: v8::ReturnValue,
) {
    let arg_len = args.length();

    for i in 0..arg_len {
        let arg = args.get(i);
        let arg_str = arg.to_string(scope).unwrap();
        print!(
            "{}{}",
            arg_str.to_rust_string_lossy(scope),
            if i < arg_len - 1 { " " } else { "" }
        );
    }

    println!();
}

fn bind_console(console_object: Local<Object>, scope: &mut HandleScope) {
    let log_levels = ["debug", "error", "info", "log"];
    for &level in log_levels.iter() {
        let mut cb_scope = v8::EscapableHandleScope::new(scope);
        let function_template = v8::FunctionTemplate::new(&mut cb_scope, console_callback);
        let function = function_template.get_function(&mut cb_scope).unwrap();
        let key = v8::String::new(&mut cb_scope, level).unwrap();
        console_object.set(&mut cb_scope, key.into(), function.into());
    }
}

fn inspect_object(value: Local<v8::Value>, scope: &mut HandleScope) {
    // If value is not an object, print it as a string
    if !value.is_object() {
        let value_str = value.to_string(scope).unwrap();
        let value_str = value_str.to_rust_string_lossy(scope);
        println!("INSPECT: {}", value_str);
        return;
    }

    let object = value.to_object(scope).unwrap();
    let keys = object
        .get_own_property_names(scope, v8::GetPropertyNamesArgs::default())
        .unwrap();
    let keys_len = keys.length();
    for i in 0..keys_len {
        let key = keys.get_index(scope, i).unwrap();
        let key_str = key.to_string(scope).unwrap();
        let key_str = key_str.to_rust_string_lossy(scope);
        let value = object.get(scope, key.into()).unwrap();
        let value_str = value.to_string(scope).unwrap();
        let value_str = value_str.to_rust_string_lossy(scope);
        println!("INSPECT {}: {}", key_str, value_str);
    }
}

fn run_script(script: &str) -> Option<String> {
    // Create a new V8 isolate
    let isolate = &mut v8::Isolate::new(Default::default());

    // Create a new V8 HandleScope for managing the lifetime of V8 handles
    let scope = &mut v8::HandleScope::new(isolate);

    // Create a new V8 context
    let context = v8::Context::new(scope);

    // Create a new V8 ContextScope for managing the lifetime of V8 handles
    let scope = &mut v8::ContextScope::new(scope, context);

    // Get global object
    let global = context.global(scope);

    // Destroy default console object
    let console_key = v8::String::new_external_onebyte_static(scope, b"console").unwrap();
    global.delete(scope, console_key.into());

    // Define console as new empty object
    let console_object = v8::Object::new(scope);
    let global = context.global(scope);
    global.set(scope, console_key.into(), console_object.into());
    bind_console(console_object, scope);

    let code = v8::String::new(scope, script).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();

    let result = script.run(scope).unwrap();
    inspect_object(result, scope);
    let result = result.to_string(scope).unwrap();

    Some(result.to_rust_string_lossy(scope))
}

fn initialize_v8() {
    // Initialize V8 runtime
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

fn main() {
    // Initialize V8 runtime
    initialize_v8();

    // Run hello world script with console.log
    {
        let result = crate::run_script("console.log('hello world')");
        println!("result: {:?} ||", result);
    }
    // Run hello world script with console.info
    {
        let result = crate::run_script("console.info('hello world')");
        println!("result: {:?} ||", result);
    }
    // Run hello world script with console.error
    {
        let result = crate::run_script("console.error('hello world')");
        println!("result: {:?} ||", result);
    }
    // Eval console object
    {
        let result = crate::run_script("console");
        println!("result: {:?} ||", result);
    }
    // Eval console object keys
    {
        let result = crate::run_script("Object.keys(console)");
        println!("result: {:?} ||", result);
    }
    // Eval 1+1
    {
        let result = crate::run_script("1+1");
        println!("result: {:?} ||", result);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    fn v8_init() {
        static V8_INIT: Once = Once::new();
        V8_INIT.call_once(crate::initialize_v8);
    }

    #[test]
    fn console_should_be_defined() {
        v8_init();

        let result = crate::run_script("typeof console");
        println!("result: {:?} ||", result);
        assert_eq!(result, Some(String::from("object")));
    }

    #[test]
    fn console_should_have_expected_keys() {
        v8_init();

        let result = crate::run_script("typeof console && Object.keys(console)");

        // Split the result into a vector of strings
        let result = result.unwrap();
        let result = result.split(',').collect::<Vec<&str>>();

        // Assert that the console object has the expected keys

        // 1.1. Logging functions - https://console.spec.whatwg.org/#logging
        assert!(result.contains(&"assert")); // 1.1.1 - https://console.spec.whatwg.org/#assert
        assert!(result.contains(&"clear")); // 1.1.2 - https://console.spec.whatwg.org/#clear
        assert!(result.contains(&"debug")); // 1.1.3 - https://console.spec.whatwg.org/#debug
        assert!(result.contains(&"error")); // 1.1.4 - https://console.spec.whatwg.org/#error
        assert!(result.contains(&"info")); // 1.1.5 - https://console.spec.whatwg.org/#info
        assert!(result.contains(&"log")); // 1.1.6 - https://console.spec.whatwg.org/#log
        assert!(result.contains(&"table")); // 1.1.7 - https://console.spec.whatwg.org/#table
        assert!(result.contains(&"trace")); // 1.1.8 - https://console.spec.whatwg.org/#trace
        assert!(result.contains(&"warn")); // 1.1.9 - https://console.spec.whatwg.org/#warn
        assert!(result.contains(&"dir")); // 1.1.10 - https://console.spec.whatwg.org/#dir
        assert!(result.contains(&"dirxml")); // 1.1.11 - https://console.spec.whatwg.org/#dirxml

        // 1.2. Counting functions - https://console.spec.whatwg.org/#counting
        assert!(result.contains(&"count")); // 1.2.1 - https://console.spec.whatwg.org/#count
        assert!(result.contains(&"countReset")); // 1.2.2 - https://console.spec.whatwg.org/#countreset

        // 1.3. Grouping functions - https://console.spec.whatwg.org/#grouping
        assert!(result.contains(&"group")); // 1.3.1 - https://console.spec.whatwg.org/#group
        assert!(result.contains(&"groupCollapsed")); // 1.3.2 - https://console.spec.whatwg.org/#groupcollapsed
        assert!(result.contains(&"groupEnd")); // 1.3.3 - https://console.spec.whatwg.org/#groupend

        // 1.4. Timing functions - https://console.spec.whatwg.org/#timing
        assert!(result.contains(&"time")); // 1.1 - https://console.spec.whatwg.org/#time
        assert!(result.contains(&"timeLog")); // 1.2 - https://console.spec.whatwg.org/#timelog
        assert!(result.contains(&"timeEnd")); // 1.3 - https://console.spec.whatwg.org/#timeend
        assert!(result.contains(&"timeStamp")); // Non-standard: https://developer.mozilla.org/en-US/docs/Web/API/Console/timeStamp

        assert!(result.contains(&"context")); // ?? What is this?

        // Non-standard functions
        assert!(result.contains(&"profile")); // Non-standard: https://developer.mozilla.org/en-US/docs/Web/API/Console/profile
        assert!(result.contains(&"profileEnd")); // Non-standard: https://developer.mozilla.org/en-US/docs/Web/API/Console/profileEnd
    }

    #[test]
    fn console_should() {
        v8_init();

        let result = crate::run_script("console.log(console)");
        println!("result: {:?} ||", result);
        assert_eq!(result, Some(String::from("undefined")));
    }
}
