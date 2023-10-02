// Validate a transaction:
// 1. The input of the transaction are previously unspent, to avoid double-spending
// 2. The sum of the inputs is greater then or equal to the sum of the outputs. No new bitcoins are created. The difference between the sum of the inputs and the sum of the outputs goes is the transaction fee for the miner.
// 3. The ScriptSig in the input successfully unlocks the previous ScriptPubKey of the outputs.
