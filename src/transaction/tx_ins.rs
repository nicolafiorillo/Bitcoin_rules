use std::{
    fmt::{Display, Formatter},
    ops::Index,
};

use super::{script::Script, tx_in::TxIn};

#[derive(Debug, Clone)]
pub struct TxIns(Vec<TxIn>);

impl TxIns {
    pub fn new(txs_in: Vec<TxIn>) -> Self {
        TxIns(txs_in)
    }

    pub fn len(&self) -> usize {
        let Self(inputs) = self;
        inputs.len()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let Self(inputs) = self;
        inputs.iter().flat_map(|i| i.serialize()).collect()
    }

    pub fn remove_script(&mut self) {
        let Self(inputs) = self;

        for i in 0..inputs.len() {
            self.0[i].remove_script();
        }
    }

    pub fn substitute_script(&mut self, index: usize, script_pub_key: Script) {
        let Self(inputs) = self;

        if index > inputs.len() {
            log::error!("input_index out of bounds");
            return;
        }

        inputs[index].substitute_script(script_pub_key);
    }
}

impl Index<usize> for TxIns {
    type Output = TxIn;

    fn index(&self, index: usize) -> &TxIn {
        let Self(inputs) = self;
        &inputs[index]
    }
}

impl Display for TxIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self(inputs) = self;

        for tx_in in inputs {
            // TODO: move in tx_in.rs
            write!(
                f,
                "   previous_transaction_id: {:02X}\n   previous_transaction_index: {:}\n   script_sig: {:}\n   sequence: {:}\n   witnesses: {:?}\n   network: {:}\n   --\n",
                tx_in.previous_transaction_id, tx_in.previous_transaction_index, tx_in.script_sig, tx_in.sequence, tx_in.witnesses, tx_in.network
            )?;
        }
        write!(f, "")
    }
}
