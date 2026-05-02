//! Pelorus **reference gateway** — bridge between **Legacy Marine Data Ecosystem** (LMDE)
//! classical CAN segments and **Pelorus Core** (**CAN FD**).
//!
//! Normative behavior is defined in **[`09-gateway-specification.md`](https://github.com/pelorus-marine/specifications/blob/main/core/09-gateway-specification.md)**
//! (specifications repository). This crate is the **home** for the reference gateway firmware and
//! tooling as they land; [`crate::SPEC_GATEWAY_DOC_PATH`] is the stable path within that corpus.
//!
//! Integration with **`pelorus-core`** ( **`pelorus-marine/platform`** ) is expected for DCIDs,
//! correlation, and decode paths — not wired in this scaffold.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Path to the gateway specification file **within** the Pelorus **`specifications`** repository
/// checkout (repo-root relative).
pub const SPEC_GATEWAY_DOC_PATH: &str = "core/09-gateway-specification.md";

/// Placeholder for gateway bridge logic (**09**): translation, arbitration, and diagnostics hooks.
///
/// The real implementation will depend on **`pelorus-core`** and product-specific LMDE stacks.
pub mod bridge {
    /// Marker type — replace with stateful gateway runtime when implemented.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct GatewayScaffold;
}
