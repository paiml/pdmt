//! Quality enforcement module for PDMT with PAIML integration
//!
//! This module provides comprehensive quality enforcement capabilities
//! including PMAT quality mode proxy integration, coverage enforcement,
//! doctest validation, property test requirements, and SATD detection.

#[cfg(feature = "quality-proxy")]
pub mod proxy;

#[cfg(feature = "quality-proxy")]
pub mod enforcement;

#[cfg(feature = "quality-proxy")]
pub mod gates;

#[cfg(feature = "quality-proxy")]
pub use proxy::{QualityProxy, ProxyMode, ProxyConfig, ProxyRequest, ProxyResponse};

#[cfg(feature = "quality-proxy")]
pub use enforcement::{QualityEnforcer, EnforcementResult, EnforcementConfig};

#[cfg(feature = "quality-proxy")]
pub use gates::{QualityGate, GateResult, QualityGatePipeline};
