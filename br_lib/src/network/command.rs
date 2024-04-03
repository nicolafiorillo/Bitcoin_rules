use std::fmt::{Display, Formatter};

use super::version::Version;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Command {
    pub bytes: [u8; 12],
}

#[derive(Debug, PartialEq)]
pub enum Commands {
    VerAck,
    Version(Version),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            &VERACK_COMMAND => "VerAck",
            &VERSION_COMMAND => "Version",
            _ => panic!("unknown_command"),
        };

        write!(f, "{:}", s)
    }
}

pub const VERACK_COMMAND: Command = Command {
    bytes: [0x76, 0x65, 0x72, 0x61, 0x63, 0x6B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
};

pub const VERSION_COMMAND: Command = Command {
    bytes: [0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00],
};
