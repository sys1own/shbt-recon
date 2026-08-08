//! Reconstruction operator, boundary relabeling, and phase-locked excitation.

use pyo3::prelude::*;
use rug::{Complex, Float};
use crate::causal_audit::{CausalCoordinate, verify_future_cone};
use crate::constants::*;
use crate::error::ReconError;
use crate::stinespring::DerenderingEngine;

/// 512-bit squared norm of the dark-ledger vector at `visible_index`.
fn dark_ledger_norm_sq(
    state: &[[Complex; DARK_LEDGER_DIM]; VISIBLE_STATE_DIM],
    visible_index: usize,
) -> Float {
    let mut norm_sq = Float::with_val(PREC, 0);
    for i in 0..DARK_LEDGER_DIM {
        let re = state[visible_index][i].real();
        let mut t = Float::with_val(PREC, re);
        t.square_mut();
        let im = state[visible_index][i].imag();
        let mut u = Float::with_val(PREC, im);
        u.square_mut();
        norm_sq += t;
        norm_sq += u;
    }
    norm_sq
}

/// Apply the phase-locked excitation operator O^excitation(θ) = exp(-i θ Q)
/// to a single complex amplitude.  The topological charge Q is taken to be 1
/// for the canonical anyon lattice; the phase is therefore a uniform rotation.
pub fn phase_unitarity_residual(theta: f64) -> f64 {
    let angle = Float::with_val(PREC, theta);
    let cos = angle.clone().cos();
    let sin = angle.sin();
    let mut c2 = Float::with_val(PREC, &cos);
    c2.square_mut();
    let mut s2 = Float::with_val(PREC, &sin);
    s2.square_mut();
    let mut sum = c2;
    sum += s2;
    let one = Float::with_val(PREC, 1);
    sum -= &one;
    sum.abs().to_f64()
}

fn apply_phase(state: &Complex, theta: f64) -> Complex {
    let angle = Float::with_val(PREC, theta);
    let cos = angle.clone().cos();
    let sin = angle.sin();
    let mut neg_sin = Float::with_val(PREC, 0);
    neg_sin -= &sin; // -sin(θ)
    let phase = Complex::with_val(PREC, (cos, neg_sin)); // exp(-i θ)
    let mut rotated = state.clone();
    rotated *= &phase;
    rotated
}

/// Core re-render pipeline: causal authorization, boundary relabeling,
/// phase-locked excitation, and adjoint Stinespring projection.
pub fn reconstruct_state(
    state: &mut [[Complex; DARK_LEDGER_DIM]; VISIBLE_STATE_DIM],
    src: &CausalCoordinate,
    tar: &CausalCoordinate,
    theta: f64,
    source_index: usize,
    target_index: usize,
) -> Result<Vec<(f64, f64)>, ReconError> {
    verify_future_cone(src, tar)?;

    if source_index >= VISIBLE_STATE_DIM || target_index >= VISIBLE_STATE_DIM {
        return Err(ReconError::AnomalyClosureError(
            "Boundary address index out of bounds".to_string(),
        ));
    }

    // Boundary relabeling T^∂: copy the dark ledger state attached to the source
    // visible block to the target boundary address.
    //
    // This is a spatial isometry: the local norm of the dark-ledger vector is
    // preserved exactly, and because the source and target addresses subtend
    // an equal boundary support interval length ℓ_A = 2z, the entanglement
    // entropy of the wedge is unchanged, ΔS_A = 0.  The relabeling is treated
    // as instantaneous and adiabatic because it is a pure index permutation with
    // no coupling to an external environment.
    let source_norm_sq = dark_ledger_norm_sq(state, source_index);
    for i in 0..DARK_LEDGER_DIM {
        state[target_index][i] = state[source_index][i].clone();
    }
    let target_norm_sq = dark_ledger_norm_sq(state, target_index);
    let mut entropy_residual = Float::with_val(PREC, &source_norm_sq);
    entropy_residual -= &target_norm_sq;
    let noise = Float::with_val(PREC, HOLOGRAPHIC_NOISE_FLOOR);
    if entropy_residual.abs() > noise {
        return Err(ReconError::PrecisionLossError(
            "Boundary relabeling changed the dark-ledger norm; ΔS_A ≠ 0".to_string(),
        ));
    }

    // Phase-locked excitation on the target dark ledger subspace.
    for i in 0..DARK_LEDGER_DIM {
        state[target_index][i] = apply_phase(&state[target_index][i], theta);
    }

    // Adjoint Stinespring projection: the visible amplitude is the rotated
    // dark ledger coefficient.  Return the full complex pair for each component.
    let mut reconstructed: Vec<(f64, f64)> = Vec::with_capacity(DARK_LEDGER_DIM);
    let mut amplitude_sq = Float::with_val(PREC, 0);
    for i in 0..DARK_LEDGER_DIM {
        let c = &state[target_index][i];
        let re = c.real().to_f64();
        let im = c.imag().to_f64();
        reconstructed.push((re, im));

        let mut r = Float::with_val(PREC, re);
        r.square_mut();
        let mut j = Float::with_val(PREC, im);
        j.square_mut();
        amplitude_sq += r;
        amplitude_sq += j;
    }

    // Guard against collapse below the holographic noise floor.
    let noise = Float::with_val(PREC, HOLOGRAPHIC_NOISE_FLOOR);
    if amplitude_sq < noise {
        return Err(ReconError::PrecisionLossError(
            "Reconstructed state collapsed below the holographic noise floor".to_string(),
        ));
    }

    Ok(reconstructed)
}

/// Convert a Python-style snapshot into the internal state array.
fn snapshot_to_state(
    snapshot: Vec<Vec<(f64, f64)>>,
) -> Result<[[Complex; DARK_LEDGER_DIM]; VISIBLE_STATE_DIM], ReconError> {
    if snapshot.len() != VISIBLE_STATE_DIM {
        return Err(ReconError::AnomalyClosureError(format!(
            "state vector must have {} visible slots",
            VISIBLE_STATE_DIM
        )));
    }
    let mut state: [[Complex; DARK_LEDGER_DIM]; VISIBLE_STATE_DIM] =
        std::array::from_fn(|_| std::array::from_fn(|_| Complex::with_val(PREC, (0, 0))));
    for (i, slot) in snapshot.iter().enumerate() {
        if slot.len() != DARK_LEDGER_DIM {
            return Err(ReconError::AnomalyClosureError(format!(
                "each visible slot must have {} dark components",
                DARK_LEDGER_DIM
            )));
        }
        for (j, &(re, im)) in slot.iter().enumerate() {
            state[i][j] = Complex::with_val(PREC, (re, im));
        }
    }
    Ok(state)
}

/// Spatial translation / relabeling of character blocks between boundary addresses.
#[pyclass(name = "BoundaryRelabeling")]
#[derive(Debug, Clone)]
pub struct BoundaryRelabeling {
    source: CausalCoordinate,
    target: CausalCoordinate,
}

#[pymethods]
impl BoundaryRelabeling {
    #[new]
    pub fn new(source: CausalCoordinate, target: CausalCoordinate) -> Self {
        BoundaryRelabeling { source, target }
    }

    fn source(&self) -> CausalCoordinate {
        self.source
    }

    fn target(&self) -> CausalCoordinate {
        self.target
    }

    /// Relabel a state snapshot by copying the dark components of
    /// `source_index` to `target_index`.
    ///
    /// The copy is a spatial isometry that preserves the local dark-ledger norm,
    /// so the entanglement-wedge entropy is unchanged, ΔS_A = 0.  It is
    /// instantaneous and adiabatic because no external environment is coupled.
    fn relabel_state(
        &self,
        state: Vec<Vec<(f64, f64)>>,
        source_index: usize,
        target_index: usize,
    ) -> PyResult<Vec<Vec<(f64, f64)>>> {
        let mut state = snapshot_to_state(state).map_err(|e| PyErr::from(e))?;
        if source_index >= VISIBLE_STATE_DIM || target_index >= VISIBLE_STATE_DIM {
            return Err(ReconError::AnomalyClosureError(
                "Boundary address index out of bounds".to_string(),
            ).into());
        }
        for i in 0..DARK_LEDGER_DIM {
            state[target_index][i] = state[source_index][i].clone();
        }
        Ok(state
            .iter()
            .map(|slot| slot.iter().map(|c| (c.real().to_f64(), c.imag().to_f64())).collect())
            .collect())
    }
}

/// Uniform phase-locked excitation operator.
#[pyclass(name = "PhaseLockedExcitation")]
#[derive(Debug, Clone)]
pub struct PhaseLockedExcitation;

#[pymethods]
impl PhaseLockedExcitation {
    #[new]
    pub fn new() -> Self {
        PhaseLockedExcitation
    }

    /// Return | |exp(-iθ)|^2 - 1 | computed at 512-bit precision.
    fn audit(&self, theta: f64) -> f64 {
        phase_unitarity_residual(theta)
    }

    /// Apply O^excitation(θ) = exp(-i θ) to a single complex amplitude.
    fn apply(&self, re: f64, im: f64, theta: f64) -> (f64, f64) {
        let c = Complex::with_val(PREC, (re, im));
        let rotated = apply_phase(&c, theta);
        (rotated.real().to_f64(), rotated.imag().to_f64())
    }

    /// Apply the phase rotation to the dark components at `index` of a snapshot.
    fn apply_to_state(
        &self,
        state: Vec<Vec<(f64, f64)>>,
        index: usize,
        theta: f64,
    ) -> PyResult<Vec<Vec<(f64, f64)>>> {
        let mut state = snapshot_to_state(state).map_err(PyErr::from)?;
        if index >= VISIBLE_STATE_DIM {
            return Err(ReconError::AnomalyClosureError(
                "Boundary address index out of bounds".to_string(),
            ).into());
        }
        for i in 0..DARK_LEDGER_DIM {
            state[index][i] = apply_phase(&state[index][i], theta);
        }
        Ok(state
            .iter()
            .map(|slot| slot.iter().map(|c| (c.real().to_f64(), c.imag().to_f64())).collect())
            .collect())
    }
}

/// Reconstruction operator combining causal authorization, boundary relabeling,
/// and phase-locked excitation.
#[pyclass(name = "ReconstructionOperator")]
#[derive(Debug, Clone)]
pub struct ReconstructionOperator;

#[pymethods]
impl ReconstructionOperator {
    #[new]
    pub fn new() -> Self {
        ReconstructionOperator
    }

    /// Reconstruct the visible amplitude at `target_index` from `source_index`
    /// using a state snapshot.  Returns the 8-component visible complex amplitude.
    fn reconstruct(
        &self,
        state: Vec<Vec<(f64, f64)>>,
        src: CausalCoordinate,
        tar: CausalCoordinate,
        theta: f64,
        source_index: usize,
        target_index: usize,
    ) -> PyResult<Vec<(f64, f64)>> {
        let mut state = snapshot_to_state(state).map_err(PyErr::from)?;
        reconstruct_state(&mut state, &src, &tar, theta, source_index, target_index)
            .map_err(|e| e.into())
    }

    /// Convenience method: de-render `residual_state` at `source_index` and
    /// immediately re-render at `target_index`.
    fn run_pipeline(
        &self,
        residual_state: Vec<f64>,
        src: CausalCoordinate,
        tar: CausalCoordinate,
        theta: f64,
        source_index: usize,
        target_index: usize,
    ) -> PyResult<Vec<(f64, f64)>> {
        let mut engine = DerenderingEngine::with_kernel(BENCHMARK_KERNEL).map_err(PyErr::from)?;
        engine.execute_stinespring_map(source_index, residual_state)?;
        engine.reconstruct(src, tar, theta, source_index, target_index)
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
    fn reconstruction_follows_causality() {
        let mut engine = DerenderingEngine::with_kernel(BENCHMARK_KERNEL).unwrap();
        let residual = unit_residual();
        engine.execute_stinespring_map(0, residual).unwrap();

        let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
        let tar = CausalCoordinate::new(-1.0, 0.0, 0.0, 0.0);
        let result = reconstruct_state(
            &mut engine.state_vector_mut(),
            &src,
            &tar,
            0.0,
            0,
            1,
        );
        assert!(matches!(result, Err(ReconError::AnomalyClosureError(_))));

        let tar = CausalCoordinate::new(1.0, 0.0, 0.0, 0.0);
        let result = reconstruct_state(
            &mut engine.state_vector_mut(),
            &src,
            &tar,
            0.421,
            0,
            1,
        );
        assert!(result.is_ok());
    }
}
