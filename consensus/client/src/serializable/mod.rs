//! Serializable transaction wrapper for WASM interop

use crate::imports::*;
use crate::result::Result;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "wasm32-sdk", wasm_bindgen(typescript_custom_section))]
const TS_SERIALIZABLE_TRANSACTION: &'static str = r#"
export type SerializableTransaction = ISerializableTransactionNumeric | ISerializableTransactionString;
export type ISerializableTransaction = SerializableTransaction;
export type SerializableTransactionT = ISerializableTransaction;
"#;

#[cfg(feature = "wasm32-sdk")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Array, typescript_type = "ISerializableTransaction")]
    pub type SerializableTransactionT;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SerializableTransaction {
    Numeric(ISerializableTransactionNumeric),
    String(ISerializableTransactionString),
}

impl SerializableTransaction {
    pub fn from_client_transaction(transaction: &Transaction) -> Result<Self> {
        Ok(Self::Numeric(ISerializableTransactionNumeric::from_client_transaction(transaction)?))
    }
}

pub mod numeric;
pub mod string;

pub type ISerializableTransactionNumeric = numeric::SerializableTransaction;
pub type ISerializableTransactionString = string::SerializableTransaction;
