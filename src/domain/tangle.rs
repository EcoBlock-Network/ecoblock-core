use serde::{Serialize, Deserialize};

use crate::domain::SensorData;
use crate::traits::Signature;

/// A block of environmental data in the local Tangle DAG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleBlock {
    pub id: String,
    pub parents: Vec<String>,
    pub data: SensorData,
    pub signature: Signature,
}
