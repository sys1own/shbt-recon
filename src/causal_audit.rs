//! Causal authorization for re-rendering.

use pyo3::prelude::*;
use crate::error::ReconError;

/// Spacetime coordinate used for causal-cone authorization.
#[pyclass(name = "CausalCoordinate")]
#[derive(Debug, Clone, Copy)]
pub struct CausalCoordinate {
    #[pyo3(get, set)]
    pub t: f64,
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
    #[pyo3(get, set)]
    pub z: f64,
}

#[pymethods]
impl CausalCoordinate {
    #[new]
    pub fn new(t: f64, x: f64, y: f64, z: f64) -> Self {
        CausalCoordinate { t, x, y, z }
    }

    /// Return true when `other` lies within or on the future causal cone of `self`
    /// (i.e. `other` is a causal descendant of `self`).
    pub fn is_causally_authorized(&self, other: CausalCoordinate) -> bool {
        self.is_causally_authorized_ref(&other)
    }
}

impl CausalCoordinate {
    pub fn is_causally_authorized_ref(&self, other: &Self) -> bool {
        let dt = other.t - self.t;
        if dt < 0.0 {
            return false;
        }
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        let spatial_interval = dx * dx + dy * dy + dz * dz;
        let temporal_interval = dt * dt;
        // inside or on the future light cone (c = 1 in natural units)
        spatial_interval <= temporal_interval
    }
}

/// Verify that `tar` is in the future causal cone of `src`.
pub fn verify_future_cone(src: &CausalCoordinate, tar: &CausalCoordinate) -> Result<(), ReconError> {
    verify_future_cone_fatal(src, tar)
}

/// Fatal causal-cone check.  Spacelike or past targets raise `AnomalyClosureError`.
pub fn verify_future_cone_fatal(src: &CausalCoordinate, tar: &CausalCoordinate) -> Result<(), ReconError> {
    if !src.is_causally_authorized_ref(tar) {
        return Err(ReconError::AnomalyClosureError(
            "Target coordinate is outside the future causal cone of the source".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn future_point_authorized() {
        let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
        let tar = CausalCoordinate::new(1.0, 0.0, 0.0, 0.0);
        assert!(src.is_causally_authorized_ref(&tar));
    }

    #[test]
    fn spacelike_point_not_authorized() {
        let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
        let tar = CausalCoordinate::new(0.0, 1.0, 1.0, 1.0);
        assert!(!src.is_causally_authorized_ref(&tar));
    }

    #[test]
    fn past_point_not_authorized() {
        let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
        let tar = CausalCoordinate::new(-1.0, 0.0, 0.0, 0.0);
        assert!(!src.is_causally_authorized_ref(&tar));
    }
}
