use futures::channel::oneshot;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {}

enum Command {
    Increment,
    Get(oneshot::Sender<i32>),
}

async fn start() -> mpsc::Sender<Command> {
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
