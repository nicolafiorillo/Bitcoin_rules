#[derive(Debug)]
pub enum TxError {
    InvalidTransactionLength,
    Invalid4BytesLength,
    Invalid32BytesLength,
    Invalid8BytesLength,
    PartiallyReadTransaction,
    VarIntError,
}
