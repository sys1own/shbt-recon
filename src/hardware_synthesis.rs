//! Hardware-synthesis auditor for phase jitter and thermal-noise limits.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::constants::{KB_J_PER_K, LN2, N_LOCAL_BITS, N_SAT_BITS, PHASE_JITTER_THRESHOLD_RAD, TEMPERATURE_K};

/// Hardware-synthesis audit for the Modular State Translocator.
///
/// Evaluates whether the projected quantum phase jitter and the per-GET
/// thermodynamic cost remain within the thermal-noise budget of the
/// boundary register.
#[pyclass(name = "HardwareSynthesisAuditor")]
#[derive(Debug, Clone)]
pub struct HardwareSynthesisAuditor {
    temperature_k: f64,
    n_local: f64,
    n_sat: f64,
    phase_jitter_threshold_rad: f64,
}

impl HardwareSynthesisAuditor {
    /// Canonical benchmark hardware-synthesis auditor.
    pub fn new() -> Self {
        Self::with_params(TEMPERATURE_K, N_LOCAL_BITS, N_SAT_BITS, PHASE_JITTER_THRESHOLD_RAD)
    }

    pub fn with_params(
        temperature_k: f64,
        n_local: f64,
        n_sat: f64,
        phase_jitter_threshold_rad: f64,
    ) -> Self {
        HardwareSynthesisAuditor {
            temperature_k,
            n_local,
            n_sat,
            phase_jitter_threshold_rad,
        }
    }

    /// Quantum projection-noise phase jitter, `Δφ = 1 / sqrt(N_local)`.
    pub fn compute_phase_jitter_rad(&self) -> f64 {
        1.0 / self.n_local.sqrt()
    }

    /// Thermal noise floor `k_B T` for the operating temperature.
    pub fn compute_thermal_noise_limit_j(&self) -> f64 {
        KB_J_PER_K * self.temperature_k
    }

    /// Per-GET energy cost `C_get = k_B T ln 2 * (N_local / N_sat)`.
    pub fn compute_c_get_j(&self) -> f64 {
        KB_J_PER_K * self.temperature_k * LN2 * (self.n_local / self.n_sat)
    }
}

#[pymethods]
impl HardwareSynthesisAuditor {
    #[new]
    #[pyo3(signature = (temperature_k=TEMPERATURE_K, n_local=N_LOCAL_BITS, n_sat=N_SAT_BITS, phase_jitter_threshold_rad=PHASE_JITTER_THRESHOLD_RAD))]
    pub fn py_new(
        temperature_k: f64,
        n_local: f64,
        n_sat: f64,
        phase_jitter_threshold_rad: f64,
    ) -> Self {
        Self::with_params(temperature_k, n_local, n_sat, phase_jitter_threshold_rad)
    }

    /// Phase jitter in radians.
    #[getter]
    pub fn phase_jitter_rad(&self) -> f64 {
        self.compute_phase_jitter_rad()
    }

    /// True if `phase_jitter_rad` is below the hardware threshold.
    pub fn phase_jitter_passes(&self) -> bool {
        self.compute_phase_jitter_rad() <= self.phase_jitter_threshold_rad
    }

    /// Thermal noise limit in joules.
    #[getter]
    pub fn thermal_noise_limit_j(&self) -> f64 {
        self.compute_thermal_noise_limit_j()
    }

    /// Per-GET energy cost in joules.
    #[getter]
    pub fn c_get_j(&self) -> f64 {
        self.compute_c_get_j()
    }

    /// True if the GET cost is below the thermal noise floor.
    pub fn thermal_noise_passes(&self) -> bool {
        self.compute_c_get_j() <= self.compute_thermal_noise_limit_j()
    }

    /// Run the full hardware-synthesis audit.
    pub fn audit<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let d = PyDict::new(py);
        d.set_item("phase_jitter_rad", self.compute_phase_jitter_rad())?;
        d.set_item("phase_jitter_threshold_rad", self.phase_jitter_threshold_rad)?;
        d.set_item("phase_jitter_passes", self.phase_jitter_passes())?;
        d.set_item("thermal_noise_limit_j", self.compute_thermal_noise_limit_j())?;
        d.set_item("c_get_j", self.compute_c_get_j())?;
        d.set_item("thermal_noise_passes", self.thermal_noise_passes())?;
        Ok(d)
    }
}

impl Default for HardwareSynthesisAuditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_jitter_below_threshold() {
        let hsa = HardwareSynthesisAuditor::new();
        assert!(hsa.compute_phase_jitter_rad() < 1.0e-30);
        assert!(hsa.phase_jitter_passes());
    }

    #[test]
    fn get_cost_below_thermal_noise() {
        let hsa = HardwareSynthesisAuditor::new();
        assert!(hsa.compute_c_get_j() < hsa.compute_thermal_noise_limit_j());
        assert!(hsa.thermal_noise_passes());
    }
}
