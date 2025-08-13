//! Quality enforcement integration

#[cfg(feature = "quality-proxy")]
pub mod proxy;

#[cfg(feature = "quality-proxy")]
pub use proxy::QualityProxy;
