use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3001").await?;
    println!("SOCKS5 proxy listening on 127.0.0.1:3001");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                println!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];

    let n = socket.read(&mut buf).await?;
    if n < 3 || buf[0] != 0x05 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid SOCKS version",
        ));
    }

    socket.write_all(&[0x05, 0x00]).await?;

    let mut remote = TcpStream::connect("127.0.0.1:3002").await?;

    socket
        .write_all(&[0x05, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
        .await?;

    io::copy_bidirectional(&mut socket, &mut remote).await?;

    println!("Connection closed");

    io::Result::Ok(())
}
