# pelorus-gateway

Reference **gateway** implementation scaffold — classical **LMDE** segments ↔ **Pelorus Core** (**CAN FD**) per **[`09-gateway-specification.md`](https://github.com/pelorus-marine/specifications/blob/main/core/09-gateway-specification.md)**.

**Status:** crate boundary and docs only; firmware, bridge logic, and UI are **to be implemented**.

Workspace root: [`../`](../). Companion reference: [`pelorus-gnss`](../pelorus-gnss/).

## Build

```bash
cargo build -p pelorus-gateway
```

## Related

- **[`pelorus-marine/platform`](https://github.com/pelorus-marine/platform)** — `pelorus-core` and future gateway-capable stacks.
- **Specifications index:** [`core/11-reference-implementations.md`](https://github.com/pelorus-marine/specifications/blob/main/core/11-reference-implementations.md).
