#[derive(Debug, PartialEq)]
pub enum TxError {
    InvalidTransactionLength,
    Invalid4BytesLength,
    Invalid32BytesLength,
    Invalid8BytesLength,
    PartiallyReadTransaction,
    VarIntError,
    TransactionNotFoundInChain,
    ScriptError,
    InputIndexOutOfBounds,
    OutputIndexOutOfBounds,
    InvalidTransactionFee,
    ScriptVerificationFailed,
}
