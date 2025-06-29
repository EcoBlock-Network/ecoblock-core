//! Ecoblock Core – Domain definitions and shared traits for the Ecoblock platform.

pub mod domain;
pub mod traits;

pub use domain::{SensorData, NodeId, TangleBlock};
pub use traits::{Signable, Signature};
