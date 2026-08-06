//! Error types and Python exception bindings for shbt-recon.

use pyo3::{exceptions::PyException, PyErr};
use pyo3::exceptions::{PyRuntimeError, PyValueError};

#[derive(Debug)]
pub enum ReconError {
    AnomalyClosureError(String),
    CausalViolationError(String),
    PrecisionLossError(String),
}

impl std::fmt::Display for ReconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReconError::AnomalyClosureError(msg) => write!(f, "AnomalyClosureError: {}", msg),
            ReconError::CausalViolationError(msg) => write!(f, "CausalViolationError: {}", msg),
            ReconError::PrecisionLossError(msg) => write!(f, "PrecisionLossError: {}", msg),
        }
    }
}

impl std::error::Error for ReconError {}

pyo3::create_exception!(shbt_recon, AnomalyClosureError, PyException);

impl From<ReconError> for PyErr {
    fn from(err: ReconError) -> PyErr {
        match err {
            ReconError::AnomalyClosureError(msg) => PyErr::new::<AnomalyClosureError, _>(msg),
            ReconError::CausalViolationError(msg) => PyValueError::new_err(msg),
            ReconError::PrecisionLossError(msg) => PyRuntimeError::new_err(msg),
        }
    }
}
