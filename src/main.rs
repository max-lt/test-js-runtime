mod base;
mod base64_utils;
mod console;
mod event_listener;
mod inspect;
mod utils;
mod v8_ext;

use crate::base::JsContext;

fn main() {
    let mut ctx = JsContext::create_init();

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

}
