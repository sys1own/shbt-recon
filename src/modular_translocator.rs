//! Modular State Translocator: production-grade orchestrator for the SHBT
//! de-render / re-render pipeline with integrated HIL, metric, and
//! thermodynamic audits.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::causal_audit::{CausalCoordinate, verify_future_cone_fatal};
use crate::constants::{EIGENVECTOR_DETUNING_TOLERANCE, HOLOGRAPHIC_NOISE_FLOOR};
use crate::error::ReconError;
use crate::hardware_synthesis::HardwareSynthesisAuditor;
use crate::hil_safety::HilSafetyMonitor;
use crate::ledger::DarkLedger;
use crate::metric_nullification::MetricNullificationAuditor;
use crate::reconstruction::reconstruct_state;
use crate::stinespring::DerenderingEngine;
use crate::thermodynamics::ThermodynamicCost;
use rug::Float;

/// Production-grade high-precision translocator.
#[pyclass(name = "ModularStateTranslocator")]
#[derive(Debug, Clone)]
pub struct ModularStateTranslocator {
    engine: DerenderingEngine,
    hil: HilSafetyMonitor,
    metric: MetricNullificationAuditor,
    hardware: HardwareSynthesisAuditor,
    thermo: ThermodynamicCost,
}

impl ModularStateTranslocator {
    /// Create with the canonical (26,8,312) benchmark kernel.
    pub fn new() -> Result<Self, ReconError> {
        let engine = DerenderingEngine::with_kernel(crate::constants::BENCHMARK_KERNEL)?;
        let hil = HilSafetyMonitor::new(0.35, EIGENVECTOR_DETUNING_TOLERANCE);
        let metric = MetricNullificationAuditor::new();
        let hardware = HardwareSynthesisAuditor::new();
        let thermo = ThermodynamicCost::new();
        Ok(ModularStateTranslocator {
            engine,
            hil,
            metric,
            hardware,
            thermo,
        })
    }

    /// Compute eigenvector-rigidity detuning of a residual state at 512-bit precision.
    fn eigen_detuning(residual_state: &[f64]) -> Result<f64, ReconError> {
        if residual_state.len() != crate::constants::DARK_LEDGER_DIM {
            return Err(ReconError::AnomalyClosureError(format!(
                "residual_state must have length {}",
                crate::constants::DARK_LEDGER_DIM
            )));
        }
        let mut norm_sq = Float::with_val(crate::constants::PREC, 0);
        for &v in residual_state.iter() {
            let f = Float::with_val(crate::constants::PREC, v);
            let mut f2 = Float::with_val(crate::constants::PREC, &f);
            f2.square_mut();
            norm_sq += f2;
        }
        let one = Float::with_val(crate::constants::PREC, 1);
        let mut diff = Float::with_val(crate::constants::PREC, &norm_sq);
        diff -= &one;
        Ok(diff.abs().to_f64())
    }
}

#[pymethods]
impl ModularStateTranslocator {
    #[new]
    pub fn py_new() -> PyResult<Self> {
        Self::new().map_err(PyErr::from)
    }

    /// Run the full translocation pipeline:
    ///
    /// 1. HIL pre-check of eigenvector rigidity.
    /// 2. Active metric nullification audit.
    /// 3. Stinespring de-rendering.
    /// 4. Nullified metric audit.
    /// 5. Causal authorization.
    /// 6. Phase-locked boundary relabeling and re-rendering.
    #[pyo3(signature = (residual_state, src, tar, theta, source_index=0, target_index=1, active_velocity_c=23.0/33.0))]
    pub fn translocate(
        &mut self,
        residual_state: Vec<f64>,
        src: CausalCoordinate,
        tar: CausalCoordinate,
        theta: f64,
        source_index: usize,
        target_index: usize,
        active_velocity_c: f64,
    ) -> PyResult<Vec<(f64, f64)>> {
        // HIL: eigenvector-rigidity detuning must remain below 1e-12.
        let detuning = Self::eigen_detuning(&residual_state).map_err(PyErr::from)?;
        let hil_status = self.hil.check_anomaly_closure(detuning);
        if hil_status == "EMERGENCY_ANOMALY_CLOSURE" {
            return Err(ReconError::AnomalyClosureError(
                "HIL safety monitor: eigenvector rigidity detuning exceeds 10^-12".to_string(),
            )
            .into());
        }

        // Metric nullification: active metric must still have det = -1.0.
        let active_metric = self.metric.audit_velocity(active_velocity_c);
        if !active_metric.passed {
            return Err(ReconError::AnomalyClosureError(
                "Metric nullification audit failed on active metric slice".to_string(),
            )
            .into());
        }

        // De-render the visible block into the dark ledger.
        self.engine
            .execute_stinespring_map(source_index, residual_state)?;

        // Metric nullification: post-nullification det = -1.0.
        let nullified_metric = self.metric.audit_velocity(0.0);
        if !nullified_metric.passed {
            return Err(ReconError::AnomalyClosureError(
                "Metric nullification audit failed after de-rendering".to_string(),
            )
            .into());
        }

        // Causal authorization: target must lie in the future causal cone.
        verify_future_cone_fatal(&src, &tar)?;

        // Phase-locked re-render at the target boundary address.
        let result = reconstruct_state(
            self.engine.state_vector_mut(),
            &src,
            &tar,
            theta,
            source_index,
            target_index,
        )
        .map_err(PyErr::from)?;

        // Guard against collapse below the holographic noise floor.
        let mut amp_sq = Float::with_val(crate::constants::PREC, 0);
        for (re, im) in &result {
            let mut r = Float::with_val(crate::constants::PREC, *re);
            r.square_mut();
            let mut j = Float::with_val(crate::constants::PREC, *im);
            j.square_mut();
            amp_sq += r;
            amp_sq += j;
        }
        let noise = Float::with_val(crate::constants::PREC, HOLOGRAPHIC_NOISE_FLOOR);
        if amp_sq < noise {
            return Err(ReconError::PrecisionLossError(
                "Reconstructed state collapsed below the holographic noise floor".to_string(),
            )
            .into());
        }

        Ok(result)
    }

    /// Full system audit: engine, HIL, metric nullification, hardware, and thermodynamics.
    pub fn audit<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let engine_audit = self.engine.audit_impl().map_err(PyErr::from)?;

        // Stinespring amplitude is the completed dark fraction 23/33.
        let eta_d = DarkLedger::new().stinespring_amplitude();
        let active_metric = self.metric.audit_velocity(eta_d as f64);
        let nullified_metric = self.metric.audit_velocity(0.0);

        let hardware_audit = self.hardware.audit(py)?;
        let thermo_audit = self.thermo.audit(py)?;

        // Run a nominal HIL step using the engine audit values.
        let eigen_detuning = engine_audit
            .get("eigenvector_rigidity_detuning")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let hil_status = self.hil.audit_hil_step(
            active_metric.minimum_gram_eigenvalue,
            active_metric.determinant_error,
            eigen_detuning,
            self.thermo.c_get_j(),
            self.thermo.landauer_limit_j(),
        );

        let d = PyDict::new(py);
        let engine_dict = PyDict::new(py);
        for (k, v) in engine_audit {
            engine_dict.set_item(k, v)?;
        }
        d.set_item("engine", engine_dict)?;
        d.set_item("hil_status", hil_status)?;
        d.set_item("active_metric", active_metric.to_dict(py)?)?;
        d.set_item("nullified_metric", nullified_metric.to_dict(py)?)?;
        d.set_item("hardware", hardware_audit)?;
        d.set_item("thermodynamics", thermo_audit)?;
        Ok(d)
    }
}

impl Default for ModularStateTranslocator {
    fn default() -> Self {
        Self::new().expect("default translocator uses the benchmark kernel")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_residual() -> Vec<f64> {
        let n = (crate::constants::DARK_LEDGER_DIM as f64).sqrt();
        vec![1.0 / n; crate::constants::DARK_LEDGER_DIM]
    }

    #[test]
    fn translocate_future_point_passes() {
        let mut trans = ModularStateTranslocator::new().unwrap();
        let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
        let tar = CausalCoordinate::new(1.0, 0.0, 0.0, 0.0);
        let result = trans
            .translocate(unit_residual(), src, tar, 0.421, 0, 1, 1.071186)
            .unwrap();
        assert_eq!(result.len(), crate::constants::DARK_LEDGER_DIM);
    }

    #[test]
    fn translocate_rejects_spacelike_target() {
        let mut trans = ModularStateTranslocator::new().unwrap();
        let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
        let tar = CausalCoordinate::new(0.0, 2.0, 0.0, 0.0);
        let result = trans.translocate(unit_residual(), src, tar, 0.0, 0, 1, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn audit_returns_all_sections() {
        pyo3::prepare_freethreaded_python();
        let mut trans = ModularStateTranslocator::new().unwrap();
        Python::with_gil(|py| {
            let audit = trans.audit(py).unwrap();
            assert!(audit.get_item("engine").unwrap().is_some());
            assert!(audit.get_item("active_metric").unwrap().is_some());
            assert!(audit.get_item("nullified_metric").unwrap().is_some());
            assert!(audit.get_item("hardware").unwrap().is_some());
            assert!(audit.get_item("thermodynamics").unwrap().is_some());
        });
    }
}
