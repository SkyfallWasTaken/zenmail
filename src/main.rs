use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use color_eyre::Result;

const ADDR: &str = "0.0.0.0:587";

mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let listener = TcpListener::bind(ADDR).await?;
    println!("Listening on {ADDR}");

    loop {
        let (mut socket, client_addr) = listener.accept().await?;
        dbg!("new client", client_addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Print the data
                let command = String::from_utf8_lossy(&buf).to_string();
                dbg!(parser::Message::from_smtp_message(command).unwrap()); //FIXME: fix

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}