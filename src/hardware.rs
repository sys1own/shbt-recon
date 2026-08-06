//! Topological-protection auditor for 2D topological-insulator edge-state waveguides.

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Auditor that verifies spin polarization of a helical edge state remains stable
/// under non-magnetic backscattering events.
#[pyclass(name = "TopologicalProtectionAuditor")]
#[derive(Debug, Clone)]
pub struct TopologicalProtectionAuditor {
    #[pyo3(get, set)]
    pub initial_spin: f64,
    #[pyo3(get, set)]
    pub backscattering_rate: f64,
    #[pyo3(get, set)]
    pub events: u64,
}

impl TopologicalProtectionAuditor {
    /// Create the canonical topological-protection auditor.
    pub fn new() -> Self {
        Self::with_params(1.0, 1.0e-12, 0)
    }

    pub fn with_params(initial_spin: f64, backscattering_rate: f64, events: u64) -> Self {
        TopologicalProtectionAuditor {
            initial_spin,
            backscattering_rate,
            events,
        }
    }

    /// Remaining spin polarization after `events` backscattering attempts.
    ///
    /// Helical edge states suppress non-magnetic backscattering, so the
    /// polarization decays only by the small rate per event.
    pub fn spin_polarization(&self) -> f64 {
        if self.backscattering_rate <= 0.0 || self.events == 0 {
            return self.initial_spin;
        }
        let survival = 1.0 - self.backscattering_rate;
        self.initial_spin * survival.powf(self.events as f64)
    }

    /// Simulate one additional backscattering event.
    pub fn simulate_backscattering(&mut self) -> f64 {
        self.events += 1;
        self.spin_polarization()
    }

    /// True when the spin polarization is above the supplied stability threshold.
    pub fn is_stable(&self, threshold: f64) -> bool {
        self.spin_polarization() >= threshold
    }

    /// Run a protection audit.
    pub fn audit(&self) -> &'static str {
        if self.is_stable(0.99) {
            "STATUS_NOMINAL_PASS"
        } else {
            "EMERGENCY_TOPOLOGICAL_DEPOLARIZATION"
        }
    }
}

#[pymethods]
impl TopologicalProtectionAuditor {
    #[new]
    #[pyo3(signature = (initial_spin=1.0, backscattering_rate=1.0e-12, events=0))]
    pub fn py_new(initial_spin: f64, backscattering_rate: f64, events: u64) -> Self {
        Self::with_params(initial_spin, backscattering_rate, events)
    }

    /// Run the protection audit and return a status string.
    fn status(&self) -> &'static str {
        self.audit()
    }

    /// Run the protection audit and return a Python dictionary.
    pub fn audit_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let d = PyDict::new(py);
        d.set_item("spin_polarization", self.spin_polarization())?;
        d.set_item("backscattering_rate", self.backscattering_rate)?;
        d.set_item("events", self.events)?;
        d.set_item("status", self.audit())?;
        Ok(d)
    }
}

impl Default for TopologicalProtectionAuditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spin_polarization_stable_under_backscattering() {
        let mut auditor = TopologicalProtectionAuditor::new();
        // Simulate one billion backscattering attempts at 1e-12 rate.
        auditor.events = 1_000_000_000;
        let polarization = auditor.spin_polarization();
        assert!(polarization > 0.99);
        assert!(auditor.is_stable(0.99));
        assert_eq!(auditor.audit(), "STATUS_NOMINAL_PASS");
    }

    #[test]
    fn single_backscattering_event_does_not_depolarize() {
        let mut auditor = TopologicalProtectionAuditor::new();
        let pol = auditor.simulate_backscattering();
        assert!((pol - (1.0 - 1.0e-12)).abs() < 1.0e-15);
        assert_eq!(auditor.audit(), "STATUS_NOMINAL_PASS");
    }
}
