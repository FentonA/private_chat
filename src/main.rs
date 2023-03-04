mod auth;

use auth::{handle_request, UserDatabase};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (sender, _receiver) = broadcast::channel(8);

    loop {
        let (mut socket, client_address) = listener.accept().await.unwrap();

        let sender = sender.clone();

        let mut receiver = sender.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            let mut line = String::new();

            loop {
                tokio::select! {
                  result = reader.read_line(&mut line) => {
                    if result.unwrap() == 0 {
                      break;
                    }

                    sender.send((line.clone(), client_address)).unwrap();

                    line.clear();
                  }
                  result = receiver.recv() => {
                    let (msg, sender_address) = result.unwrap();

                    if sender_address != client_address {
                      writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                  }
                }
            }
        });
    }
}
