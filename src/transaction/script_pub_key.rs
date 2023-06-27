#[derive(Debug)]
pub struct ScriptPubKey {
    content: Vec<u8>,
}

impl ScriptPubKey {
    pub fn new(content: Vec<u8>) -> Self {
        ScriptPubKey { content: content }
    }

    pub fn serialize(&self) -> Vec<u8> {
        todo!();
    }
}
