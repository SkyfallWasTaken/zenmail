#[derive(Debug)]
pub enum Message {
    Helo,
    Rset,
    Quit,
    NoOp,
    Vrfy,
}

// TODO: is this the right approach?
impl Message {
    pub fn from_smtp_message(message: String) -> Option<Message> {
        let mut message = message.split_whitespace();
        let binding = message.next()?.to_uppercase(); //TODO: change var name
        let cmd = binding.as_str();

        match cmd {
            "HELO" => Some(Self::Helo),
            "RSET" => Some(Self::Rset),
            "QUIT" => Some(Self::Quit),
            "NOOP" => Some(Self::NoOp),
            "VRFY" => Some(Self::Vrfy),
            _ => None,
        }
    }
}
