use lib::core::JsRuntime;
use lib::fetch::RuntimeFetchMessage;
use lib::utils::file::read_script_file;

use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use actix_web::web;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;

async fn handle_request(_req: HttpRequest) -> HttpResponse {
    let worker_id = format!("{}", actix_web::rt::System::current().id());

    println!("Worker {}", worker_id);

    let body = reqwest::get("https://httpbin.org/get") // Fetch a URL
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let response = HttpResponse::Ok()
        .append_header(("X-Worker-Id", worker_id))
        .content_type("text/plain")
        .body(body);

    println!("Response: {:?}", response);

    response
}

async fn serve() -> std::io::Result<()> {
    let server =
        HttpServer::new(move || App::new().service(web::resource("/{path}*").to(handle_request)))
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

    match serve().await {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    };
}

#[cfg(test)]
mod tests {}
