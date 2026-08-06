//! Thermodynamic cost of information retrieval for the Modular State Translocator.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use rug::Float;
use crate::constants::{KB_J_PER_K, LN2, N_LOCAL_BITS, N_SAT_BITS, PREC, TEMPERATURE_K};

/// Energy-to-information retrieval cost, `C_get = k_B T ln 2 * (N_local / N_sat)`.
#[pyclass(name = "ThermodynamicCost")]
#[derive(Debug, Clone)]
pub struct ThermodynamicCost {
    n_local: f64,
    n_sat: f64,
    temperature_k: f64,
    c_get_j: f64,
    landauer_limit_j: f64,
    ratio: f64,
}

impl ThermodynamicCost {
    /// Create with the canonical benchmark values.
    pub fn new() -> Self {
        Self::with_params(TEMPERATURE_K, N_LOCAL_BITS, N_SAT_BITS)
    }

    /// Create with explicit thermodynamic parameters.
    pub fn with_params(temperature_k: f64, n_local: f64, n_sat: f64) -> Self {
        let mut n_local_f = Float::with_val(PREC, n_local);
        let n_sat_f = Float::with_val(PREC, n_sat);
        n_local_f /= &n_sat_f;
        let ratio = n_local_f.to_f64();

        let mut landauer = Float::with_val(PREC, KB_J_PER_K * temperature_k);
        landauer *= Float::with_val(PREC, LN2);
        let landauer_limit_j = landauer.to_f64();

        let mut c_get = Float::with_val(PREC, &landauer);
        c_get *= &n_local_f;
        let c_get_j = c_get.to_f64();

        ThermodynamicCost {
            n_local,
            n_sat,
            temperature_k,
            c_get_j,
            landauer_limit_j,
            ratio,
        }
    }
}

#[pymethods]
impl ThermodynamicCost {
    #[new]
    pub fn py_new() -> Self {
        Self::new()
    }

    /// Local register size `N_local` (bits).
    #[getter]
    pub fn n_local_bits(&self) -> f64 {
        self.n_local
    }

    /// Saturated horizon register size `N_sat` (bits).
    #[getter]
    pub fn n_sat_bits(&self) -> f64 {
        self.n_sat
    }

    /// Operating temperature (K).
    #[getter]
    pub fn temperature_k(&self) -> f64 {
        self.temperature_k
    }

    /// Ratio `N_local / N_sat`.
    #[getter]
    pub fn ratio(&self) -> f64 {
        self.ratio
    }

    /// Landauer limit `k_B T ln 2` (J).
    #[getter]
    pub fn landauer_limit_j(&self) -> f64 {
        self.landauer_limit_j
    }

    /// Retrieval cost per GET operation `C_get` (J).
    #[getter]
    pub fn c_get_j(&self) -> f64 {
        self.c_get_j
    }

    /// Return all thermodynamic quantities as a Python dictionary.
    pub fn audit<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let d = PyDict::new(py);
        d.set_item("n_local_bits", self.n_local)?;
        d.set_item("n_sat_bits", self.n_sat)?;
        d.set_item("temperature_k", self.temperature_k)?;
        d.set_item("ratio", self.ratio)?;
        d.set_item("landauer_limit_j", self.landauer_limit_j)?;
        // Report C_get at 12 significant figures to match the macro target.
        d.set_item("c_get_j", format!("{:.11e}", self.c_get_j))?;
        Ok(d)
    }
}

impl Default for ThermodynamicCost {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_get_is_positive_and_tiny() {
        let tc = ThermodynamicCost::new();
        assert!(tc.c_get_j > 0.0);
        assert!(tc.c_get_j < tc.landauer_limit_j);
        assert!(tc.c_get_j < 1.0e-70);
    }

    #[test]
    fn ratio_matches_benchmark() {
        let tc = ThermodynamicCost::new();
        let expected = 1.20e72 / 3.31e122;
        assert!((tc.ratio - expected).abs() < 1.0e-55);
    }
}
