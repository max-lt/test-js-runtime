use std::fmt::Write;
use v8::{Context, HandleScope, Local};

// mod crate::v8_ext;
use crate::v8_ext::iterator::FunctionCallbackArgumentsExt;
use crate::inspect::inspect_v8_value;

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

pub fn bind_console(scope: &mut HandleScope, context: Local<Context>) {
    let global = context.global(scope);
    let script = crate::utils::load_script(scope, "console.js", include_str!("console.js"));
    let _result = script.run(scope).unwrap();

    // Get Console class
    let console_key = v8::String::new_external_onebyte_static(scope, b"buildConsole").unwrap();
    let console_factory = global.get(scope, console_key.into()).unwrap();
    let console_factory: Local<v8::Function> = console_factory.try_into().unwrap();

    // Set the logger_callback function as 'consoleLogger' property of the global object
    let console_log_template = v8::FunctionTemplate::new(scope, logger_callback);
    let console_log_function = console_log_template.get_function(scope).unwrap();

      // Set the Console instance as a property of the global object
    let undefined = v8::undefined(scope);
    let console_instance_key: Local<v8::String> = v8::String::new_external_onebyte_static(scope, b"console").unwrap();
    let console_instance = console_factory
        .call(scope, undefined.into(), &[console_log_function.into()])
        .unwrap();

    global.set(scope, console_instance_key.into(), console_instance.into());

    // Delete the Console class from the global object
    global.delete(scope, console_key.into());
}

#[cfg(test)]
mod tests {
  use crate::base::JsRuntime;

  #[test]
  fn console_should_be_defined() {
      let mut runtime = JsRuntime::new();
      let mut ctx = runtime.create_context();

      let result = ctx.run_script("typeof console");

      println!("result: {:?} ||", result);
      assert_eq!(result, String::from("object"));
  }

  #[test]
  fn console_keys_should_be_enumerable() {
      let mut runtime = JsRuntime::new();
      let mut ctx = runtime.create_context();

      let result = ctx.run_script("Object.keys(console).length");

      println!("result: {:?} ||", result);
      assert_eq!(result, String::from("20"));
  }

  #[test]
  fn console_should_have_expected_keys() {
      let mut runtime = JsRuntime::new();

      let mut ctx = runtime.create_context();

      let result = ctx.run_script("Object.keys(console).toString()");

      println!("result: {:?} ||", result);

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
  }
}
