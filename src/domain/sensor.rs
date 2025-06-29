use serde::{Serialize, Deserialize};

/// Represents environmental sensor measurements collected at a given moment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensorData {
    pub pm25: f32,
    pub co2: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub timestamp: u64,
}

/// Unique identifier for a node in the Ecoblock mesh.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);
