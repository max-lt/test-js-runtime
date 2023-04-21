use std::sync::Arc;
use std::sync::Mutex;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

use crate::base::JsContext;

async fn handle_connection(ctx: Arc<Mutex<JsContext>>, mut stream: tokio::net::TcpStream) {
  let mut buffer = [0; 1024];

  if let Ok(_read) = stream.read(&mut buffer).await {
      let result;
      {
          let mut context = ctx.lock().unwrap();
          result = context.fetch().unwrap_or("Error during fetch".to_owned());
      }

      let response = "HTTP/1.1 200 OK\r\n\r\n".to_owned() + &result + "\n";

      if let Err(e) = stream.write_all(response.as_bytes()).await {
          eprintln!("Failed to write to socket: {:?}", e);
      }
  }
}

pub async fn serve(ctx: Arc<Mutex<JsContext>>) {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Server running on http://127.0.0.1:3000");

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
              let ctx = ctx.clone();

                tokio::task::spawn_local(async move {
                    handle_connection(ctx, stream).await;
                });
            }
            Err(e) => eprintln!("Accept error: {:?}", e),
        }
    }
}
