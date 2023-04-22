mod base;
mod exts;
mod serve;
mod utils;

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

#[actix_web::main]

async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Get arguments
    let args: Vec<String> = std::env::args().collect();
    let mut ctx: JsContext = JsContext::create_init();

    // Run script or eval code
    match args.get(1) {
        Some(arg) if arg == "eval" => match args.get(2) {
            Some(code) => println!("result: {:?}", ctx.eval(code)),
            None => {
                eprintln!("Usage: {} eval <code>", args[0]);
                std::process::exit(1);
            }
        },
        Some(arg) if arg == "serve" => match args.get(2) {
            Some(path) => {
                let script = read_script_file(path);

                ctx.eval(&script).unwrap();

                if !ctx.has_fetch_handler() {
                    eprintln!("Error: No fetch handler registered");
                    std::process::exit(1);
                }

                match crate::serve::serve(script).await {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                };
            },
            None => {
                eprintln!("Usage: {} serve <code>", args[0]);
                std::process::exit(1);
            }
        },
        Some(path) => println!("result: {:?}",  ctx.eval(&read_script_file(path))),
        None => {
            eprintln!("Usage: {} <file> or {} eval <code>", args[0], args[0]);
            std::process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {}
