//! MCP tool integration

#[cfg(feature = "mcp-tools")]
pub mod tools;

#[cfg(feature = "mcp-tools")]
pub use tools::create_template_tool;
