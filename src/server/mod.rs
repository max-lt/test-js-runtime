use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use actix_web::web;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;

mod response;

use crate::base::JsRuntime;

async fn handle_request(data: Data<AppState>, req: HttpRequest) -> HttpResponse {
    let worker_id = format!("{}", actix_web::rt::System::current().id());
    let mut ctx = data.ctx.lock().unwrap();

    println!("Worker {} will emit fetch event", worker_id);

    let event = match ctx.fetch(req) {
        Some(event) => event,
        None => {
            return HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body("Cannot create event, check your response type");
        }
    };

    println!("Worker {} waiting for resp", worker_id);

    let timeout = tokio::time::timeout(Duration::from_millis(10), event.receiver);

    // Did we timeout or did we receive a response?
    let response = match timeout.await {
        Ok(rcv) => rcv,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .append_header(("X-Worker-Id", worker_id))
                .content_type("text/html; charset=utf-8")
                .body("Timeout");
        }
    };

    let response = match response {
        Ok(response) => response.into(),
        Err(_) => HttpResponse::InternalServerError()
            .append_header(("X-Worker-Id", worker_id))
            .content_type("text/html; charset=utf-8")
            .body("Error"),
    };

    println!("Response: {:?}", response);

    response
}

struct AppState {
    ctx: Arc<Mutex<JsRuntime>>,
}

pub async fn serve(script: String) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let mut ctx = JsRuntime::create_init();
        ctx.eval(script.as_str()).unwrap();
        let ctx = Arc::new(Mutex::new(ctx));

        App::new()
            .app_data(Data::new(AppState { ctx }))
            .service(web::resource("/{path}*").to(handle_request))
    })
    .workers(2) // Set number of workers to 2 to reduce logging noise
    .bind(("127.0.0.1", 3000))?
    .run();

    server.await
}
