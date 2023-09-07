use lib::core::JsRuntime;
fn main() {
    std::env::set_var("RUST_LOG", "debug");

    JsRuntime::create_snapshot();

    println!("Done");
}

#[cfg(test)]
mod tests {}
