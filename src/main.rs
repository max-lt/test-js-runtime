mod base;
mod exts;
mod server;
mod utils;

use crate::base::JsRuntime;
use crate::exts::event::EventListener;
use crate::exts::fetch::Fetch;
use crate::server::serve;

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

    let mut ctx: JsRuntime = JsRuntime::create_init();

    // Run script or eval code
    match args.get(1) {
        Some(arg) if arg == "eval" => match args.get(2) {
            Some(script) => {
                ctx.eval(script).unwrap();
            }
            None => {
                eprintln!("Usage: {} eval <code>", args[0]);
                std::process::exit(1);
            }
        },
        Some(arg) if arg == "serve" => match args.get(2) {
            Some(path) => {
                let mut ctx: JsRuntime = JsRuntime::create_init();

                let script = read_script_file(path);

                ctx.eval(&script).unwrap();

                if !ctx.has_fetch_handler() {
                    eprintln!("Error: No fetch handler registered");
                    std::process::exit(1);
                }

                match serve(script).await {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                };
            }
            None => {
                eprintln!("Usage: {} serve <code>", args[0]);
                std::process::exit(1);
            }
        },
        Some(path) => {
            let script = &read_script_file(path);
            ctx.eval(script).unwrap();
        }
        None => {
            eprintln!("Usage: {} <file> or {} eval <code>", args[0], args[0]);
            std::process::exit(1);
        }
    };

    if let Some(arg) = args.get(2) {
        if arg == "--test" {
            ctx.dispatch_event("test").unwrap();
        }
    }
}

#[cfg(test)]
mod tests {}
