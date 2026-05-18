use std::sync::Arc;

use serde::Serialize;
use tokio::net::TcpListener;

use axum::{Extension, Router, routing::get};

#[tokio::main]
async fn main() {
    let actor = actor::start().await;

    let message = Arc::new("hello".to_string());
    let app = Router::new()
        .route("/", get(|| async { "hello" }))
        .route("/json", get(hello_json))
        .layer(Extension(message))
        .layer(Extension(actor));

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn hello_json(Extension(message): Extension<Arc<String>>) -> axum::Json<HelloJson> {
    axum::Json(HelloJson {
        message: message.to_string(),
    })
}

mod actor {
    use tokio::sync::{mpsc, oneshot};

    pub enum Command {
        Increment,
        Get(oneshot::Sender<i32>),
    }

    pub async fn start() -> mpsc::Sender<Command> {
        let (tx, mut rx) = mpsc::channel(2);
        let mut counter = 0;

        tokio::spawn(async move {
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::Increment => counter += 1,
                    Command::Get(sender) => sender.send(counter).unwrap(),
                }
            }
        });

        tx
    }

    async fn increment(tx: mpsc::Sender<Command>) {
        tx.send(Command::Increment).await.unwrap();
    }

    async fn get(tx: mpsc::Sender<Command>) -> i32 {
        let (s, r) = oneshot::channel();
        tx.send(Command::Get(s)).await.unwrap();
        r.await.unwrap()
    }
}
