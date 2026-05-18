# pelorus-vdr

Reference **voyage data recorder** scaffold — Tokio-side service writing ASAM MDF4 with channel names `pelorus/<dcid>` (see `pelorus_core::vdr::channel_map` on **`pelorus-core`**).

**Status:** binary boundary and channel naming only; rotation, IPC from M7, and NVMe retention are **to be implemented** (Phase 3 in [`PELORUS_IMPLEMENTATION_PLAN.md`](https://github.com/pelorus-marine/platform/blob/main/PELORUS_IMPLEMENTATION_PLAN.md)).

Workspace root: [`../`](../). Requires **[`pelorus-marine/platform`](https://github.com/pelorus-marine/platform)** checked out beside this repo for `pelorus-core` path dependencies (same layout as [`ecdis`](https://github.com/pelorus-marine/ecdis)).

## Build

```bash
cargo build -p pelorus-vdr
cargo run -p pelorus-vdr
```

## Related

- **[`pelorus-marine/platform`](https://github.com/pelorus-marine/platform)** — `pelorus-core` (`vdr` feature), `mdf4-rs`.
- **Specifications:** [`core/10-implementation.md`](https://github.com/pelorus-marine/specifications/blob/main/core/10-implementation.md) (reference implementations table).
