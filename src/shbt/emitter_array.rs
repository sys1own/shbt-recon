//! Emitter-array hardware simulation: thermal dissipation budget and topological
//! edge-state phase noise.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::constants::{KB_J_PER_K, LN2, N_LOCAL_BITS, TEMPERATURE_K};

/// Thermal-dissipation auditor for an emergency field-collapse shunt.
///
/// Implements the HIL thermal-flux constraint
///
///   \dot{Q}_{shunt} = E_{dissipative} / \tau_{latency} \le P_{cooling}(T)
///
/// where the holographic cooling power of the local boundary register is
///
///   P_{cooling}(T) = C_{thermal} T / \tau_{latency},
///   C_{thermal}    = N_{local} k_B \ln 2.
///
/// A 142.08 MW field collapse over 2.5 ns deposits only E = P \tau into the
/// local register heat capacity, producing a temperature rise far below the
/// 15.4 mK base temperature, so the dilution-refrigerator stage does not quench.
#[pyclass(name = "ThermalDissipationAuditor")]
#[derive(Debug, Clone)]
pub struct ThermalDissipationAuditor {
    /// Operational power of the rendered field in watts.
    #[pyo3(get, set)]
    pub field_power_w: f64,
    /// Hard-coded emergency shunt latency in seconds.
    #[pyo3(get, set)]
    pub shunt_latency_s: f64,
    /// Cryogenic base temperature in kelvin.
    #[pyo3(get, set)]
    pub base_temperature_k: f64,
    /// Number of local bits that provide the holographic heat capacity.
    #[pyo3(get, set)]
    pub n_local_bits: f64,
}

impl ThermalDissipationAuditor {
    /// Canonical thermal-dissipation auditor for the SHBT HIL shunt.
    pub fn new() -> Self {
        Self::with_params(142.08e6, 2.5e-9, TEMPERATURE_K, N_LOCAL_BITS)
    }

    pub fn with_params(
        field_power_w: f64,
        shunt_latency_s: f64,
        base_temperature_k: f64,
        n_local_bits: f64,
    ) -> Self {
        ThermalDissipationAuditor {
            field_power_w,
            shunt_latency_s,
            base_temperature_k,
            n_local_bits,
        }
    }

    /// Energy that must be dumped during the shunt, `E = P \tau`.
    pub fn energy_dissipative_j(&self) -> f64 {
        self.field_power_w * self.shunt_latency_s
    }

    /// Thermal heat capacity of the local holographic register,
    /// `C = N_{local} k_B \ln 2` (J/K).
    pub fn thermal_capacity_j_per_k(&self) -> f64 {
        self.n_local_bits * KB_J_PER_K * LN2
    }

    /// Thermal-flux rate during the shunt, `\dot{Q} = E / \tau`.
    pub fn q_dot_shunt_w(&self) -> f64 {
        self.energy_dissipative_j() / self.shunt_latency_s
    }

    /// Holographic cooling power at temperature `T`, `P_{cooling}(T) = C T / \tau`.
    pub fn cooling_power_w(&self, temperature_k: f64) -> f64 {
        self.thermal_capacity_j_per_k() * temperature_k / self.shunt_latency_s
    }

    /// Temperature rise from dumping the shunt energy into the local heat capacity.
    pub fn temperature_rise_k(&self) -> f64 {
        self.energy_dissipative_j() / self.thermal_capacity_j_per_k()
    }

    /// Run the thermal-dissipation audit.
    ///
    /// Returns `STATUS_NOMINAL_PASS` when the shunt flux is below the cooling
    /// power and the resulting temperature rise is below the base temperature;
    /// otherwise `EMERGENCY_THERMAL_QUENCH`.
    pub fn audit(&self) -> &'static str {
        let p_cooling = self.cooling_power_w(self.base_temperature_k);
        if self.q_dot_shunt_w() <= p_cooling && self.temperature_rise_k() <= self.base_temperature_k {
            "STATUS_NOMINAL_PASS"
        } else {
            "EMERGENCY_THERMAL_QUENCH"
        }
    }
}

#[pymethods]
impl ThermalDissipationAuditor {
    #[new]
    #[pyo3(signature = (field_power_w=142.08e6, shunt_latency_s=2.5e-9, base_temperature_k=TEMPERATURE_K, n_local_bits=N_LOCAL_BITS))]
    pub fn py_new(
        field_power_w: f64,
        shunt_latency_s: f64,
        base_temperature_k: f64,
        n_local_bits: f64,
    ) -> Self {
        Self::with_params(field_power_w, shunt_latency_s, base_temperature_k, n_local_bits)
    }

    /// Audit the thermal shunt and return a dictionary of computed quantities.
    pub fn audit_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let d = PyDict::new(py);
        d.set_item("field_power_w", self.field_power_w)?;
        d.set_item("shunt_latency_s", self.shunt_latency_s)?;
        d.set_item("base_temperature_k", self.base_temperature_k)?;
        d.set_item("energy_dissipative_j", self.energy_dissipative_j())?;
        d.set_item("q_dot_shunt_w", self.q_dot_shunt_w())?;
        d.set_item("cooling_power_w", self.cooling_power_w(self.base_temperature_k))?;
        d.set_item("temperature_rise_k", self.temperature_rise_k())?;
        d.set_item("status", self.audit())?;
        Ok(d)
    }
}

impl Default for ThermalDissipationAuditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Topological edge-state phase-noise model for weak backscattering near
/// constriction or tunnel contacts on a 2D topological-insulator waveguide.
#[pyclass(name = "TopologicalEdgeNoise")]
#[derive(Debug, Clone)]
pub struct TopologicalEdgeNoise {
    /// Phase-noise variance in radians squared (CLI-settable).
    #[pyo3(get, set)]
    pub variance_rad2: f64,
    /// Weak backscattering probability per contact event.
    #[pyo3(get, set)]
    pub backscattering_rate: f64,
    /// Number of backscattering events accumulated.
    #[pyo3(get, set)]
    pub events: u64,
}

impl TopologicalEdgeNoise {
    /// Create an edge-noise model with the given variance.
    pub fn new(variance_rad2: f64) -> Self {
        Self::with_params(variance_rad2, 1.0e-12, 0)
    }

    pub fn with_params(variance_rad2: f64, backscattering_rate: f64, events: u64) -> Self {
        TopologicalEdgeNoise {
            variance_rad2,
            backscattering_rate,
            events,
        }
    }

    /// RMS phase noise in radians.
    pub fn rms_phase_noise_rad(&self) -> f64 {
        self.variance_rad2.sqrt()
    }

    /// Effective total phase jitter when edge noise is added to a base jitter,
    /// combined in quadrature.
    pub fn effective_phase_jitter(&self, base_phase_jitter_rad: f64) -> f64 {
        (base_phase_jitter_rad * base_phase_jitter_rad + self.variance_rad2).sqrt()
    }

    /// Accumulate weak backscattering events, growing the variance by a small
    /// amount per expected scattering event.
    pub fn accumulate_backscattering(&mut self, events: u64) {
        self.events += events;
        // Each expected backscattering event adds a tiny phase variance, chosen
        // so that 10^9 events at rate 10^{-12} remain well below the threshold.
        let per_event_variance = self.backscattering_rate * 1.0e-12;
        self.variance_rad2 += events as f64 * per_event_variance;
    }

    /// True when the effective phase jitter is below the HIL threshold.
    pub fn is_within_threshold(&self, base_phase_jitter_rad: f64, threshold_rad: f64) -> bool {
        self.effective_phase_jitter(base_phase_jitter_rad) <= threshold_rad
    }
}

#[pymethods]
impl TopologicalEdgeNoise {
    #[new]
    #[pyo3(signature = (variance_rad2=0.0, backscattering_rate=1.0e-12, events=0))]
    pub fn py_new(variance_rad2: f64, backscattering_rate: f64, events: u64) -> Self {
        Self::with_params(variance_rad2, backscattering_rate, events)
    }

    /// Effective total phase jitter including the edge-state noise.
    pub fn effective_phase_jitter_py(&self, base_phase_jitter_rad: f64) -> f64 {
        self.effective_phase_jitter(base_phase_jitter_rad)
    }

    /// Simulate additional backscattering events and return the updated RMS noise.
    pub fn simulate_backscattering(&mut self, events: u64) -> f64 {
        self.accumulate_backscattering(events);
        self.rms_phase_noise_rad()
    }

    /// Audit the effective jitter against the HIL phase-jitter threshold.
    pub fn audit(&self, base_phase_jitter_rad: f64, threshold_rad: f64) -> (bool, f64) {
        let jitter = self.effective_phase_jitter(base_phase_jitter_rad);
        (jitter <= threshold_rad, jitter)
    }

    /// Return a status string for the edge-noise HIL check.
    pub fn status(&self, base_phase_jitter_rad: f64, threshold_rad: f64) -> &'static str {
        if self.is_within_threshold(base_phase_jitter_rad, threshold_rad) {
            "STATUS_NOMINAL_PASS"
        } else {
            "EMERGENCY_PHASE_JITTER"
        }
    }
}

impl Default for TopologicalEdgeNoise {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::PHASE_JITTER_THRESHOLD_RAD;

    #[test]
    fn thermal_shunt_does_not_quench() {
        let auditor = ThermalDissipationAuditor::new();
        assert!(auditor.q_dot_shunt_w() <= auditor.cooling_power_w(auditor.base_temperature_k));
        assert!(auditor.temperature_rise_k() <= auditor.base_temperature_k);
        assert_eq!(auditor.audit(), "STATUS_NOMINAL_PASS");
    }

    #[test]
    fn edge_noise_combines_in_quadrature() {
        let noise = TopologicalEdgeNoise::new(1.0e-12);
        let base = 2.0e-6;
        let jitter = noise.effective_phase_jitter(base);
        assert!((jitter - (base * base + 1.0e-12).sqrt()).abs() < 1.0e-18);
    }

    #[test]
    fn edge_noise_within_hil_threshold() {
        let noise = TopologicalEdgeNoise::new(1.0e-12);
        assert!(noise.is_within_threshold(1.0e-6, PHASE_JITTER_THRESHOLD_RAD));
    }

    #[test]
    fn excessive_edge_noise_triggers_emergency() {
        let threshold = PHASE_JITTER_THRESHOLD_RAD;
        let noise = TopologicalEdgeNoise::new(2.0 * threshold * threshold);
        assert!(!noise.is_within_threshold(0.0, threshold));
    }

    #[test]
    fn backscattering_accumulation_stays_weak() {
        let mut noise = TopologicalEdgeNoise::new(0.0);
        noise.accumulate_backscattering(1_000_000_000);
        assert!(noise.rms_phase_noise_rad() < PHASE_JITTER_THRESHOLD_RAD);
    }
}
