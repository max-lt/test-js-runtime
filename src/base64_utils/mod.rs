use base64::Engine;
use v8::{Context, HandleScope, Local};

fn add_padding(base64_input: String) -> String {
    let padding_needed = (4 - base64_input.len() % 4) % 4;
    format!("{}{}", base64_input, "=".repeat(padding_needed))
}

fn atob(scope: &mut HandleScope, args: v8::FunctionCallbackArguments, mut ret: v8::ReturnValue) {
    // Check if there's at least one argument
    if args.length() < 1 {
        let exception_str =
            v8::String::new(scope, "1 argument required, but only 0 present").unwrap();
        let exception = v8::Exception::error(scope, exception_str);
        scope.throw_exception(exception);
        return;
    }

    let str = args.get(0);
    let str = add_padding(str.to_rust_string_lossy(scope));

    let engine = base64::engine::general_purpose::URL_SAFE;
    match engine.decode(&str) {
        Ok(decoded) => {
            // Convert the decoded Vec<u8> to a V8 string and set it as the return value
            let decoded_str = v8::String::new(scope, &String::from_utf8_lossy(&decoded)).unwrap();
            ret.set(decoded_str.into());
        }
        Err(_) => {
            // Handle base64 decode error, for example, by throwing a V8 exception
            let exception_str = v8::String::new(scope, "Invalid base64 input string").unwrap();
            let exception = v8::Exception::error(scope, exception_str);
            scope.throw_exception(exception);
        }
    }
}

fn btoa(scope: &mut HandleScope, args: v8::FunctionCallbackArguments, mut ret: v8::ReturnValue) {
    // Check if there's at least one argument
    if args.length() < 1 {
        let exception_str =
            v8::String::new(scope, "1 argument required, but only 0 present").unwrap();
        let exception = v8::Exception::error(scope, exception_str);
        scope.throw_exception(exception);
        return;
    }

    // Get the first argument and convert it to a Rust string
    let input_str = args.get(0);
    let input_str = input_str.to_rust_string_lossy(scope);

    // Encode the input string as base64
    let engine = base64::engine::general_purpose::URL_SAFE;
    let encoded = engine.encode(&input_str.into_bytes());

    // Convert the encoded base64 string to a V8 string and set it as the return value
    let encoded_str = v8::String::new(scope, &encoded).unwrap();
    ret.set(encoded_str.into());
}

pub fn bind_base64(scope: &mut HandleScope, context: Local<Context>) {
    let global = context.global(scope);

    // Bind atob
    {
        let atob_key = v8::String::new_external_onebyte_static(scope, b"atob").unwrap();
        let function_template = v8::FunctionTemplate::new(scope, atob);
        let function = function_template.get_function(scope).unwrap();
        global.set(scope, atob_key.into(), function.into());
    }

    // Bind btoa
    {
        let btoa_key = v8::String::new_external_onebyte_static(scope, b"btoa").unwrap();
        let function_template = v8::FunctionTemplate::new(scope, btoa);
        let function = function_template.get_function(scope).unwrap();
        global.set(scope, btoa_key.into(), function.into());
    }
}

#[cfg(test)]
mod tests {
    // use crate::base::JsRuntime;

    // #[test]
    // fn test_console_log() {
    //     let mut runtime = JsRuntime::new();

    //     // Run hello world script with console.log
    //     {
    //         let result = runtime.run_script("console.log('hello world')");
    //         assert_eq!(result, Some("hello world".to_string()));
    //     }
    // }

    // #[test]
    // fn test_console_info() {
    //     let mut runtime = JsRuntime::new();

    //     // Run hello world script with console.info
    //     {
    //         let result = runtime.run_script("console.info('hello world')");
    //         assert_eq!(result, Some("hello world".to_string()));
    //     }
    // }
}
