//! Hardware-in-the-Loop safety monitor for real-time reconstruction audits.

use pyo3::prelude::*;

/// Real-time safety monitor with configurable Gram-eigenvalue and
/// eigenvector-rigidity thresholds.
#[pyclass(name = "HilSafetyMonitor")]
pub struct HilSafetyMonitor {
    #[pyo3(get, set)]
    pub min_gram_threshold: f64,
    #[pyo3(get, set)]
    pub detuning_tolerance: f64,
}

#[pymethods]
impl HilSafetyMonitor {
    /// Create a new HIL monitor.
    #[new]
    #[pyo3(signature = (min_gram_threshold=0.350000, detuning_tolerance=1.0e-12))]
    pub fn new(min_gram_threshold: f64, detuning_tolerance: f64) -> Self {
        HilSafetyMonitor {
            min_gram_threshold,
            detuning_tolerance,
        }
    }

    /// Audit a single HIL step.
    ///
    /// Inputs:
    /// - `min_gram_eig`: smallest Gram-matrix eigenvalue observed.
    /// - `max_det_err`: largest Lorentzian determinant residual observed.
    /// - `eigenvector_rigidity_detuning`: eigenvector-rigidity detuning measured
    ///   during the current de-render/re-render loop.
    /// - `max_info_density`: maximum local information density.
    /// - `budget_limit`: operational upper bound for `max_info_density`.
    ///
    /// Returns `"STATUS_NOMINAL_PASS"` when all checks are within bounds;
    /// otherwise returns an emergency trigger identifier.
    #[pyo3(signature = (min_gram_eig, max_det_err, eigenvector_rigidity_detuning, max_info_density, budget_limit))]
    fn audit_hil_step(
        &self,
        min_gram_eig: f64,
        max_det_err: f64,
        eigenvector_rigidity_detuning: f64,
        max_info_density: f64,
        budget_limit: f64,
    ) -> String {
        if eigenvector_rigidity_detuning > self.detuning_tolerance {
            return "EMERGENCY_ANOMALY_CLOSURE".to_string();
        }
        const DET_TOL: f64 = 1.0e-12;
        if max_det_err > DET_TOL {
            return "EMERGENCY_DETERMINANT_VIOLATION".to_string();
        }
        if min_gram_eig < self.min_gram_threshold {
            return "EMERGENCY_GRAM_EIGENVALUE".to_string();
        }
        if max_info_density > budget_limit {
            return "EMERGENCY_INFORMATION_DENSITY".to_string();
        }
        "STATUS_NOMINAL_PASS".to_string()
    }

    /// Standalone eigenvector-rigidity check used by the de-render loop.
    #[pyo3(signature = (detuning))]
    fn check_anomaly_closure(&self, detuning: f64) -> String {
        if detuning > self.detuning_tolerance {
            "EMERGENCY_ANOMALY_CLOSURE".to_string()
        } else {
            "STATUS_NOMINAL_PASS".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominal_step_passes() {
        let monitor = HilSafetyMonitor::new(0.35, 1.0e-12);
        assert_eq!(
            monitor.audit_hil_step(0.5, 1.0e-13, 1.0e-13, 1.0e60, 1.0e70),
            "STATUS_NOMINAL_PASS"
        );
    }

    #[test]
    fn anomaly_closure_triggered() {
        let monitor = HilSafetyMonitor::new(0.35, 1.0e-12);
        assert_eq!(
            monitor.audit_hil_step(0.5, 1.0e-13, 1.0e-11, 1.0e60, 1.0e70),
            "EMERGENCY_ANOMALY_CLOSURE"
        );
    }

    #[test]
    fn determinant_violation_triggered() {
        let monitor = HilSafetyMonitor::new(0.35, 1.0e-12);
        assert_eq!(
            monitor.audit_hil_step(0.5, 1.0e-11, 1.0e-13, 1.0e60, 1.0e70),
            "EMERGENCY_DETERMINANT_VIOLATION"
        );
    }
}
