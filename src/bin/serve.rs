use lib::core::JsRuntime;
use lib::fetch::RuntimeFetchMessage;
use lib::utils::file::read_script_file;

use std::time::Duration;

use actix_web::web;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;

struct AppState {
    script: String,
}

async fn handle_request(data: Data<AppState>, req: HttpRequest) -> HttpResponse {
    let worker_id = format!("{}", actix_web::rt::System::current().id());

    let script = data.script.clone();

    let mut rt = JsRuntime::create_init();

    rt.eval(script.as_str()).unwrap();

    println!("Worker {} will emit fetch event", worker_id);

    let mut fetch = RuntimeFetchMessage::new(req.into());

    match rt.send_message(&mut fetch) {
        Some(_) => {}
        None => {
            return HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body("Cannot create event, check your response type");
        }
    };

    println!("Worker {} waiting for resp", worker_id);

    // Poll timers
    let time = std::time::SystemTime::now();
    rt.run_event_loop().await;
    println!("Time EvL: {:?}", time.elapsed().unwrap());

    let timeout = tokio::time::timeout(Duration::from_millis(1000), fetch.get_response());

    // Did we timeout or did we receive a response?
    let time = std::time::SystemTime::now();
    let response = match timeout.await {
        Ok(rcv) => rcv,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .append_header(("X-Worker-Id", worker_id))
                .content_type("text/html; charset=utf-8")
                .body("Timeout");
        }
    };
    println!("Time RES: {:?}", time.elapsed().unwrap());

    let response = match response {
        Some(response) => response.into(),
        None => HttpResponse::InternalServerError()
            .append_header(("X-Worker-Id", worker_id))
            .content_type("text/html; charset=utf-8")
            .body("Error"),
    };

    println!("Response: {:?}", response);

    response
}

async fn serve(script: String) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let script = script.clone();

        App::new()
            .app_data(Data::new(AppState { script }))
            .service(web::resource("/{path}*").to(handle_request))
    })
    .workers(1) // Set number of workers to 1 to reduce logging noise
    .bind(("127.0.0.1", 3000))?
    .run();

    println!("Server running at http://localhost:3000");

    server.await
}

#[actix_web::main]

async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Get arguments
    let args: Vec<String> = std::env::args().collect();

    // Run script or eval code
    match args.get(1) {
        Some(path) => {
            let script = read_script_file(path);

            match serve(script).await {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        None => {
            eprintln!("Usage: {} <file> or {} eval <code>", args[0], args[0]);
            std::process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {}
