use std::env;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3000".to_owned());

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {addr}");

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (stream_reader, mut stream_writer) = socket.split();
            let mut message = String::new();
            let mut reader = BufReader::new(stream_reader);

            loop {
                let n = reader.read_line(&mut message).await.unwrap();
                if n == 0 {
                    break;
                }
                stream_writer.write_all(message.as_bytes()).await.unwrap();
                message.clear();
            }
        });
    }
}
