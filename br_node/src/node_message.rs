#[derive(Debug, Clone, PartialEq)]
pub enum NodeMessage {
    NodeReady,
    GetHeadersRequest,
}
