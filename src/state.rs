use tokio::net::TcpStream;

#[derive(PartialEq)]
pub enum State {
    NewConn,
}
