mod auth;

use auth::{authenticate_user, initialize_database, Database};
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

async fn main_async() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (sender, _receiver) = broadcast::channel(8);

    let database = initialize_database();
    let db = Arc::new(database);

    loop {
        let (mut socket, client_address) = listener.accept().await.unwrap();
        let sender = sender.clone();
        let db = db.clone();
        let mut receiver = sender.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            let mut line = String::new();

            let mut authenticated_user = None;

            // Authentication loop
            loop {
                if authenticated_user.is_none() {
                    let (username, password) = get_credentials(&mut reader).await;
                    if let Some(user) = authenticate_user(&username, &password, &db) {
                        authenticated_user = Some(user);
                        writer.write_all(b"Welcome!\n").await.unwrap();
                    } else {
                        writer.write_all(b"Authentication failed.\n").await.unwrap();
                    }
                } else {
                    break;
                }
            }

            // Chat loop
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

async fn get_credentials<R: AsyncRead + Unpin>(reader: &mut BufReader<R>) -> (String, String) {
    let mut username = String::new();
    let mut password = String::new();

    reader.read_line(&mut username).await.unwrap();
    reader.read_line(&mut password).await.unwrap();

    (username.trim().to_string(), password.trim().to_string())
}

fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(main_async());
}
