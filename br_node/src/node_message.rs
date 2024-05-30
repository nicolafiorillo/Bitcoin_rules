use brl::hashing::hash256::Hash256;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeMessage {
    NodeReady(u8),
    GetHeadersRequest(Hash256),
}
