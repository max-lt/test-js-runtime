use tokio::time::Duration;

use lib::core::JsRuntime;
use lib::core::RuntimeBasicMessage;
use lib::fetch::JsRequest;
use lib::fetch::RuntimeFetchMessage;
use lib::utils::file::read_script_file;

async fn run(args: Vec<String>) {
    let snapshot = match std::fs::read("snapshot.bin") {
        Ok(snapshot) => Some(snapshot),
        Err(_) => None,
    };

    let mut rt: JsRuntime = JsRuntime::create_init(snapshot);

    let start = std::time::SystemTime::now();

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

            match event_type.as_str() {
                "fetch" => {
                    let mut event = RuntimeFetchMessage::new(JsRequest {
                        url: "https://example.org/get".to_string(),
                        method: "GET".to_string(),
                        // headers: vec![],
                        // body: None,
                    });

                    rt.send_message(&mut event);

                    let time = std::time::SystemTime::now();
                    rt.run_event_loop().await;
                    println!("Time EvL: {:?}", time.elapsed().unwrap());

                    let time = std::time::SystemTime::now();
                    let res = event.get_response().await;
                    println!("Time RES: {:?}", time.elapsed().unwrap());

                    match res {
                        Some(res) => {
                            println!("Response: {:?}", res);
                        }
                        None => {
                            println!("Cannot get response");
                        }
                    }
                }
                _ => {
                    let mut event = RuntimeBasicMessage::new(event_type);

                    rt.send_message(&mut event);

                    rt.run_event_loop().await;
                }
            };
        }
    }

    println!("Time: {:?}", start.elapsed().unwrap());
}

#[tokio::main]

async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Get arguments
    let args: Vec<String> = std::env::args().collect();

    let timeout = tokio::time::timeout(Duration::from_millis(1000), run(args));

    // Did we timeout ?
    match timeout.await {
        Ok(_) => {
            println!("Done");
        }
        Err(_) => {
            println!("Timeout");
        }
    };
}

#[cfg(test)]
mod tests {}
