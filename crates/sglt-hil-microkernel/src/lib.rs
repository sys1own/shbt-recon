//! sglt-hil-microkernel: freestanding C11 shbt-os runtime wrapper for the
//! SHBT-MMIO-1 causal engine register block (56 B @ 0x70000000), SECDED
//! Hamming(72,64) ECC, AVX-512 Givens remapping, and the 2PN causal
//! interval evaluator with sub-2.50 ns quench interlock.

/// SHBT-MMIO-1 base address.
pub const MMIO_BASE: u32 = 0x7000_0000;
/// Register block size in bytes.
pub const MMIO_BLOCK_BYTES: usize = 56;
/// Quench latency bound (ns).
pub const QUENCH_BUDGET_NS: f64 = 2.50;
/// Post-quench recovery bound (ns).
pub const RECOVERY_BUDGET_NS: f64 = 120.00;

/// SHBT-MMIO-1 causal engine register block (packed, 56 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ShbtMmioBlock {
    /// 0x00 — causal lightcone authorization control/status, low word.
    pub causal_cone_lo: u32,
    /// 0x04 — upper word; bit 31 triggers 2PN evaluation.
    pub causal_cone_hi: u32,
    /// 0x08 — central mass M_☉ (f64, kg).
    pub pn2_metric_m0: f64,
    /// 0x10 — quadrupole coefficient J₂ (f64).
    pub pn2_metric_j2: f64,
    /// 0x18 — gravitomagnetic spin S_x (f32).
    pub pn2_spin_x: f32,
    /// 0x1C — gravitomagnetic spin S_y (f32).
    pub pn2_spin_y: f32,
    /// 0x20 — gravitomagnetic spin S_z (f32).
    pub pn2_spin_z: f32,
    /// 0x24 — computed Lorentz factor γ in 16.16 fixed point.
    pub target_vel_gamma: u32,
    /// 0x28 — Δs²_2PN lower 32 bits.
    pub ds2_interval_lo: u32,
    /// 0x2C — Δs²_2PN upper 32 bits (signed).
    pub ds2_interval_hi: i32,
    /// 0x30 — anomaly→quench latch timer (ns).
    pub quench_time_ns: u32,
    /// 0x34 — bit0 spacelike, bit1 quench active, bit2 spin error.
    pub anomaly_flags: u32,
}

const _: () = assert!(std::mem::size_of::<ShbtMmioBlock>() == MMIO_BLOCK_BYTES);
const _: () = assert!(std::mem::offset_of!(ShbtMmioBlock, anomaly_flags) == 0x34);

extern "C" {
    /// Initialize the register block with solar ephemeris constants.
    pub fn shbt_causal_init(mmio: *mut ShbtMmioBlock);
    /// Evaluate the 2PN causal interval; returns true when timelike.
    #[allow(clippy::too_many_arguments)]
    pub fn shbt_evaluate_2pn_causal_interval(
        mmio: *mut ShbtMmioBlock,
        dt: f64,
        dx: f64,
        dy: f64,
        dz: f64,
        vx: f64,
        vy: f64,
        vz: f64,
    ) -> bool;
}

/// True on virtualized CI where TSC-based nanosecond timing is unreliable.
pub fn virtualized_ci() -> bool {
    std::env::var_os("SGLT_CI_VIRTUAL_ENV").is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    const C: f64 = 299_792_458.0;

    #[test]
    fn register_block_layout() {
        assert_eq!(std::mem::size_of::<ShbtMmioBlock>(), 56);
        assert_eq!(std::mem::offset_of!(ShbtMmioBlock, quench_time_ns), 0x30);
    }

    #[test]
    fn causal_interval_timelike() {
        let mut mmio = ShbtMmioBlock::default();
        unsafe { shbt_causal_init(&mut mmio) };
        // 1 s separation at 100 km distance inside the solar well.
        let ok = unsafe {
            shbt_evaluate_2pn_causal_interval(&mut mmio, 1.0, 1.0e5, 0.0, 0.0, 0.45 * C, 0.0, 0.0)
        };
        assert!(ok);
        assert_eq!(mmio.anomaly_flags & 1, 0);
        assert!(mmio.target_vel_gamma > (1u32 << 16));
    }

    #[test]
    fn causal_interval_spacelike_latched() {
        let mut mmio = ShbtMmioBlock::default();
        unsafe { shbt_causal_init(&mut mmio) };
        // Superluminal apparent separation -> spacelike violation.
        let ok = unsafe {
            shbt_evaluate_2pn_causal_interval(&mut mmio, 1.0e-3, 1.0e9, 0.0, 0.0, 0.0, 0.0, 0.0)
        };
        assert!(!ok);
        assert_ne!(mmio.anomaly_flags & 1, 0);
        assert_ne!(mmio.causal_cone_hi & 0x8000_0000, 0);
    }
}
