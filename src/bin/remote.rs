use std::{borrow::Borrow, net::Ipv4Addr};

use tokio::{
    io::{self, AsyncReadExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", 3002)).await?;

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

    socket.read(&mut buf).await?;

    let mut remote = match buf[3] {
        0x01 => {
            let ip = Ipv4Addr::new(buf[4], buf[5], buf[6], buf[7]);
            let port = u16::from_be_bytes([buf[8], buf[9]]);

            TcpStream::connect((ip, port)).await?
        }
        0x03 => {
            let domain_length = buf[4] as usize;
            let domain = &buf[5..5 + domain_length];
            let domain = String::from_utf8_lossy(domain);
            let domain = domain.borrow();
            let port = u16::from_be_bytes([buf[5 + domain_length], buf[5 + domain_length + 1]]);

            TcpStream::connect((domain, port)).await?
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Address type not supported",
            ))
        }
    };

    io::copy_bidirectional(&mut socket, &mut remote).await?;

    io::Result::Ok(())
}
