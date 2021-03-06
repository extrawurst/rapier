//! Data structures modified with guaranteed deterministic behavior after deserialization.

pub use self::coarena::Coarena;
pub use parry::utils::MaybeSerializableData;

pub mod arena;
mod coarena;
pub(crate) mod graph;
pub mod pubsub;
