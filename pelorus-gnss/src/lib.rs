//! First Pelorus Core reference hardware pairing.
//!
//! This crate captures the recommended GNSS RF module, antenna, and MCU
//! development board stack aligned with **Pelorus Core** from `ARCHITECTURE.md`
//! and core docs: **CAN FD** sensor plane, **Rust-first** firmware,
//! **selective wake** (GNSS is explicitly named for groups like anchor watch /
//! underway / storm in `core/04-power-management.md`), and **9–32 V**–class
//! marine supply later—not required on day-one breadboards.
//!
//! Receiver and antenna are not on CAN; the **MCU + CAN FD transceiver** tie into
//! Pelorus Core.

pub mod can_fd;

/// Ideal GNSS receiver (RF/engine module) recommendations.
pub mod gnss {
    /// Primary recommendation: u-blox ZED-F9P on a proven breakout such as
    /// **ArduSimple simpleRTK2B** or equivalent.
    pub const PRIMARY_MODULE: &str = "u-blox ZED-F9P";

    /// Typical carrier board class for the primary module.
    pub const PRIMARY_BREAKOUT_CLASS: &str = "ArduSimple simpleRTK2B-class breakout";

    /// Forward-looking alternative in the same footprint family: more bands,
    /// higher max rate, PPP-RTK / Galileo HAS–style features—better if the
    /// reference hardware should stay current through 2026+.
    pub const FORWARD_LOOKING_ALTERNATIVE: &str = "u-blox ZED-X20P";

    /// If “ideal” means hardened marine/industrial RF first and cost second,
    /// vendors such as **Septentrio** (e.g. Mosaic-class OEM modules) are noted
    /// in the reference—but they are a worse **default** for an open,
    /// low-friction first reference.
    pub const MARINE_INDUSTRIAL_RF_NOTE: &str =
        "Septentrio Mosaic-class OEM (robustness/maritime tooling vs cost/integration)";

    /// Rationale bullets for the primary GNSS choice (first Pelorus reference).
    pub const WHY_PRIMARY: &[&str] = &[
        "Multi-band + RTK: L1/L2-class observables → faster ambiguity resolution and better behavior near structure/water multipath than single-band chips—still without survey-grade pricing.",
        "Documentation and tooling: public integration manual, UBX binary protocol, u-center for bring-up—important when defining DCIDs and conformance tests.",
        "Interfaces: UART/SPI/I2C to host MCU; Pelorus uses CAN FD, so the reference stack parses UBX/NMEA on the MCU and publishes position on the bus—native GNSS-on-CAN modules are rare and usually proprietary.",
        "Ecosystem: many third-party boards; easiest path for others to replicate the reference implementation.",
    ];
}

/// Ideal antenna pairing with the GNSS module.
pub mod antenna {
    /// Match the module’s dual-band needs; usual reference pairing with ZED-F9P.
    pub const PRIMARY_CLASS: &str = "u-blox ANN-MB (or equivalent dual-band active GNSS)";

    /// Typical electrical class for bias and gain (from reference pairing).
    pub const TYPICAL_BIAS_GAIN_NOTE: &str =
        "dual-band active GNSS antenna, ~28 dB-class gain, 2.7–5.5 V bias";

    /// Fixed-mount / exposed reference rig guidance.
    pub const FIXED_MOUNT_NOTE: &str =
        "prefer marine/outdoor IP-rated antenna and short low-loss coax (often TNC/SMA pigtail); follow ground plane guidance from the antenna datasheet—critical for stable phase center for RTK.";
}

/// Ideal MCU development board and bench additions.
pub mod mcu {
    /// Example: STM32 Nucleo with dual CAN FD and mainstream Rust support.
    pub const PRIMARY_DEV_BOARD: &str = "NUCLEO-G474RE";

    /// MCU family summary from the reference (dual FDCAN, UART GNSS, timers/DMA).
    pub const PRIMARY_MCU_FAMILY: &str = "STM32G474 (dual FDCAN)";

    /// Rust ecosystem paths called out for G4-class parts.
    pub const RUST_ECOSYSTEM: &[&str] = &["stm32-rs", "embassy", "probe-rs"];

    /// What you still add on the bench beyond the Nucleo.
    pub const BENCH_ADDITIONS: &[&str] = &[
        "CAN FD transceiver (and eventually partial-networking / selective-wake–capable parts when exercising §04 power-management behavior)—Nucleo alone does not provide ISO 11898-2:2016 wake PHY without the right transceiver and schematic.",
    ];

    /// Practical bench combo for proving GNSS → Pelorus CAN FD before M12 cable plants.
    pub const PRACTICAL_COMBO: &str =
        "NUCLEO-G474RE + ZED-F9P breakout (UART to MCU) + CAN FD transceiver shield/breakout + bench termination";
}

/// One row of the reference summary table (`Role` → suggested part class).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SummaryRow {
    pub role: &'static str,
    pub suggested_part_class: &'static str,
}

/// Summary table from the first-reference pairing document.
pub const SUMMARY_TABLE: &[SummaryRow] = &[
    SummaryRow {
        role: "GNSS RF engine",
        suggested_part_class:
            "u-blox ZED-F9P (first ref); ZED-X20P if maximum longevity / bands",
    },
    SummaryRow {
        role: "GNSS carrier board",
        suggested_part_class: "simpleRTK2B-class breakout (or SparkFun u-blox board with preferred docs)",
    },
    SummaryRow {
        role: "Antenna",
        suggested_part_class: "Dual-band active (e.g. ANN-MB class), outdoor-rated for deck tests",
    },
    SummaryRow {
        role: "MCU dev board",
        suggested_part_class:
            "NUCLEO-G474RE (dual CAN FD) + separate CAN FD transceiver hardware",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_table_has_four_rows() {
        assert_eq!(SUMMARY_TABLE.len(), 4);
    }

    #[test]
    fn summary_roles_are_unique() {
        let mut roles: Vec<_> = SUMMARY_TABLE.iter().map(|r| r.role).collect();
        roles.sort_unstable();
        let deduped_len = roles.len();
        roles.dedup();
        assert_eq!(roles.len(), deduped_len);
    }
}
