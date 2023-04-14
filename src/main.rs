fn console_log_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let arg_len = args.length();
    for i in 0..arg_len {
        let arg = args.get(i);
        let arg_str = arg.to_string(scope).unwrap();
        print!("{}{}", arg_str.to_rust_string_lossy(scope), if i < arg_len - 1 { " " } else { "" });
    }
    println!();
}

fn run_script(script: &str) -> Option<std::string::String> {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    // Create the console.log function
    let console_log_template = v8::FunctionTemplate::new(scope, console_log_callback);
    let console_log_function = console_log_template.get_function(scope).unwrap();

    // Bind the console.log function to the global console object
    let global = context.global(scope);
    let console_key = v8::String::new(scope, "console").unwrap();
    let console_object = global.get(scope, console_key.into()).unwrap();
    let console_object = console_object.to_object(scope).unwrap();
    let log_key = v8::String::new(scope, "log").unwrap();
    console_object.set(scope, log_key.into(), console_log_function.into());

    let code = v8::String::new(scope, script).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();

    Some(result.to_rust_string_lossy(scope))
}

fn main() {
    let script = r#"
        console.log('Hello', 'World');
    "#;

    let result = run_script(script);
    println!("result: {:?} ||", result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_not_have_console() {
        let result = crate::run_script("typeof console");
        println!("result: {:?} ||", result);
        // assert_eq!(result, Some(String::from("undefined")));
    }
}
