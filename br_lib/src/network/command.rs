use std::fmt::{Display, Formatter};

use super::{fee_filter::FeeFilter, ping::Ping, send_compact::SendCompact, version::Version};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Command {
    pub bytes: [u8; 12],
}

#[derive(Debug, PartialEq)]
pub enum Commands {
    VerAck,
    Version(Version),
    GetHeaders,
    SendCompact(SendCompact),
    Ping(Ping),
    FeeFilter(FeeFilter),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            VERACK_COMMAND => "VerAck",
            VERSION_COMMAND => "Version",
            GET_HEADERS_COMMAND => "GetHeaders",
            SEND_COMPACT_COMMAND => "SendCompact",
            PING_COMMAND => "Ping",
            FEE_FILTER_COMMAND => "FeeFilter",
            _ => panic!("unknown_command"),
        };

        write!(f, "{:}", s)
    }
}

// Use https://www.rapidtables.com/convert/number/ascii-to-hex.html to convert ASCII to HEX

pub const VERACK_COMMAND: Command = Command {
    bytes: [0x76, 0x65, 0x72, 0x61, 0x63, 0x6B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
};

pub const VERSION_COMMAND: Command = Command {
    bytes: [0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00],
};

pub const GET_HEADERS_COMMAND: Command = Command {
    bytes: [0x67, 0x65, 0x74, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00, 0x00],
};

pub const SEND_COMPACT_COMMAND: Command = Command {
    bytes: [0x73, 0x65, 0x6E, 0x64, 0x63, 0x6D, 0x70, 0x63, 0x74, 0x00, 0x00, 0x00],
};

pub const PING_COMMAND: Command = Command {
    bytes: [0x70, 0x69, 0x6E, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
};

pub const FEE_FILTER_COMMAND: Command = Command {
    bytes: [0x66, 0x65, 0x65, 0x66, 0x69, 0x6C, 0x74, 0x65, 0x72, 0x00, 0x00, 0x00],
};
