fn read_script_file(ctx: &mut JsContext, path: &str) -> Result<String, EvalError> {
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

    ctx.eval(&contents)
}
