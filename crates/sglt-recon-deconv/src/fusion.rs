//! Fibonacci fusion tree: τ ⊗ τ = 1 ⊕ τ. Each level of the binary
//! fusion tree decomposes the left-descendant Hilbert space, with basis
//! transitions governed by the F-matrix F_τττ^τ and the R-symbols.

use crate::PHI;

/// F-matrix element F_τττ^τ = [[φ⁻¹, φ⁻¹ᐟ²], [φ⁻¹ᐟ², −φ⁻¹]].
pub fn f_matrix() -> [[f64; 2]; 2] {
    let inv = 1.0 / PHI;
    let sqinv = PHI.sqrt().recip();
    [[inv, sqinv], [sqinv, -inv]]
}

/// R-symbol eigenphases R_ττ^1 and R_ττ^τ (radians).
pub fn r_phases() -> (f64, f64) {
    // exp(4πi/5), exp(−3πi/5)
    (
        4.0 * std::f64::consts::PI / 5.0,
        -3.0 * std::f64::consts::PI / 5.0,
    )
}

/// Unitary residual ‖F F† − I‖_F (identically ~0 up to rounding).
pub fn f_unitarity_residual() -> f64 {
    let f = f_matrix();
    let mut resid = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            let mut acc = 0.0;
            for k in 0..2 {
                acc += f[i][k] * f[j][k];
            }
            resid += (acc - (i == j) as u8 as f64).abs();
        }
    }
    resid
}

/// Byte-stream compression ratio of the hierarchical fusion tree:
/// each level encodes log₂ φ bits per anyon.
pub fn compression_ratio() -> f64 {
    PHI.log2().recip()
}

/// Dark-ledger capacity partition η_D = 23/33 ≈ 0.69697.
pub fn eta_dark() -> f64 {
    crate::ETA_D_NUM as f64 / crate::CAPACITY_DENOM as f64
}

/// Depth of the 124-braid fusion tree.
pub fn tree_depth() -> u32 {
    (crate::BRAID_COUNT as f64).log2().ceil() as u32
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::TOTAL_DIM;

    #[test]
    fn quantum_dimensions() {
        assert!((PHI - 1.61803398875).abs() < 1e-10);
        assert!((TOTAL_DIM - 1.90211303259).abs() < 1e-10);
        assert!((TOTAL_DIM * TOTAL_DIM - (2.0 + PHI)).abs() < 1e-12);
    }

    #[test]
    fn f_matrix_is_unitary() {
        assert!(f_unitarity_residual() < 1e-12);
    }

    #[test]
    fn compression_within_bounds() {
        let c = compression_ratio();
        assert!(c > 0.5 && c < 1.5, "c = {c}");
        assert!((eta_dark() - 0.69697).abs() < 1e-4);
    }
}
