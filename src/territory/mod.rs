// traits
/// Public interface for customs territory-specific logic.
///
/// This trait defines the core behavior that every customs territory module
/// (e.g., China Mainland, European Union, United States) must implement.
/// It allows the crate to handle territory-specific HS code validation,
/// commodity description lookup, and tax rate queries in a uniform way.

pub trait TerritoryPublicMethod {}
// traits end
#[cfg(feature = "cn-mainland")]
pub mod cn_mainland;
