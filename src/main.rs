mod core;
mod utils;

use crate::core::JsRuntime;

fn read_script_file(path: &str) -> String {
    let file = std::path::Path::new(path)
        .canonicalize()
        .unwrap_or_else(|_| {
            eprintln!("Error: Invalid file path");
            std::process::exit(1);
        });

    let contents: String = std::fs::read_to_string(&file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read the file");
        std::process::exit(1);
    });

    contents
}

#[actix_web::main]

async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Get arguments
    let args: Vec<String> = std::env::args().collect();

    let mut rt: JsRuntime = JsRuntime::create();

    // Run script or eval code
    match args.get(1) {
        Some(arg) if arg == "eval" => match args.get(2) {
            Some(script) => {
                println!("{}", rt.eval(script).unwrap());
                rt.run_event_loop().await;
            }
            None => {
                eprintln!("Usage: {} eval <code>", args[0]);
                std::process::exit(1);
            }
        },
        Some(path) => {
            let script = &read_script_file(path);
            rt.eval(script).unwrap();
            rt.run_event_loop().await;
        }
        None => {
            eprintln!("Usage: {} <file> or {} eval <code>", args[0], args[0]);
            std::process::exit(1);
        }
    };

    for arg in args.iter().skip(2) {
        // event=<eventName>
        if arg.starts_with("--event=") {
            let event_type: String = arg.trim_start_matches("--event=").to_string();

            println!("Triggering event: {}", event_type);

            let event_type: String = arg.trim_start_matches("--event=").to_string();

            let event = crate::core::JsEvent::new(event_type);

            // rt.dispatch_event(event).unwrap();
            match rt.dispatch_event(event) {
                Some(result) => {
                    println!("Result: {:?}", result);
                }
                None => {
                    println!("No result");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {}
