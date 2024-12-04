mod response;
mod request;
mod config;

use std::io::{Error, ErrorKind};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info};
use crate::response::{Status, ContentType};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut debug = false;

    let mut subscriber = tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_names(true)
        .event_format(tracing_subscriber::fmt::format().compact());

    if debug {
        subscriber = subscriber.with_max_level(tracing::Level::DEBUG);
    }

    subscriber.init();

    info!("Starting server...");
    info!("Biding on http://127.0.0.1:8080");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                debug!("Handling connection with {:?}", addr);

                tokio::spawn(async move {
                    handle(socket).await;
                });
            }

            Err(e) => {
                error!("{}", e);
            }
        }
    }
}

async fn handle(mut socket: TcpStream) {
    let mut buffer = BufReader::new(&mut socket);

    match buffer.fill_buf().await {
        Ok(buf) => {
            match request::parse(buf) {
                Ok(request) => {
                    let (content_type, specifier) = response::get_content_type(&request.path);

                    match response::create_resp(Status::Success, content_type, specifier, &request.path) {
                        Ok((resp, data)) => {
                            socket.write_all(resp.as_bytes()).await.unwrap();
                            socket.write_all(data.as_slice()).await.unwrap();
                            socket.flush().await.unwrap();
                        },

                        Err(e) => {
                            match e.downcast_ref::<Error>() {
                                Some(e) if e.kind() == ErrorKind::NotFound => {
                                    match response::create_resp(Status::NotFound, ContentType::Text, "html", "404.html") {
                                        Ok((resp, _data)) => {
                                            socket.write_all(resp.as_bytes()).await.unwrap();
                                            socket.flush().await.unwrap();
                                        }

                                        Err(_e) => {
                                            let resp = response::create_basic_html_resp(Status::NotFound,
                                                                             r#"
                                                                             <!DOCTYPE html>
                                                                            <html lang="en">
                                                                            <head>
                                                                                <meta charset="UTF-8">
                                                                                <title>Not Found</title>
                                                                            </head>
                                                                            <body>
                                                                              <h1>
                                                                                404 Not Found
                                                                              </h1>
                                                                              <h2>
                                                                                Hosted on BoringWebServer 0.1.0
                                                                              <h2>
                                                                            </body>
                                                                            </html>
                                                                                         "#
                                            );

                                            socket.write_all(resp.as_bytes()).await.unwrap();
                                            socket.flush().await.unwrap();
                                        }
                                    }
                                }

                                _ => error!("Unable to create respons for request {:?} -> {}", request, e)
                            }
                        },
                    }
                }

                Err(e) => {
                    error!("An error occurred while parsing the request: {}", e)
                }
            }
        },

        Err(e) => error!("An error occurred: {}", e),
    }

}