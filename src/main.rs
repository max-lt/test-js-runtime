mod base;
mod exts;
mod utils;

use crate::base::JsContext;

fn main() {
    let mut ctx = JsContext::create_init();

    // Get arguments
    let args: Vec<String> = std::env::args().collect();

    // Run script or eval code
    let result = match args.get(1) {
        Some(arg) if arg == "eval" => match args.get(2) {
            Some(code) => ctx.eval(code),
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

            ctx.eval(&contents)
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
mod tests {

}
