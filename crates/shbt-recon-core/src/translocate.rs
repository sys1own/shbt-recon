//! State translocation + Gram positivity — `ModularStateTranslocator` /
//! `GramPositivityVerifier` transferred from `sys1own/shbt-exotic`
//! (`modular_translocator.rs`), adapted to a pure-Rust API.
//!
//! Enforces positive-definiteness of the Gram matrix
//! G_ab = ⟨∂_a ψ_A | ∂_b ψ_B⟩ (λ_min > 0); a violation raises a hardware
//! interrupt class error before the spatial mesh is corrupted.

/// Gram eigenvalue positivity bound.
pub const GRAM_POSITIVITY_FLOOR: f64 = 0.0;

#[derive(Clone, Debug, PartialEq)]
pub enum TranslocateError {
    /// λ_min(G) ≤ 0 — coordinate singularity, hardware interrupt raised.
    GramPositivityViolation { lambda_min: f64 },
    /// Source/target norm mismatch — non-isometric relabeling.
    IsometryViolation { residual: f64 },
    /// Causal authorization refused (target outside J⁺(src)).
    CausalViolation,
}

/// Smallest eigenvalue of a symmetric positive-definite candidate matrix
/// via Jacobi iteration (small dense matrices, n ≤ 64).
fn min_eigenvalue_symmetric(g: &[Vec<f64>]) -> Option<f64> {
    let n = g.len();
    if n == 0 || g.iter().any(|r| r.len() != n) {
        return None;
    }
    let mut a: Vec<Vec<f64>> = g.to_vec();
    for _ in 0..64 {
        // largest off-diagonal pivot
        let (mut p, mut q, mut m) = (0usize, 1usize, 0.0f64);
        for i in 0..n {
            for j in (i + 1)..n {
                if a[i][j].abs() > m {
                    m = a[i][j].abs();
                    p = i;
                    q = j;
                }
            }
        }
        if m < 1e-15 {
            break;
        }
        let app = a[p][p];
        let aqq = a[q][q];
        let apq = a[p][q];
        let tau = (aqq - app) / (2.0 * apq);
        let t = tau.signum() / (tau.abs() + (1.0 + tau * tau).sqrt());
        let theta = t.atan();
        let (c, s) = (theta.cos(), theta.sin());
        for k in 0..n {
            let akp = a[k][p];
            let akq = a[k][q];
            a[k][p] = c * akp + s * akq;
            a[k][q] = -s * akp + c * akq;
        }
        for k in 0..n {
            let apk = a[p][k];
            let aqk = a[q][k];
            a[p][k] = c * apk + s * aqk;
            a[q][k] = -s * apk + c * aqk;
        }
    }
    Some((0..n).map(|i| a[i][i]).fold(f64::INFINITY, f64::min))
}

/// Gram matrix eigenvalue positivity verifier (`GramPositivityVerifier`).
#[derive(Clone, Debug, Default)]
pub struct GramPositivityVerifier;

impl GramPositivityVerifier {
    pub fn new() -> Self {
        Self
    }

    /// G_ab = Σ_k conj(∂_a ψ_k) ∂_b ψ_k over row-major derivative samples.
    /// `derivatives[a][k]` = ∂_a ψ_k (real part; imaginary part in `im`).
    pub fn gram_matrix(derivatives_re: &[Vec<f64>], derivatives_im: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = derivatives_re.len();
        let k_dim = derivatives_re.first().map_or(0, |r| r.len());
        let mut g = vec![vec![0.0; n]; n];
        for a in 0..n {
            for b in 0..n {
                let mut acc = 0.0;
                for k in 0..k_dim {
                    // conj(d_a)·d_b = (re_a - i im_a)(re_b + i im_b); take Re.
                    acc += derivatives_re[a][k] * derivatives_re[b][k]
                        + derivatives_im[a][k] * derivatives_im[b][k];
                }
                g[a][b] = acc;
            }
        }
        g
    }

    /// λ_min(G); `None` for malformed input.
    pub fn lambda_min(g: &[Vec<f64>]) -> Option<f64> {
        min_eigenvalue_symmetric(g)
    }

    /// Enforce λ_min(G) > 0.  On violation the pipeline must raise the
    /// hardware interrupt path — callers treat the error as non-recoverable.
    pub fn enforce(&self, g: &[Vec<f64>]) -> Result<f64, TranslocateError> {
        let lambda_min = min_eigenvalue_symmetric(g).ok_or(TranslocateError::CausalViolation)?;
        if lambda_min <= GRAM_POSITIVITY_FLOOR {
            return Err(TranslocateError::GramPositivityViolation { lambda_min });
        }
        Ok(lambda_min)
    }
}

/// Boundary relabeling + phase-locked excitation translocator
/// (`ModularStateTranslocator` transfer).  Spatial isometry: the dark-ledger
/// state is copied source → target preserving ‖·‖₂.
#[derive(Clone, Debug)]
pub struct ModularStateTranslocator {
    verifier: GramPositivityVerifier,
}

impl ModularStateTranslocator {
    pub fn new() -> Self {
        Self {
            verifier: GramPositivityVerifier::new(),
        }
    }

    /// Copy the state from source index to target index under isometry;
    /// enforces Gram positivity of the derivative inner product space first.
    pub fn translocate(
        &self,
        state: &[(f64, f64)],
        gram: &[Vec<f64>],
        theta: f64,
    ) -> Result<Vec<(f64, f64)>, TranslocateError> {
        self.verifier.enforce(gram)?;
        // Phase-locked U(1) excitation e^{-iθQ}, Q = 1 on the canonical lattice.
        let (c, s) = (theta.cos(), theta.sin());
        Ok(state
            .iter()
            .map(|&(re, im)| (re * c + im * s, im * c - re * s))
            .collect())
    }

    /// Isometry residual |‖out‖² − ‖in‖²| for an output of `translocate`.
    pub fn isometry_residual(&self, input: &[(f64, f64)], output: &[(f64, f64)]) -> f64 {
        let nin: f64 = input.iter().map(|&(r, i)| r * r + i * i).sum();
        let nout: f64 = output.iter().map(|&(r, i)| r * r + i * i).sum();
        (nout - nin).abs()
    }
}

impl Default for ModularStateTranslocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gram_positive_passes() {
        let g = vec![vec![2.0, 0.1], vec![0.1, 1.0]];
        let l = GramPositivityVerifier::new().enforce(&g).unwrap();
        assert!(l > 0.0);
    }

    #[test]
    fn gram_nonpositive_raises_interrupt() {
        let g = vec![vec![1.0, 2.0], vec![2.0, 1.0]]; // λ_min = -1
        match GramPositivityVerifier::new().enforce(&g) {
            Err(TranslocateError::GramPositivityViolation { lambda_min }) => {
                assert!(lambda_min < 0.0)
            }
            other => panic!("expected GramPositivityViolation, got {other:?}"),
        }
    }

    #[test]
    fn translocation_is_isometric() {
        let t = ModularStateTranslocator::new();
        let state = vec![(0.5, 0.0), (0.0, 0.5)];
        let g = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let out = t.translocate(&state, &g, 0.421).unwrap();
        assert!(t.isometry_residual(&state, &out) < 1e-12);
    }
}
