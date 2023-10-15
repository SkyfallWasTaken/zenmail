pub const GREETING: &[u8] = "220 smtp.example.org\n".as_bytes();
pub const HELO: &str = "250-smtp2.example.com ready when you are, [$hostname]\n"; // FIXME: change
pub const OK: &[u8] = "250 Ok\n".as_bytes();
pub const BYE: &str = "221 Bye\n";
