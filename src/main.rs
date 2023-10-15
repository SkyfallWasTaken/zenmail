use eyre::eyre;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

use color_eyre::Result;

const ADDR: &str = "0.0.0.0:587";

mod messages;
mod parser;
mod state;

use state::State;

use crate::parser::Command;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let listener = TcpListener::bind(ADDR).await?;
    info!("Listening on {ADDR}");

    loop {
        let (mut socket, client_addr) = listener.accept().await?;
        info!("New client: {}", client_addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            let mut state = State::NewConn;

            socket.write_all(messages::GREETING).await.unwrap();

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        error!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Print the data
                let raw_command = String::from_utf8_lossy(&buf).to_string();
                let cmd = Command::from_smtp_message(raw_command)
                    .ok_or_else(|| eyre!("command not recognized"))?;

                debug!("Command: {:?}", &cmd);

                match cmd {
                    Command::Helo if state == State::NewConn => socket
                        .write_all(
                            messages::HELO
                                .replace("$hostname", client_addr.ip().to_string().as_str()) // TODO: remove the weird template system
                                .as_bytes(),
                        )
                        .await
                        .unwrap(),
                    _ => todo!(),
                }

                Ok(())
            }
        });
    }
}
