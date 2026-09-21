//! Causal Point observer memory — `CausalPointMemory` /
//! `CausalHistoryProjection` transferred from `sys1own/shbt-precision`
//! (`src/shbt/causal_point.rs`), adapted to a pure-Rust API.
//!
//! Lightcone authorization x_tar ∈ J⁺(x_src) and the rank-one history
//! projection operator Π_{A,ι} = |ψ_{A,ι}⟩⟨ψ_{A,ι}| with strict idempotency
//! Π² = Π.

/// A point on the boundary causal manifold (signature −+++).
pub type SpacetimePoint = [f64; 4];

/// Causal lightcone authorization (`x_tar ∈ J⁺(x_src)` check).
#[derive(Clone, Copy, Debug, Default)]
pub struct LightconeAuthorization;

impl LightconeAuthorization {
    /// True iff the target is inside or on the future cone of the source:
    /// η_μν Δx^μ Δx^ν ≤ 0 and t_tar > t_src (or coincident).
    pub fn in_future_cone(src: SpacetimePoint, tar: SpacetimePoint) -> bool {
        let dt = tar[0] - src[0];
        let ds2 = -(dt * dt)
            + (tar[1] - src[1]).powi(2)
            + (tar[2] - src[2]).powi(2)
            + (tar[3] - src[3]).powi(2);
        ds2 <= 0.0 && dt >= 0.0
    }

    /// Authorize `tar`; returns Err on violation (spacelike or past-directed).
    pub fn authorize(src: SpacetimePoint, tar: SpacetimePoint) -> Result<(), ()> {
        if Self::in_future_cone(src, tar) {
            Ok(())
        } else {
            Err(())
        }
    }
}

/// Rank-one history projection operator Π_{A,ι} = |ψ⟩⟨ψ| over a normalized
/// complex state vector, enforcing idempotency Π² = Π so that dark-ledger
/// history records are invariant under repeated evaluation.
#[derive(Clone, Debug)]
pub struct CausalHistoryProjection {
    /// |ψ_{A,ι}⟩ components (re, im).
    pub psi: Vec<(f64, f64)>,
}

impl CausalHistoryProjection {
    /// Build the projector from a (not necessarily normalized) state.
    /// Returns `None` for a zero state.
    pub fn new(psi: Vec<(f64, f64)>) -> Option<Self> {
        let norm2: f64 = psi.iter().map(|&(r, i)| r * r + i * i).sum();
        if norm2 <= 0.0 {
            return None;
        }
        let norm = norm2.sqrt();
        Some(Self {
            psi: psi.iter().map(|&(r, i)| (r / norm, i / norm)).collect(),
        })
    }

    /// Apply Π to a vector: Π|φ⟩ = |ψ⟩⟨ψ|φ⟩.
    pub fn apply(&self, phi: &[(f64, f64)]) -> Vec<(f64, f64)> {
        let mut inner = (0.0, 0.0);
        for (i, &(pr, pi)) in self.psi.iter().enumerate() {
            let (qr, qi) = phi.get(i).copied().unwrap_or((0.0, 0.0));
            // conj(ψ)·φ
            inner.0 += pr * qr + pi * qi;
            inner.1 += pr * qi - pi * qr;
        }
        self.psi
            .iter()
            .map(|&(pr, pi)| (pr * inner.0 - pi * inner.1, pr * inner.1 + pi * inner.0))
            .collect()
    }

    /// Idempotency residual ‖Π²φ − Πφ‖₂; must be ≤ tol for dark-ledger
    /// record invariance.
    pub fn idempotency_residual(&self, phi: &[(f64, f64)], tol: f64) -> bool {
        let once = self.apply(phi);
        let twice = self.apply(&once);
        let resid: f64 = once
            .iter()
            .zip(twice.iter())
            .map(|(&(ar, ai), &(br, bi))| (ar - br).powi(2) + (ai - bi).powi(2))
            .sum::<f64>()
            .sqrt();
        resid <= tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn future_cone_accepts_timelike() {
        assert!(LightconeAuthorization::in_future_cone(
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.5, 0.0, 0.0]
        ));
    }

    #[test]
    fn future_cone_rejects_spacelike_and_past() {
        assert!(!LightconeAuthorization::in_future_cone(
            [0.0, 0.0, 0.0, 0.0],
            [0.1, 5.0, 0.0, 0.0]
        ));
        assert!(!LightconeAuthorization::in_future_cone(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ));
    }

    #[test]
    fn projector_is_idempotent() {
        let p = CausalHistoryProjection::new(vec![(1.0, 0.0), (1.0, 0.0)]).unwrap();
        let phi = vec![(0.6, 0.1), (0.8, -0.2)];
        assert!(p.idempotency_residual(&phi, 1e-12));
    }
}
