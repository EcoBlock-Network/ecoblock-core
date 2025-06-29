//! Domain types for the Ecoblock platform: sensor data, node identity, and Tangle blocks.

pub mod sensor;
pub mod tangle;

pub use sensor::{SensorData, NodeId};
pub use tangle::TangleBlock;
