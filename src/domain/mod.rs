//! Domain types for the Ecoblock platform: sensor data, node identity, and Tangle blocks.

pub mod sensor;
pub mod tangle_data;

pub use sensor::{SensorData, NodeId};
pub use tangle_data::TangleBlockData;
