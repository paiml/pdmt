//! Data models for PDMT operations
//!
//! This module contains all the data structures used throughout the library,
//! organized by functional area.

pub mod content;
pub mod template;

#[cfg(feature = "todo-validation")]
pub mod todo;

#[cfg(feature = "quality-proxy")]
pub mod quality;

#[cfg(feature = "mcp-tools")]
pub mod mcp;
