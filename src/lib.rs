//! SHBT Destination Reconstruction & State Decoupling Simulator.
//!
//! This crate implements the isometric Stinespring map, dark-capacity ledger,
//! causal authorization, and phase-locked reconstruction operator described in
//! the SHBT technical specification.  The Modular State Translocator wraps
//! these primitives with HIL safety, metric nullification, hardware-synthesis,
//! and thermodynamic audits.

pub mod causal_audit;
pub mod constants;
pub mod error;
pub mod hardware_synthesis;
pub mod hil_safety;
pub mod ledger;
pub mod metric_nullification;
pub mod modular_translocator;
pub mod reconstruction;
pub mod stinespring;
pub mod thermodynamics;

pub use causal_audit::*;
pub use error::*;
pub use hardware_synthesis::*;
pub use hil_safety::*;
pub use ledger::*;
pub use metric_nullification::*;
pub use modular_translocator::*;
pub use reconstruction::*;
pub use stinespring::*;
pub use thermodynamics::*;

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
    m.add_class::<metric_nullification::MetricNullificationAuditor>()?;
    m.add_class::<hardware_synthesis::HardwareSynthesisAuditor>()?;
    m.add_class::<thermodynamics::ThermodynamicCost>()?;
    m.add_class::<modular_translocator::ModularStateTranslocator>()?;
    m.add("AnomalyClosureError", m.py().get_type::<error::AnomalyClosureError>())?;
    m.add("CausalViolationError", m.py().get_type::<CausalViolationError>())?;
    Ok(())
}
