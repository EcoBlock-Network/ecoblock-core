use serde::{Serialize, Deserialize};

/// Abstracted signature structure. To be implemented in `ecoblock-crypto`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature(pub String);

/// A trait for objects that can be signed.
/// Implementors should define how to extract the byte payload.
pub trait Signable {
    fn payload(&self) -> Vec<u8>;
}
