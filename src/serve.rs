use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::sync::mpsc::RecvTimeoutError;

use actix_web::web;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;

use crate::base::JsContext;

async fn handle_request(data: Data<AppState>, req: HttpRequest) -> HttpResponse {
    let mut ctx = data.ctx.lock().unwrap();

    let event = ctx.fetch(req).unwrap();

    let worker_id = format!("{}", actix_web::rt::System::current().id());

    let response = match event.response_receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(response) => response,
        Err(RecvTimeoutError::Timeout) => {
            return HttpResponse::InternalServerError()
                .append_header(("X-Worker-Id", worker_id))
                .content_type("text/html; charset=utf-8")
                .body("Timeout")
        },
        Err(_) => {
            return HttpResponse::InternalServerError()
                .append_header(("X-Worker-Id", worker_id))
                .content_type("text/html; charset=utf-8")
                .body("Error")
        },
    };

    println!("Response: {:?}", response);

    HttpResponse::Ok()
        .append_header(("X-Worker-Id", worker_id))
        .content_type("text/html; charset=utf-8")
        .body(response)
}

struct AppState {
    ctx: Arc<Mutex<JsContext>>,
}

pub async fn serve(script: String) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let mut ctx = JsContext::create_init();
        ctx.eval(script.as_str()).unwrap();
        let ctx = Arc::new(Mutex::new(ctx));

        App::new()
            .app_data(Data::new(AppState { ctx }))
            .service(web::resource("/{path}*").to(handle_request))
    })
    .bind(("127.0.0.1", 3000))?
    .run();

    server.await
}
