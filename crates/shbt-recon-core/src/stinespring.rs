//! Stinespring dilation — exact 10/33 active · 23/33 dark capacity split,
//! transferred from the canonical SHBT (26, 8, 312) boundary kernel.
//!
//! D^derender = η_D U with η_D = 23/33 (completed dark capacity) and
//! residual c_dark^res = 10/33.  U is an isometry: U†U = I_vis.

/// Completed dark capacity η_D = 23/33.
pub const DARK_COMPLETED_NUM: u64 = 23;
/// Residual dark capacity c_res = 10/33.
pub const DARK_RESIDUAL_NUM: u64 = 10;
/// Common denominator of the capacity partition.
pub const CAPACITY_DENOM: u64 = 33;
/// Dark ledger dimension (boundary character block width).
pub const DARK_LEDGER_DIM: usize = 8;

/// UnifiedStinespringFrame total allocation in SRAM (bytes).
pub const UNIFIED_STINESPRING_FRAME_BYTES: usize = 2112;
/// Visible active operational window (bytes): 2112 * 10 / 33 = 640 B.
pub const VISIBLE_ACTIVE_WINDOW_BYTES: usize = 640;
/// Dark completion ledger allocation (bytes): 2112 * 23 / 33 = 1472 B.
pub const DARK_LEDGER_BYTES: usize = 1472;
/// 124 Fibonacci anyon braid channel log space (bytes): 124 * 8 = 992 B.
pub const DARK_BRAID_LATTICE_BYTES: usize = 992;
/// SECDED Hamming(72,64) syndrome metadata allocation (bytes): 1472 - 992 = 480 B.
pub const SECDED_METADATA_BYTES: usize = 480;
/// Fibonacci braid capacity C_braid = 100 / φ² ≈ 38.19660112501052.
pub const FIBONACCI_BRAID_CAPACITY: f64 = 38.19660112501052;

/// Completed fraction η_D = 23/33.
pub fn completed_fraction() -> f64 {
    DARK_COMPLETED_NUM as f64 / CAPACITY_DENOM as f64
}

/// Residual fraction c_res = 10/33.
pub fn residual_fraction() -> f64 {
    DARK_RESIDUAL_NUM as f64 / CAPACITY_DENOM as f64
}

/// Exact integer partition check: 10 + 23 = 33.
pub const fn partition_is_exact() -> bool {
    DARK_COMPLETED_NUM + DARK_RESIDUAL_NUM == CAPACITY_DENOM
}

/// De-render a visible local state |C_loc⟩ = Σ r_i|i⟩ into the coupled
/// visible⊗dark frame: active amplitudes scaled by √(10/33) (gauge-charge
/// zeroed slice) and dark amplitudes by √(23/33).  Returns (active, dark).
/// Complex amplitude pair (re, im) on the boundary character basis.
pub type Amplitude = (f64, f64);

pub fn derender(state: &[Amplitude]) -> (Vec<Amplitude>, Vec<Amplitude>) {
    let a = residual_fraction().sqrt();
    let d = completed_fraction().sqrt();
    let active = state.iter().map(|&(r, i)| (r * a, i * a)).collect();
    let dark = state.iter().map(|&(r, i)| (r * d, i * d)).collect();
    (active, dark)
}

/// Total trace conservation residual: ‖active‖² + ‖dark‖² − ‖state‖².
pub fn trace_residual(state: &[(f64, f64)], active: &[(f64, f64)], dark: &[(f64, f64)]) -> f64 {
    let n = |v: &[(f64, f64)]| v.iter().map(|&(r, i)| r * r + i * i).sum::<f64>();
    n(active) + n(dark) - n(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_exact() {
        assert!(partition_is_exact());
        assert!((completed_fraction() - 23.0 / 33.0).abs() < 1e-15);
        assert!((residual_fraction() - 10.0 / 33.0).abs() < 1e-15);
    }

    #[test]
    fn derender_preserves_total_trace() {
        let s: Vec<(f64, f64)> = (0..DARK_LEDGER_DIM)
            .map(|i| ((i + 1) as f64 / 8.0, 0.0))
            .collect();
        let (a, d) = derender(&s);
        assert!(trace_residual(&s, &a, &d).abs() < 1e-12);
    }

    #[test]
    fn frame_partition_exact() {
        assert_eq!(VISIBLE_ACTIVE_WINDOW_BYTES + DARK_LEDGER_BYTES, UNIFIED_STINESPRING_FRAME_BYTES);
        assert_eq!(DARK_BRAID_LATTICE_BYTES + SECDED_METADATA_BYTES, DARK_LEDGER_BYTES);
        assert_eq!(DARK_BRAID_LATTICE_BYTES / 124, 8);
    }
}
