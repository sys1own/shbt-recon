//! SHBT Destination Reconstruction & State Decoupling Simulator.
//!
//! This crate implements the isometric Stinespring map, dark-capacity ledger,
//! causal authorization, and phase-locked reconstruction operator described in
//! the SHBT Phase 1 specification.

pub mod causal_audit;
pub mod constants;
pub mod error;
pub mod hil_safety;
pub mod ledger;
pub mod reconstruction;
pub mod stinespring;

pub use causal_audit::*;
pub use error::*;
pub use hil_safety::*;
pub use ledger::*;
pub use reconstruction::*;
pub use stinespring::*;

use pyo3::prelude::*;

pyo3::create_exception!(shbt_recon, CausalViolationError, pyo3::exceptions::PyValueError);

#[pymodule(name = "_core")]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<causal_audit::CausalCoordinate>()?;
    m.add_class::<ledger::DarkLedger>()?;
    m.add_class::<stinespring::DerenderingEngine>()?;
    m.add_class::<reconstruction::ReconstructionOperator>()?;
    m.add_class::<reconstruction::BoundaryRelabeling>()?;
    m.add_class::<reconstruction::PhaseLockedExcitation>()?;
    m.add_class::<hil_safety::HilSafetyMonitor>()?;
    m.add("AnomalyClosureError", m.py().get_type::<error::AnomalyClosureError>())?;
    m.add("CausalViolationError", m.py().get_type::<CausalViolationError>())?;
    Ok(())
}
