/// Release name of this build: package version plus short commit when the
/// build knew it (`0.13.1-556084e`). Shown to MCP clients and in
/// `get_installation_info`; `--version` keeps the bare package version, which
/// the newer-binary probe compares.
pub const RELEASE: &str = env!("KONNECT_RELEASE");

pub mod config_resolution;
pub mod design_hash;
pub(crate) mod freerouting_mcp;
pub mod gates;
pub mod guidance;
pub(crate) mod java_runtime;
pub mod kicad_install;
pub mod mcp;
pub(crate) mod native_specctra_bridge;
pub mod observability;
pub mod outcome;
pub mod progress;
pub mod router;
pub(crate) mod runtime_info;
pub(crate) mod specctra;
pub(crate) mod specctra_ses;
#[cfg(test)]
pub(crate) mod test_support;
pub mod tools;
