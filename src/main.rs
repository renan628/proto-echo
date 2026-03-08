use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = socket.read(&mut buf).await.unwrap_or_else(|e| {
                    eprintln!("socket read error {}", e);
                    0
                });

                if n == 0 {
                    return;
                }

                if let Err(e) = socket.write_all(&buf[..n]).await {
                    eprintln!("socket write error: {}", e);
                };
            }
        });
    }
}
