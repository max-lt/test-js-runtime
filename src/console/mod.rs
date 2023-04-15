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

// Read the console module from the filesystem
fn read_console_module() -> &'static str {
    static CONSOLE_MODULE: &str = include_str!("console.js");

    CONSOLE_MODULE
}

pub fn bind_console(scope: &mut HandleScope, context: Local<Context>) {
    let global = context.global(scope);
    let script = read_console_module();

    let code = v8::String::new(scope, script).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let _result = script.run(scope).unwrap();

    // Get Console class
    let console_key = v8::String::new_external_onebyte_static(scope, b"Console").unwrap();
    let console_class = global.get(scope, console_key.into()).unwrap();

    // Create an instance of Console class
    let console_ctor: Local<v8::Function> = console_class.try_into().unwrap();

    // Set the logger_callback function as 'consoleLogger' property of the global object
    let console_log_template = v8::FunctionTemplate::new(scope, logger_callback);
    let console_log_function = console_log_template.get_function(scope).unwrap();

    // Set the Console instance as a property of the global object
    let console_instance_key = v8::String::new_external_onebyte_static(scope, b"console").unwrap();
    let console_instance = console_ctor
        .new_instance(scope, &[console_log_function.into()])
        .unwrap();

    global.set(scope, console_instance_key.into(), console_instance.into());

    // Delete the Console class from the global object
    global.delete(scope, console_key.into());
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
