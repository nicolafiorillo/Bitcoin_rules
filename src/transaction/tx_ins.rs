use std::{
    fmt::{Display, Formatter},
    ops::Index,
};

use super::{script_pub_key::ScriptPubKey, tx_in::TxIn};

#[derive(Debug, Clone)]
pub struct TxIns(Vec<TxIn>);

impl TxIns {
    pub fn new(txs_in: Vec<TxIn>) -> Self {
        TxIns(txs_in)
    }

    pub fn amount(&self) -> u64 {
        self.0.iter().fold(0u64, |acc, i: &TxIn| acc + i.amount())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.0.iter().flat_map(|i| i.serialize()).collect()
    }

    pub fn retreive_amount(&mut self) {
        for i in 0..self.0.len() {
            self.0[i].retreive_amount();
        }
    }

    pub fn remove_script(&mut self) {
        for i in 0..self.0.len() {
            self.0[i].remove_script();
        }
    }

    pub fn substitute_script(&mut self, input_index: usize, script_pub_key: ScriptPubKey) {
        self.0[input_index].substitute_script(script_pub_key);
    }
}

impl Index<usize> for TxIns {
    type Output = TxIn;

    fn index(&self, index: usize) -> &TxIn {
        &self.0[index]
        // TODO: exit with Result<TxIn, TxError>
    }
}

impl Display for TxIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for tx_in in &self.0 {
            write!(
                f,
                "   previous_transaction_id: {:02X}\n   previous_transaction_index: {:}\n   previous_transaction_script_pubkey: {:?}\n   script_sig: {:}   sequence: {:}\n   network: {:}   amount: {:?}\n   --\n",
                tx_in.previous_transaction_id, tx_in.previous_transaction_index, tx_in.previous_transaction_script_pubkey, tx_in.script_sig, tx_in.sequence, tx_in.network, tx_in.amount
            )?;
        }
        write!(f, "")
    }
}
