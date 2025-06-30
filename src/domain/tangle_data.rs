use serde::{Deserialize, Serialize};
use crate::domain::SensorData;
use crate::traits::Signable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleBlockData {
    pub parents: Vec<String>,
    pub data: SensorData,
}

impl Signable for TangleBlockData {
    fn payload(&self) -> Vec<u8> {
        serde_json::to_vec(&(&self.parents, &self.data)).unwrap_or_default()
    }
}
