mod base;
mod base64;
mod console;
mod event_listener;
mod inspect;
mod utils;
mod v8_ext;

use crate::base::JsRuntime;

fn main() {
    let mut runtime = JsRuntime::new();
    let mut ctx = runtime.create_context();

    // Get arguments
    let args: Vec<String> = std::env::args().collect();

    // Run script or eval code
    let result = match args.get(1) {
        Some(arg) if arg == "eval" => match args.get(2) {
            Some(code) => ctx.run_script(code),
            None => {
                eprintln!("Usage: {} eval <code>", args[0]);
                std::process::exit(1);
            }
        },
        Some(file) => {
            let file = std::path::Path::new(file)
                .canonicalize()
                .unwrap_or_else(|_| {
                    eprintln!("Error: Invalid file path");
                    std::process::exit(1);
                });

            let contents = std::fs::read_to_string(&file).unwrap_or_else(|_| {
                eprintln!("Error: Unable to read the file");
                std::process::exit(1);
            });

            ctx.run_script(&contents)
        }
        None => {
            eprintln!("Usage: {} <file> or {} eval <code>", args[0], args[0]);
            std::process::exit(1);
        }
    };

    // Display result
    println!("result: {:?}", result);

    // If args contains --fetch, trigger the fetch event
    if args.contains(&String::from("--fetch")) {
        let start = std::time::Instant::now();
        ctx.trigger_fetch_event();
        let end = std::time::Instant::now();
        println!("Time elapsed: {:?}", end - start);
    }
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
