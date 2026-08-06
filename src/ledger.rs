//! Dark capacity ledger with exact 512-bit rational fractions.

use pyo3::prelude::*;
use rug::{Float, Rational};
use crate::constants::{COMPLETED_FRAC, PREC, RESIDUAL_FRAC};

/// Exact dark-capacity partitioning for the Stinespring map.
#[pyclass(name = "DarkLedger")]
#[derive(Debug, Clone)]
pub struct DarkLedger {
    residual: Rational,
    completed: Rational,
    /// Stinespring amplitude ratio η_D = completed fraction.
    amplitude: Rational,
}

#[pymethods]
impl DarkLedger {
    /// Initialize the ledger to the exact (10/33, 23/33) benchmark fractions.
    #[new]
    pub fn new() -> Self {
        let residual = Rational::from(RESIDUAL_FRAC);
        let completed = Rational::from(COMPLETED_FRAC);
        let amplitude = completed.clone();
        DarkLedger {
            residual,
            completed,
            amplitude,
        }
    }

    /// Residual capacity c_dark^res = 10/33.
    fn residual_fraction(&self) -> f64 {
        Float::with_val(PREC, &self.residual).to_f64()
    }

    /// Completed capacity c_dark^comp = 23/33.
    fn completed_fraction(&self) -> f64 {
        Float::with_val(PREC, &self.completed).to_f64()
    }

    /// Isometric Stinespring amplitude η_D = 23/33.
    fn stinespring_amplitude(&self) -> f64 {
        Float::with_val(PREC, &self.amplitude).to_f64()
    }

    /// 512-bit decimal string of the residual fraction.
    fn residual_float512(&self) -> String {
        Float::with_val(PREC, &self.residual).to_string()
    }

    /// 512-bit decimal string of the completed fraction.
    fn completed_float512(&self) -> String {
        Float::with_val(PREC, &self.completed).to_string()
    }

    /// 512-bit decimal string of the Stinespring amplitude.
    fn amplitude_float512(&self) -> String {
        Float::with_val(PREC, &self.amplitude).to_string()
    }
}

impl Default for DarkLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl DarkLedger {
    pub fn residual_rational(&self) -> &Rational {
        &self.residual
    }

    pub fn completed_rational(&self) -> &Rational {
        &self.completed
    }

    pub fn amplitude_rational(&self) -> &Rational {
        &self.amplitude
    }

    pub fn amplitude_float(&self) -> Float {
        Float::with_val(PREC, &self.amplitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_fractions_match_report() {
        let ledger = DarkLedger::new();
        let tol = 1.0e-15;
        assert!((ledger.residual_fraction() - 10.0 / 33.0).abs() < tol);
        assert!((ledger.completed_fraction() - 23.0 / 33.0).abs() < tol);
        assert!((ledger.stinespring_amplitude() - 23.0 / 33.0).abs() < tol);
    }
}
