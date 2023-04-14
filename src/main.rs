fn run_script(script: &str) -> Option<String> {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, script).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();

    Some(result.to_rust_string_lossy(scope))
}

fn main() {
    let result = run_script("typeof console !== 'undefined' && Object.keys(console)");
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
