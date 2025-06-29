use serde::{Serialize, Deserialize};

use crate::domain::SensorData;
use crate::traits::{Signature, Signable};


/// A block of environmental data in the local Tangle DAG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleBlock {
    pub id: String,
    pub parents: Vec<String>,
    pub data: SensorData,
    pub signature: Signature,
}


impl TangleBlock {
    pub fn new_unsigned(parents: Vec<String>, data: SensorData) -> Self {
        let temp = TangleBlock {
            id: String::new(),
            parents,
            data,
            signature: Signature("".into()),
        };

        let id = blake3::hash(&temp.payload()).to_hex().to_string();

        Self {
            id,
            ..temp
        }
    }
}

impl Signable for TangleBlock {
    fn payload(&self) -> Vec<u8> {
        serde_json::to_vec(&(
            &self.parents,
            &self.data,
        ))
        .unwrap_or_default()
    }
}