#[derive(Debug, PartialEq)]
pub enum HandshakeState {
    Connected,
    LocalVersionSent,
    RemoteVersionReceived,
    LocalVerackSent,
    RemoteVerackReceived,
    HandshakeCompleted,
}
