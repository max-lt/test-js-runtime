mod base;
mod exts;
mod serve;
mod utils;

use std::sync::{Arc, Mutex};

use base::EvalError;

use crate::base::JsContext;

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

#[tokio::main]
async fn main() {

    // Get arguments
    let args: Vec<String> = std::env::args().collect();

    if let Some(arg) = args.get(1) {
        let script = match args.get(2) {
            Some(path) => read_script_file(path),
            None => {
                eprintln!("Usage: {} {} <file>", args[0], arg);
                std::process::exit(1);
            }
        };

        let mut ctx: JsContext = JsContext::create_init();

        ctx.eval(&script).unwrap();

        if !ctx.has_fetch_handler() {
            eprintln!("Error: No fetch handler registered");
            std::process::exit(1);
        }

        let ctx = Arc::new(Mutex::new(ctx));

        if arg == "serve" {
            let local_set = tokio::task::LocalSet::new();
            local_set
                .run_until(async {
                    crate::serve::serve(ctx).await;
                })
                .await;
            return;
        }
    }

    let mut ctx: JsContext = JsContext::create_init();

    // Run script or eval code
    let result = match args.get(1) {
        Some(arg) if arg == "eval" => match args.get(2) {
            Some(code) => ctx.eval(code),
            None => {
                eprintln!("Usage: {} eval <code>", args[0]);
                std::process::exit(1);
            }
        },
        Some(path) => {
            let script = read_script_file(path);

            ctx.eval(&script)
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
        ctx.fetch();
    }
}

#[cfg(test)]
mod tests {}
