mod base;
mod base64;
mod console;
mod inspect;
mod utils;
mod v8_ext;

use crate::base::JsRuntime;

fn main() {
    let mut runtime = JsRuntime::new();

    let mut ctx = runtime.create_context();

    // Get file to read from args
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file = &args[1];

    // Adjust file path from user context
    let file = std::path::Path::new(file).canonicalize().unwrap();

    // Read file
    let contents = std::fs::read_to_string(file).expect("Something went wrong reading the file");

    // Run script
    let result = ctx.run_script(&contents);

    println!("result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use crate::base::JsRuntime;

    #[test]
    fn console_should_be_defined() {
        let mut runtime = JsRuntime::new();

        let mut ctx = runtime.create_context();

        // let isolate = &mut runtime.isolate;

        let result = ctx.run_script("typeof console");
        println!("result: {:?} ||", result);
        assert_eq!(result, Some(String::from("object")));
    }

    #[test]
    fn console_should_have_expected_keys() {
        let mut runtime = JsRuntime::new();

        let mut ctx = runtime.create_context();

        // let isolate = &mut runtime.isolate;

        let result = ctx.run_script("typeof console && Object.keys(console)");

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
}
