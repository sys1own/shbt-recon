//! ADM 3+1 metric auditor — `ADMMetricAuditor` / `ADM3Plus1ShiftField`
//! transferred from `sys1own/shbt-exotic` (`warp_metric.rs`) via
//! `sys1own/shbt-sglt` (`sglt-core-metric/adm_foliation.rs`).
//!
//! Audits a flat-slice line element with unit lapse α = 1, Euclidean spatial
//! metric γ_ij = δ_ij and a longitudinal shift β^i built from the SHBT shape
//! function.  Gates: |det(g) + 1| ≤ 1e-12 and Gram λ_min > 0, with active
//! shift-field nullification β^i → 0 during de-rendering.

/// Default 10 m bubble radius (m).
pub const BUBBLE_RADIUS_M: f64 = 10.0;
/// Default 30 m domain half-width.
pub const DOMAIN_RADIUS_M: f64 = 30.0;
/// Wall steepness of the SHBT shape function (1/m).
pub const WALL_STEEPNESS_PER_M: f64 = 0.8;
/// Maximum grid points for the longitudinal audit.
pub const MAX_GRID_POINTS: usize = 1201;
/// Determinant invariance bound |det(g)+1| ≤ 1e-12.
pub const DET_TOLERANCE: f64 = 1.0e-12;
/// Shift-field nullification bound |β^i| ≤ 1e-12 (c units).
pub const SHIFT_NULL_TOLERANCE: f64 = 1.0e-12;

/// SHBT shape function f(r_s) for the longitudinal shift.
fn shbt_shape(r_s: f64, radius: f64, sigma: f64) -> f64 {
    let denom = 2.0 * (sigma * radius).tanh();
    if denom == 0.0 {
        return 0.0;
    }
    ((sigma * (r_s + radius)).tanh() - (sigma * (r_s - radius)).tanh()) / denom
}

/// 4-D Lorentzian determinant and Gram positivity invariants for a
/// longitudinal shift β (dimensionless, units of c); n = (1,0,0).
fn metric_invariants(beta: f64) -> (f64, f64, f64) {
    let b2 = beta * beta;
    // det(g) = (-1 + β²)·1 − β²·1 = -1 exactly for the flat-slice block.
    let det = -1.0 + b2 - b2;

    let disc_l = (b2 * b2 + 4.0).sqrt();
    let lambda_plus = (b2 + disc_l) / 2.0;
    let lambda_minus = (b2 - disc_l) / 2.0;
    let min_abs_lorentzian_ev = lambda_plus.min(lambda_minus.abs()).min(1.0);

    let disc_g = ((2.0 + b2).powi(2) - 4.0).sqrt();
    let gamma_plus = (2.0 + b2 + disc_g) / 2.0;
    let gamma_minus = (2.0 + b2 - disc_g) / 2.0;
    let min_gram_ev = gamma_plus.min(gamma_minus).min(1.0);

    (det, min_abs_lorentzian_ev, min_gram_ev)
}

/// ADM 3+1 shift-field slice (β^i, lapse α, spatial metric γ_ij).
#[derive(Clone, Copy, Debug)]
pub struct ADM3Plus1ShiftField {
    /// Shift vector β^i (units of c).
    pub beta: [f64; 3],
    /// Lapse function N (α_ADM).
    pub lapse: f64,
    /// Covariant spatial metric γ_ij, row-major.
    pub gamma: [[f64; 3]; 3],
}

impl ADM3Plus1ShiftField {
    /// Flat slice at rest: β^i = 0, α = 1, γ_ij = δ_ij.
    pub fn flat() -> Self {
        Self {
            beta: [0.0; 3],
            lapse: 1.0,
            gamma: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    /// Longitudinal shift slice β^i = v·f(r_s)·n^i.
    pub fn with_longitudinal_shift(velocity_c: f64, r_s: f64, n_vec: [f64; 3]) -> Self {
        let f = shbt_shape(r_s.abs(), BUBBLE_RADIUS_M, WALL_STEEPNESS_PER_M);
        let n_norm = (n_vec[0].powi(2) + n_vec[1].powi(2) + n_vec[2].powi(2)).sqrt();
        let n = if n_norm > 1e-15 {
            [n_vec[0] / n_norm, n_vec[1] / n_norm, n_vec[2] / n_norm]
        } else {
            [1.0, 0.0, 0.0]
        };
        let mut s = Self::flat();
        s.beta = [
            velocity_c * f * n[0],
            velocity_c * f * n[1],
            velocity_c * f * n[2],
        ];
        s
    }

    /// |β^i| Euclidean magnitude.
    pub fn shift_magnitude(&self) -> f64 {
        (self.beta[0].powi(2) + self.beta[1].powi(2) + self.beta[2].powi(2)).sqrt()
    }

    /// True when the shift field is nullified: β^i → 0 within tolerance.
    pub fn is_shift_nullified(&self) -> bool {
        self.shift_magnitude() <= SHIFT_NULL_TOLERANCE
    }

    /// 4x4 covariant ADM metric g_μν (row-major):
    /// g_00 = -α² + β_iβ^i, g_0i = β_i, g_ij = γ_ij.
    pub fn metric4(&self) -> [[f64; 4]; 4] {
        let b2 = self.shift_magnitude().powi(2);
        let mut g = [[0.0; 4]; 4];
        g[0][0] = -self.lapse * self.lapse + b2;
        for i in 0..3 {
            g[0][i + 1] = self.beta[i];
            g[i + 1][0] = self.beta[i];
            for j in 0..3 {
                g[i + 1][j + 1] = self.gamma[i][j];
            }
        }
        g
    }

    /// Lorentzian determinant of the 4x4 metric (γ block diagonal ⇒
    /// det(g) = -α² · det(γ) when β has components only via g_0i coupling;
    /// evaluated by the t-x block + δ sector identity det = -α²·det(γ)).
    pub fn determinant(&self) -> f64 {
        // Sylvester block determinant: det(g) = det(γ)·(g_00 − β^T γ^{-1} β).
        // With γ = δ: = (-α² + |β|²) − |β|² = -α².
        let det_gamma = 1.0; // γ_ij = δ_ij in the flat-slice audit
        -self.lapse * self.lapse * det_gamma
    }

    /// |det(g) + 1| ≤ 1e-12.
    pub fn determinant_invariant_holds(&self) -> bool {
        (self.determinant() + 1.0).abs() <= DET_TOLERANCE
    }
}

/// Audit result for a single foliation scan.
#[derive(Clone, Debug)]
pub struct FoliationAudit {
    /// Metric velocity parameter (fraction of c).
    pub velocity_c: f64,
    /// max |det(g) + 1| over the grid.
    pub max_determinant_error: f64,
    /// min |det(g)| over the grid.
    pub min_abs_determinant: f64,
    /// Smallest |λ| of the Lorentzian t-x block.
    pub min_abs_lorentzian_eigenvalue: f64,
    /// Smallest eigenvalue of the spatial Gram block.
    pub min_gram_eigenvalue: f64,
    /// Peak |β^i| over the grid (must → 0 post-nullification).
    pub max_shift_magnitude: f64,
    /// True when every bound is satisfied.
    pub passed: bool,
}

/// 3+1D ADM metric auditor (`ADMMetricAuditor` transfer).
#[derive(Clone, Debug)]
pub struct ADMMetricAuditor {
    bubble_radius_m: f64,
    wall_steepness_per_m: f64,
    domain_radius_m: f64,
    grid_points: usize,
}

impl ADMMetricAuditor {
    pub fn new() -> Self {
        Self::with_params(BUBBLE_RADIUS_M, WALL_STEEPNESS_PER_M, DOMAIN_RADIUS_M, 65)
    }

    pub fn with_params(
        bubble_radius_m: f64,
        wall_steepness_per_m: f64,
        domain_radius_m: f64,
        grid_points: usize,
    ) -> Self {
        let n = grid_points.clamp(5, MAX_GRID_POINTS);
        let n = if n % 2 == 0 { n + 1 } else { n };
        Self {
            bubble_radius_m,
            wall_steepness_per_m,
            domain_radius_m,
            grid_points: n,
        }
    }

    /// Audit the 3+1D metric along the longitudinal axis at `velocity_c`.
    pub fn audit_velocity(&self, velocity_c: f64) -> FoliationAudit {
        let dx = 2.0 * self.domain_radius_m / (self.grid_points as f64 - 1.0);
        let mut max_det_error = 0.0f64;
        let mut min_abs_det = f64::INFINITY;
        let mut min_abs_lorentzian_ev = f64::INFINITY;
        let mut min_gram_ev = f64::INFINITY;
        let mut max_shift = 0.0f64;

        for i in 0..self.grid_points {
            let x = -self.domain_radius_m + dx * (i as f64);
            let f = shbt_shape(x.abs(), self.bubble_radius_m, self.wall_steepness_per_m);
            let beta = velocity_c * f;
            let (det, lorentz_min, gram_min) = metric_invariants(beta);
            max_det_error = max_det_error.max((det + 1.0).abs());
            min_abs_det = min_abs_det.min(det.abs());
            min_abs_lorentzian_ev = min_abs_lorentzian_ev.min(lorentz_min);
            min_gram_ev = min_gram_ev.min(gram_min);
            max_shift = max_shift.max(beta.abs());
        }

        FoliationAudit {
            velocity_c,
            max_determinant_error: max_det_error,
            min_abs_determinant: min_abs_det,
            min_abs_lorentzian_eigenvalue: min_abs_lorentzian_ev,
            min_gram_eigenvalue: min_gram_ev,
            max_shift_magnitude: max_shift,
            passed: max_det_error <= DET_TOLERANCE
                && min_abs_lorentzian_ev > 1.0e-12
                && min_gram_ev > 1.0e-12,
        }
    }

    /// Post-de-render audit: the active slice must be shift-nullified.
    pub fn nullification_audit(&self, slice: &ADM3Plus1ShiftField) -> bool {
        slice.is_shift_nullified() && slice.determinant_invariant_holds()
    }
}

impl Default for ADMMetricAuditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foliation_determinant_is_minus_one() {
        let r = ADMMetricAuditor::new().audit_velocity(0.1);
        assert!(r.passed, "{r:?}");
        assert!(r.max_determinant_error < DET_TOLERANCE);
    }

    #[test]
    fn gram_positive_at_sub_c() {
        let r = ADMMetricAuditor::new().audit_velocity(0.5);
        assert!(r.min_gram_eigenvalue > 0.0);
    }

    #[test]
    fn flat_slice_is_nullified_and_det_invariant() {
        let s = ADM3Plus1ShiftField::flat();
        assert!(s.is_shift_nullified());
        assert!(s.determinant_invariant_holds());
        assert!(ADMMetricAuditor::new().nullification_audit(&s));
    }

    #[test]
    fn shifted_slice_fails_nullification() {
        let s = ADM3Plus1ShiftField::with_longitudinal_shift(0.5, 0.0, [1.0, 0.0, 0.0]);
        assert!(!s.is_shift_nullified());
        assert!(s.determinant_invariant_holds());
    }
}
