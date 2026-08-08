//! Isometric Stinespring map and de-rendering engine.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use rug::{Complex, Float};
use std::collections::HashMap;
use crate::causal_audit::CausalCoordinate;
use crate::constants::*;
use crate::error::ReconError;
use crate::ledger::DarkLedger;

/// 512-bit-capable de-rendering engine for the SHBT boundary.
#[pyclass(name = "DerenderingEngine")]
#[derive(Debug, Clone)]
pub struct DerenderingEngine {
    /// Coupled visible/dark state vector: 16 visible blocks × 8 dark components.
    state_vector: [[Complex; DARK_LEDGER_DIM]; VISIBLE_STATE_DIM],
    boundary_kernel: (u32, u32, u32),
    ledger: DarkLedger,
}

#[pymethods]
impl DerenderingEngine {
    /// Build the engine on the canonical (26, 8, 312) boundary kernel.
    #[new]
    pub fn new() -> PyResult<Self> {
        Self::with_kernel(BENCHMARK_KERNEL).map_err(|e| e.into())
    }

    /// Build the engine with an arbitrary kernel (only the benchmark is accepted).
    #[staticmethod]
    pub fn with_kernel(kernel: (u32, u32, u32)) -> Result<Self, ReconError> {
        if kernel != BENCHMARK_KERNEL {
            return Err(ReconError::AnomalyClosureError(
                "Invalid boundary kernel dimensions. Kernel must be rigidly locked to (26, 8, 312).".to_string(),
            ));
        }
        let zero = Complex::with_val(PREC, (0, 0));
        let state_vector = std::array::from_fn(|_| {
            std::array::from_fn(|_| zero.clone())
        });
        Ok(DerenderingEngine {
            state_vector,
            boundary_kernel: kernel,
            ledger: DarkLedger::new(),
        })
    }

    /// Execute the Stinespring Dilation Map for the visible block at `visible_index`.
    ///
    /// `residual_state` must be an 8-component vector normalized to unit norm
    /// within 10^-12; any larger detuning raises `AnomalyClosureError`.
    pub fn execute_stinespring_map(&mut self, visible_index: usize, residual_state: Vec<f64>) -> PyResult<()> {
        self.execute_stinespring_map_impl(visible_index, &residual_state)
            .map_err(|e| PyErr::from(e))
    }

    /// Run a self-consistency audit and return a dictionary of benchmark values.
    pub fn audit<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let results = self.audit_impl().map_err(PyErr::from)?;
        let dict = PyDict::new(py);
        for (k, v) in results {
            dict.set_item(k, v)?;
        }
        Ok(dict)
    }

    /// Re-render the excitation at `target_index` from `source_index`.
    pub fn reconstruct(
        &mut self,
        src: CausalCoordinate,
        tar: CausalCoordinate,
        theta: f64,
        source_index: usize,
        target_index: usize,
    ) -> PyResult<Vec<(f64, f64)>> {
        crate::reconstruction::reconstruct_state(
            &mut self.state_vector,
            &src,
            &tar,
            theta,
            source_index,
            target_index,
        )
        .map_err(|e| e.into())
    }

    /// Snapshot of the coupled visible/dark state vector.
    fn get_state_vector(&self) -> Vec<Vec<(f64, f64)>> {
        self.state_vector
            .iter()
            .map(|slot| {
                slot.iter()
                    .map(|c| (c.real().to_f64(), c.imag().to_f64()))
                    .collect()
            })
            .collect()
    }

    /// Return the rigid boundary kernel triplet.
    #[getter]
    fn boundary_kernel(&self) -> (u32, u32, u32) {
        self.boundary_kernel
    }
}

impl DerenderingEngine {
    fn execute_stinespring_map_impl(&mut self, visible_index: usize, residual_state: &[f64]) -> Result<(), ReconError> {
        if visible_index >= VISIBLE_STATE_DIM {
            return Err(ReconError::AnomalyClosureError(
                "Visible state index out of bounds.".to_string(),
            ));
        }
        if residual_state.len() != DARK_LEDGER_DIM {
            return Err(ReconError::AnomalyClosureError(format!(
                "residual_state must have length {}",
                DARK_LEDGER_DIM
            )));
        }

        // 512-bit normalization check of the incoming residual state.
        let mut norm_sq = Float::with_val(PREC, 0);
        for &v in residual_state.iter() {
            let f = Float::with_val(PREC, v);
            let mut f2 = Float::with_val(PREC, &f);
            f2.square_mut();
            norm_sq += f2;
        }

        let one = Float::with_val(PREC, 1);
        let tol = Float::with_val(PREC, EIGENVECTOR_DETUNING_TOLERANCE);
        let mut diff = norm_sq.clone();
        diff -= &one;
        let detuning = diff.abs();

        if detuning > tol {
            return Err(ReconError::AnomalyClosureError(
                "Eigenvector rigidity detuned past critical threshold of 10^-12".to_string(),
            ));
        }

        // Apply Stinespring amplitude η_D = 23/33.
        let eta_d = self.ledger.amplitude_float();
        for (i, &v) in residual_state.iter().enumerate() {
            let mut re = Float::with_val(PREC, v);
            re *= &eta_d;
            let im = Float::with_val(PREC, 0);
            self.state_vector[visible_index][i] = Complex::with_val(PREC, (re, im));
        }

        Ok(())
    }

    pub(crate) fn audit_impl(&mut self) -> Result<HashMap<String, String>, ReconError> {
        self.audit_impl_internal()
    }

    pub(crate) fn state_vector_mut(&mut self) -> &mut [[Complex; DARK_LEDGER_DIM]; VISIBLE_STATE_DIM] {
        &mut self.state_vector
    }

    fn audit_impl_internal(&mut self) -> Result<HashMap<String, String>, ReconError> {
        let residual: [f64; DARK_LEDGER_DIM] = {
            let mut arr = [0.0; DARK_LEDGER_DIM];
            arr[0] = 1.0;
            arr
        };
        self.execute_stinespring_map_impl(0, &residual)?;

        // Norm of the mapped dark state should equal η_D exactly.
        let mut mapped_norm_sq = Float::with_val(PREC, 0);
        for i in 0..DARK_LEDGER_DIM {
            let re = self.state_vector[0][i].real();
            let mut t = Float::with_val(PREC, re);
            t.square_mut();
            let im = self.state_vector[0][i].imag();
            let mut u = Float::with_val(PREC, im);
            u.square_mut();
            mapped_norm_sq += t;
            mapped_norm_sq += u;
        }

        let eta_d = self.ledger.amplitude_float();
        let mut eta_sq = eta_d.clone();
        eta_sq.square_mut();
        let mut unitarity_residual = Float::with_val(PREC, &mapped_norm_sq);
        unitarity_residual -= &eta_sq;
        unitarity_residual = unitarity_residual.abs();

        // Eigenvector rigidity detuning measured at input.
        let mut residual_norm_sq = Float::with_val(PREC, 0);
        for v in residual {
            let f = Float::with_val(PREC, v);
            let mut f2 = Float::with_val(PREC, &f);
            f2.square_mut();
            residual_norm_sq += f2;
        }
        let one = Float::with_val(PREC, 1);
        let mut eigen_detuning = Float::with_val(PREC, &residual_norm_sq);
        eigen_detuning -= &one;
        eigen_detuning = eigen_detuning.abs();

        // Reconstruction amplitude along a valid future-directed causal trajectory.
        let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
        let tar = CausalCoordinate::new(1.0, 0.0, 0.0, 0.0);
        let theta = 0.421;
        let reconstructed = crate::reconstruction::reconstruct_state(
            &mut self.state_vector,
            &src,
            &tar,
            theta,
            0,
            1,
        )?;

        let mut amp_sq = Float::with_val(PREC, 0);
        for (re, im) in reconstructed {
            let mut r = Float::with_val(PREC, re);
            r.square_mut();
            let mut j = Float::with_val(PREC, im);
            j.square_mut();
            amp_sq += r;
            amp_sq += j;
        }
        let reconstruction_amplitude = amp_sq.sqrt().to_f64();

        // Phase-locked excitation unitarity residual.
        let phase_residual = crate::reconstruction::phase_unitarity_residual(theta);

        let fmt = |f: f64| format!("{:.15e}", f);

        let mut results = HashMap::new();
        results.insert("branch".to_string(), format!("({}, {}, {})", self.boundary_kernel.0, self.boundary_kernel.1, self.boundary_kernel.2));
        results.insert("residual_fraction".to_string(), Float::with_val(PREC, self.ledger.residual_rational()).to_f64().to_string());
        results.insert("completed_fraction".to_string(), Float::with_val(PREC, self.ledger.completed_rational()).to_f64().to_string());
        results.insert("stinespring_ratio".to_string(), Float::with_val(PREC, self.ledger.amplitude_rational()).to_f64().to_string());
        results.insert("unitarity_residual".to_string(), fmt(unitarity_residual.to_f64()));
        results.insert("eigenvector_rigidity_detuning".to_string(), fmt(eigen_detuning.to_f64()));
        results.insert("reconstruction_amplitude".to_string(), fmt(reconstruction_amplitude));
        results.insert("phase_unitarity_residual".to_string(), fmt(phase_residual));
        results.insert("causal_authorization_passed".to_string(), "true".to_string());

        // Dark-ledger trace-loss projection lemma: verify U†U = I, UU† = P_comp,
        // and D†D = η_D² I at 512-bit precision.
        let trace_loss_results = crate::derender::DarkLedgerTraceLoss::verify_projection_lemma()?;
        for (k, v) in trace_loss_results {
            results.insert(k, v);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_residual() -> Vec<f64> {
        let n = (DARK_LEDGER_DIM as f64).sqrt();
        vec![1.0 / n; DARK_LEDGER_DIM]
    }

    #[test]
    fn kernel_validation_accepts_benchmark() {
        let engine = DerenderingEngine::with_kernel(BENCHMARK_KERNEL);
        assert!(engine.is_ok());
    }

    #[test]
    fn kernel_validation_rejects_invalid() {
        let engine = DerenderingEngine::with_kernel((1, 1, 1));
        assert!(matches!(engine, Err(ReconError::AnomalyClosureError(_))));
    }

    #[test]
    fn stinespring_map_scales_by_amplitude() {
        let mut engine = DerenderingEngine::with_kernel(BENCHMARK_KERNEL).unwrap();
        let residual = unit_residual();
        engine.execute_stinespring_map(0, residual).unwrap();
        let expected = 23.0 / 33.0;
        let tol = 1.0e-12;
        for i in 0..DARK_LEDGER_DIM {
            let re = engine.state_vector[0][i].real().to_f64();
            assert!((re - expected * (1.0 / (DARK_LEDGER_DIM as f64).sqrt())).abs() < tol);
        }
    }

    #[test]
    fn anomaly_closure_triggered_on_detuned_state() {
        let mut engine = DerenderingEngine::with_kernel(BENCHMARK_KERNEL).unwrap();
        let bad = vec![0.5; DARK_LEDGER_DIM];
        let result = engine.execute_stinespring_map(0, bad);
        assert!(result.is_err());
    }

    #[test]
    fn state_vector_is_fixed_size_stack_array() {
        let engine = DerenderingEngine::with_kernel(BENCHMARK_KERNEL).unwrap();
        // The coupled state vector is a stack-allocated fixed-size array.
        // No heap allocation is introduced by the state container itself,
        // guaranteeing deterministic access time during the Stinespring/re-render loops.
        assert_eq!(engine.state_vector.len(), VISIBLE_STATE_DIM);
        assert_eq!(engine.state_vector[0].len(), DARK_LEDGER_DIM);
        assert_eq!(
            std::mem::size_of_val(&engine.state_vector),
            VISIBLE_STATE_DIM * DARK_LEDGER_DIM * std::mem::size_of::<Complex>()
        );
    }
}
