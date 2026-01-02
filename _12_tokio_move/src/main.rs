use std::time::Instant;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(1);

    tokio::spawn(async move {
        for i in 1..=10000 {
            sender.send(i).await.unwrap();
        }
    });

    while let Some(i) = receiver.recv().await {
        println!("{}", i);
    }
}
