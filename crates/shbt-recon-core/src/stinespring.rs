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
pub fn derender(state: &[(f64, f64)]) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
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
}
