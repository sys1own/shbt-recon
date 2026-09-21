//! Third-order wake-tensor momentum compensation — `WakeTensorComp`
//! transferred from `sys1own/shbt-exotic` (`mass_congestion_engine.rs`).
//!
//! μ_comp(t) = −∫₀ᵗ W_ijk(t−τ) ẋ^j(τ) ẋ^k(τ) dτ with eigenvector-rigidity
//! bound |δμ| ≤ 1e-12.

/// Eigenvector rigidity bound |δμ| ≤ 1e-12.
pub const RIGIDITY_TOLERANCE: f64 = 1.0e-12;

/// Wake tensor spectral coefficients (W₁, W₂, W₃), transferred verbatim from
/// `shbt-exotic::mass_congestion_engine` (512-bit literal decimals reduced to
/// f64 — the root crate keeps full `rug::Rational` precision).
pub const WAKE_1: f64 = 1.7724538509055160;
pub const WAKE_2: f64 = 0.0342371948123984;
pub const WAKE_3: f64 = 0.0000154091152918;

/// Third-order wake tensor momentum compensator (`WakeTensorComp` transfer).
#[derive(Clone, Copy, Debug)]
pub struct WakeTensorComp {
    /// Wake coefficients [W₁, W₂, W₃].
    pub wake: [f64; 3],
}

impl WakeTensorComp {
    pub fn new() -> Self {
        Self {
            wake: [WAKE_1, WAKE_2, WAKE_3],
        }
    }

    /// W_ijk contraction with velocity pair (ẋ^j, ẋ^k) evaluated as the
    /// diagonal third-order kernel W(v) = W₁v + W₂v² + W₃v³.
    fn kernel(&self, v: f64) -> f64 {
        self.wake[0] * v + self.wake[1] * v * v + self.wake[2] * v * v * v
    }

    /// μ_comp(t) = −∫₀ᵗ W(ẋ(τ))·ẋ(τ) dτ, trapezoidal over samples
    /// `velocities[i] = ẋ(τ_i)` with uniform step `dt`.
    pub fn mu_comp(&self, velocities: &[f64], dt: f64) -> f64 {
        if velocities.len() < 2 || dt <= 0.0 {
            return 0.0;
        }
        let mut acc = 0.0;
        for w in velocities.windows(2) {
            let f0 = self.kernel(w[0]) * w[0];
            let f1 = self.kernel(w[1]) * w[1];
            acc += 0.5 * (f0 + f1) * dt;
        }
        -acc
    }

    /// Eigenvector rigidity residual |μ_comp(t₂) − μ_comp(t₁)| between
    /// successive frames; the translocation is rigid when ≤ 1e-12.
    pub fn rigidity_residual(&self, velocities_a: &[f64], velocities_b: &[f64], dt: f64) -> f64 {
        (self.mu_comp(velocities_b, dt) - self.mu_comp(velocities_a, dt)).abs()
    }

    /// True when the frame transition preserves eigenvector rigidity.
    pub fn is_rigid(&self, velocities_a: &[f64], velocities_b: &[f64], dt: f64) -> bool {
        self.rigidity_residual(velocities_a, velocities_b, dt) <= RIGIDITY_TOLERANCE
    }
}

impl Default for WakeTensorComp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_frame_is_rigid() {
        let w = WakeTensorComp::new();
        let v = vec![0.0; 16];
        assert_eq!(w.mu_comp(&v, 1e-3), 0.0);
        assert!(w.is_rigid(&v, &v, 1e-3));
    }

    #[test]
    fn compensation_opposes_wake() {
        let w = WakeTensorComp::new();
        let v: Vec<f64> = (0..64).map(|i| 1e-9 * i as f64).collect();
        let mu = w.mu_comp(&v, 1e-3);
        assert!(mu < 0.0, "compensation must oppose the wake, mu={mu}");
    }
}
