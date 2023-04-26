use std::fmt::Write;
use v8::HandleScope;
use v8::Local;

use crate::base::JsExt;
use crate::utils;
use crate::utils::init::load_script;
use crate::utils::inspect::inspect_v8_value;
use crate::utils::iterator::FunctionCallbackArgumentsExt;

fn logger_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _ret: v8::ReturnValue,
) {
    let mut output = String::new();

    let level = args.get(0).to_rust_string_lossy(scope);

    // If no arguments are passed, just print the level
    if args.length() < 2 {
        println!("console output: [{}]", level);
        return;
    }

    for arg in args.iter().skip(1) {
        if arg.is_string() {
            write!(&mut output, " {}", arg.to_rust_string_lossy(scope))
                .expect("Error writing to output string");
            continue;
        }

        write!(&mut output, " {}", inspect_v8_value(arg, scope))
            .expect("Error writing to output string");
    }

    println!("console output: [{}]{}", level, output);
}

fn bind_console(scope: &mut HandleScope) {
    let context = scope.get_current_context();
    let global = context.global(scope);
    let script = load_script(scope, "console.js", include_str!("console.js"));
    let _result = script.run(scope).unwrap();

    // Get Console class
    let console_key = utils::v8_str_static!(scope, b"buildConsole");
    let console_factory = global.get(scope, console_key.into()).unwrap();
    let console_factory: Local<v8::Function> = console_factory.try_into().unwrap();

    // Set the logger_callback function as 'consoleLogger' property of the global object
    let console_log_template = v8::FunctionTemplate::new(scope, logger_callback);
    let console_log_function = console_log_template.get_function(scope).unwrap();

    // Set the Console instance as a property of the global object
    let undefined = v8::undefined(scope);
    let console_instance_key: Local<v8::String> = utils::v8_str_static!(scope, b"console");
    let console_instance = console_factory
        .call(scope, undefined.into(), &[console_log_function.into()])
        .unwrap();

    global.set(scope, console_instance_key.into(), console_instance.into());

    // Delete the Console class from the global object
    global.delete(scope, console_key.into());
}

pub struct ConsoleExt;

impl JsExt for ConsoleExt {
    fn bind<'s>(&self, scope: &mut v8::HandleScope<'s>) {
        bind_console(scope);
    }
}

#[cfg(test)]
mod tests {
    use crate::base::JsRuntime;
    use crate::exts::console::ConsoleExt;

    fn prepare_runtime() -> JsRuntime {
        let mut rt = JsRuntime::create();

        rt.register(&ConsoleExt);

        rt
    }

    #[test]
    fn console_should_be_defined() {
        let mut rt = prepare_runtime();

        let result = rt.eval("typeof console").unwrap();

        println!("result: {:?} ||", result);
        assert_eq!(result, String::from("object"));
    }

    #[test]
    fn console_keys_should_be_enumerable() {
        let mut rt = prepare_runtime();

        let result = rt.eval("Object.keys(console).length").unwrap();

        println!("result: {:?} ||", result);
        assert_eq!(result, String::from("20"));
    }

    #[test]
    fn console_should_have_expected_keys() {
        let mut rt = prepare_runtime();

        let result = rt.eval("Object.keys(console).toString()").unwrap();

        println!("result: {:?} ||", result);

        let function_str = String::from("function");

        // Assert that the console object has the expected keys

        // 1.1. Logging functions - https://console.spec.whatwg.org/#logging
        assert!(result.contains(&"assert")); // 1.1.1 - https://console.spec.whatwg.org/#assert
        assert_eq!(function_str, rt.eval("typeof console.assert").unwrap());
        assert!(result.contains(&"clear")); // 1.1.2 - https://console.spec.whatwg.org/#clear
        assert_eq!(function_str, rt.eval("typeof console.clear").unwrap());
        assert!(result.contains(&"debug")); // 1.1.3 - https://console.spec.whatwg.org/#debug
        assert_eq!(function_str, rt.eval("typeof console.debug").unwrap());
        assert!(result.contains(&"error")); // 1.1.4 - https://console.spec.whatwg.org/#error
        assert_eq!(function_str, rt.eval("typeof console.error").unwrap());
        assert!(result.contains(&"info")); // 1.1.5 - https://console.spec.whatwg.org/#info
        assert_eq!(function_str, rt.eval("typeof console.info").unwrap());
        assert!(result.contains(&"log")); // 1.1.6 - https://console.spec.whatwg.org/#log
        assert_eq!(function_str, rt.eval("typeof console.log").unwrap());
        assert!(result.contains(&"table")); // 1.1.7 - https://console.spec.whatwg.org/#table
        assert_eq!(function_str, rt.eval("typeof console.table").unwrap());
        assert!(result.contains(&"trace")); // 1.1.8 - https://console.spec.whatwg.org/#trace
        assert_eq!(function_str, rt.eval("typeof console.trace").unwrap());
        assert!(result.contains(&"warn")); // 1.1.9 - https://console.spec.whatwg.org/#warn
        assert_eq!(function_str, rt.eval("typeof console.warn").unwrap());
        assert!(result.contains(&"dir")); // 1.1.10 - https://console.spec.whatwg.org/#dir
        assert_eq!(function_str, rt.eval("typeof console.dir").unwrap());
        assert!(result.contains(&"dirxml")); // 1.1.11 - https://console.spec.whatwg.org/#dirxml
        assert_eq!(function_str, rt.eval("typeof console.dirxml").unwrap());

        // 1.2. Counting functions - https://console.spec.whatwg.org/#counting
        assert!(result.contains(&"count")); // 1.2.1 - https://console.spec.whatwg.org/#count
        assert_eq!(function_str, rt.eval("typeof console.count").unwrap());
        assert!(result.contains(&"countReset")); // 1.2.2 - https://console.spec.whatwg.org/#countreset
        assert_eq!(function_str, rt.eval("typeof console.countReset").unwrap());

        // 1.3. Grouping functions - https://console.spec.whatwg.org/#grouping
        assert!(result.contains(&"group")); // 1.3.1 - https://console.spec.whatwg.org/#group
        assert_eq!(function_str, rt.eval("typeof console.group").unwrap());
        assert!(result.contains(&"groupCollapsed")); // 1.3.2 - https://console.spec.whatwg.org/#groupcollapsed
        assert_eq!(function_str, rt.eval("typeof console.groupCollapsed").unwrap());
        assert!(result.contains(&"groupEnd")); // 1.3.3 - https://console.spec.whatwg.org/#groupend
        assert_eq!(function_str, rt.eval("typeof console.groupEnd").unwrap());

        // 1.4. Timing functions - https://console.spec.whatwg.org/#timing
        assert!(result.contains(&"time")); // 1.1 - https://console.spec.whatwg.org/#time
        assert_eq!(function_str, rt.eval("typeof console.time").unwrap());
        assert!(result.contains(&"timeLog")); // 1.2 - https://console.spec.whatwg.org/#timelog
        assert_eq!(function_str, rt.eval("typeof console.timeLog").unwrap());
        assert!(result.contains(&"timeEnd")); // 1.3 - https://console.spec.whatwg.org/#timeend
        assert_eq!(function_str, rt.eval("typeof console.timeEnd").unwrap());
        assert!(result.contains(&"timeStamp")); // Non-standard: https://developer.mozilla.org/en-US/docs/Web/API/Console/timeStamp
        assert_eq!(function_str, rt.eval("typeof console.timeStamp").unwrap());
    }

    #[test]
    fn test_console_log() {
        let mut rt = prepare_runtime();

        let expect = "hello world";

        let result = rt.eval(&format!("console.log('{}');", expect));
        println!("result: {:?} ||", result);

        // TODO: console output should be part of rt
        // assert_eq!(result, expect);
    }
}
