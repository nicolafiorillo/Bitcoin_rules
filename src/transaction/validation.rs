// 1. The input of the transaction are previouslu unspent.
//    no double-spending
// 2. The sum of the inputs is greater then or equal to the sum of the outputs.
//    no new bitcoins are created
// 3. The ScriptSig in the input successfully unlocks the previous ScriptPubKey of the outputs.
//    concatenated script is valid

// 1. set of UTXO should be available (mocked)
