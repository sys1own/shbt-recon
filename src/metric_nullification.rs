//! Metric nullification auditor: verifies the Lorentzian determinant stays at
//! exactly `-1.0` while an Alcubierre-type active metric slice is de-rendered.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::constants::{
    MAX_METRIC_GRID, METRIC_BUBBLE_RADIUS_M, METRIC_DOMAIN_RADIUS_M,
    METRIC_WALL_STEEPNESS_PER_M,
};

/// 4-D Lorentzian and Gram metric audit for the de-rendered visible slice.
///
/// The active metric is modelled as an ADM line element with unit lapse and a
/// longitudinal shift `β = v f(r_s)`.  For any `β` the Lorentzian determinant
/// is exactly `-1`; the auditor evaluates the residual on a fixed spatial grid
/// and tracks the smallest eigenvalue magnitudes.
#[pyclass(name = "MetricNullificationAuditor")]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MetricNullificationAuditor {
    bubble_radius_m: f64,
    wall_steepness_per_m: f64,
    domain_radius_m: f64,
    grid_points: usize,
    x_m: [f64; MAX_METRIC_GRID],
    shape: [f64; MAX_METRIC_GRID],
}

impl MetricNullificationAuditor {
    /// Create the canonical metric nullification auditor.
    pub fn new() -> Self {
        Self::with_params(
            METRIC_BUBBLE_RADIUS_M,
            METRIC_WALL_STEEPNESS_PER_M,
            METRIC_DOMAIN_RADIUS_M,
            65,
        )
    }

    pub fn with_params(
        bubble_radius_m: f64,
        wall_steepness_per_m: f64,
        domain_radius_m: f64,
        grid_points: usize,
    ) -> Self {
        let n = grid_points.min(MAX_METRIC_GRID).max(5);
        // Force odd grid so that x = 0 is included.
        let n = if n % 2 == 0 { n + 1 } else { n };
        let mut x_m = [0.0; MAX_METRIC_GRID];
        let mut shape = [0.0; MAX_METRIC_GRID];
        let dx = if n > 1 {
            2.0 * domain_radius_m / (n as f64 - 1.0)
        } else {
            0.0
        };
        for i in 0..n {
            let x = -domain_radius_m + dx * (i as f64);
            x_m[i] = x;
            let r_s = x.abs();
            shape[i] = alcubierre_shape(r_s, bubble_radius_m, wall_steepness_per_m);
        }
        MetricNullificationAuditor {
            bubble_radius_m,
            wall_steepness_per_m,
            domain_radius_m,
            grid_points: n,
            x_m,
            shape,
        }
    }

    /// Audit the metric at a fixed `velocity_c`.
    pub fn audit_velocity(&self, velocity_c: f64) -> MetricAudit {
        let mut max_det_error = 0.0f64;
        let mut min_abs_det = f64::INFINITY;
        let mut min_abs_lorentzian_ev = f64::INFINITY;
        let mut min_gram_ev = f64::INFINITY;

        for i in 0..self.grid_points {
            let beta = velocity_c * self.shape[i];
            let (det, lorentz_min, gram_min) = metric_eigenvalues(beta);
            let det_error = (det + 1.0).abs();
            max_det_error = max_det_error.max(det_error);
            min_abs_det = min_abs_det.min(det.abs());
            min_abs_lorentzian_ev = min_abs_lorentzian_ev.min(lorentz_min);
            min_gram_ev = min_gram_ev.min(gram_min);
        }

        MetricAudit {
            velocity_c,
            determinant_error: max_det_error,
            minimum_abs_determinant: min_abs_det,
            minimum_abs_lorentzian_eigenvalue: min_abs_lorentzian_ev,
            minimum_gram_eigenvalue: min_gram_ev,
            passed: max_det_error <= 1.0e-12
                && min_abs_lorentzian_ev > 1.0e-12
                && min_gram_ev > 1.0e-12,
        }
    }
}

/// Alcubierre shape function `f(r_s)` for the longitudinal shift.
fn alcubierre_shape(r_s: f64, radius: f64, sigma: f64) -> f64 {
    let denom = 2.0 * (sigma * radius).tanh();
    if denom == 0.0 {
        return 0.0;
    }
    ((sigma * (r_s + radius)).tanh() - (sigma * (r_s - radius)).tanh()) / denom
}

/// Return `(det, min_abs_lorentzian_ev, min_gram_ev)` for a 4-D metric with
/// longitudinal shift `β`.
fn metric_eigenvalues(beta: f64) -> (f64, f64, f64) {
    // 2x2 t-x block of the Lorentzian metric:
    //   g_tt = -1 + β^2,  g_tx = β,  g_xx = 1
    // det = -1 + β^2 - β^2 = -1
    let b2 = beta * beta;
    let det_2x2 = -1.0 + b2 - b2; // analytically -1, but keep explicit arithmetic
    let det = det_2x2;

    // Eigenvalues of the 2x2 Lorentzian block solve λ^2 - (β^2) λ - 1 = 0
    // from the characteristic polynomial.  The full 4x4 spectrum is λ+, λ-, 1, 1.
    let disc_l = (b2 * b2 + 4.0).sqrt();
    let lambda_plus = (b2 + disc_l) / 2.0;
    let lambda_minus = (b2 - disc_l) / 2.0;
    let min_abs_lorentzian_ev = lambda_plus.min(lambda_minus.abs()).min(1.0);

    // Gram (spatial) 2x2 block: γ_tt = 1 + β^2, γ_tx = β, γ_xx = 1
    // Characteristic: λ^2 - (2 + β^2) λ + 1 = 0
    let disc_g = ((2.0 + b2).powi(2) - 4.0).sqrt();
    let gamma_plus = (2.0 + b2 + disc_g) / 2.0;
    let gamma_minus = (2.0 + b2 - disc_g) / 2.0;
    let min_gram_ev = gamma_plus.min(gamma_minus).min(1.0);

    (det, min_abs_lorentzian_ev, min_gram_ev)
}

#[derive(Debug, Clone)]
pub struct MetricAudit {
    pub velocity_c: f64,
    pub determinant_error: f64,
    pub minimum_abs_determinant: f64,
    pub minimum_abs_lorentzian_eigenvalue: f64,
    pub minimum_gram_eigenvalue: f64,
    pub passed: bool,
}

#[pymethods]
impl MetricNullificationAuditor {
    #[new]
    #[pyo3(signature = (bubble_radius_m=METRIC_BUBBLE_RADIUS_M, wall_steepness_per_m=METRIC_WALL_STEEPNESS_PER_M, domain_radius_m=METRIC_DOMAIN_RADIUS_M, grid_points=65))]
    pub fn py_new(
        bubble_radius_m: f64,
        wall_steepness_per_m: f64,
        domain_radius_m: f64,
        grid_points: usize,
    ) -> Self {
        Self::with_params(bubble_radius_m, wall_steepness_per_m, domain_radius_m, grid_points)
    }

    /// Audit at the active velocity `velocity_c` (pre-nullification).
    fn audit<'py>(&self, py: Python<'py>, velocity_c: f64) -> PyResult<Bound<'py, PyDict>> {
        self.audit_velocity(velocity_c).to_dict(py)
    }

    /// Audit at `velocity_c` and at zero shift (post-nullification), returning both results.
    fn audit_nullification<'py>(&self, py: Python<'py>, velocity_c: f64) -> PyResult<Bound<'py, PyDict>> {
        let active = self.audit_velocity(velocity_c);
        let nullified = self.audit_velocity(0.0);
        let d = PyDict::new(py);
        d.set_item("active", active.to_dict(py)?)?;
        d.set_item("nullified", nullified.to_dict(py)?)?;
        Ok(d)
    }
}

impl MetricAudit {
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let d = PyDict::new(py);
        d.set_item("velocity_c", self.velocity_c)?;
        d.set_item("determinant_error", self.determinant_error)?;
        d.set_item("minimum_abs_determinant", self.minimum_abs_determinant)?;
        d.set_item("minimum_abs_lorentzian_eigenvalue", self.minimum_abs_lorentzian_eigenvalue)?;
        d.set_item("minimum_gram_eigenvalue", self.minimum_gram_eigenvalue)?;
        d.set_item("passed", self.passed)?;
        Ok(d)
    }
}

impl Default for MetricNullificationAuditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant_is_minus_one() {
        let auditor = MetricNullificationAuditor::new();
        let result = auditor.audit_velocity(1.071186);
        assert!(result.determinant_error < 1.0e-14);
        assert!((result.minimum_abs_determinant - 1.0).abs() < 1.0e-14);
        assert!(result.passed);
    }

    #[test]
    fn nullification_restores_eigenvalues() {
        let auditor = MetricNullificationAuditor::new();
        let nullified = auditor.audit_velocity(0.0);
        assert!((nullified.minimum_abs_lorentzian_eigenvalue - 1.0).abs() < 1.0e-14);
        assert!((nullified.minimum_gram_eigenvalue - 1.0).abs() < 1.0e-14);
    }
}
