//! Landauer GET thermodynamic accounting — `LandauerGetCalculator`
//! transferred from `sys1own/shbt-precision`.
//!
//! C_get = max(1, log₂|R|) for reduction-matrix rank |R|, with dissipation
//! bound Q_H ≥ k_B · T · ln 2 · C_op.

/// Boltzmann constant (J/K).
pub const KB_J_PER_K: f64 = 1.380_649e-23;

/// GET operation cost calculator (`LandauerGetCalculator` transfer).
#[derive(Clone, Copy, Debug, Default)]
pub struct LandauerGetCalculator;

impl LandauerGetCalculator {
    pub fn new() -> Self {
        Self
    }

    /// GET cost C_get = max(1, log₂|R|) for reduction-matrix rank `rank`.
    pub fn c_get(&self, rank: u64) -> f64 {
        if rank <= 1 {
            return 1.0;
        }
        (rank as f64).log2().max(1.0)
    }

    /// Landauer dissipation bound for an operation of cost `c_op` at `t` K:
    /// Q_H ≥ k_B T ln2 · C_op (J).
    pub fn dissipation_bound_j(&self, c_op: f64, t: f64) -> f64 {
        KB_J_PER_K * t * std::f64::consts::LN_2 * c_op
    }

    /// Dissipation bound for a rank-`rank` GET at `t` K (J).
    pub fn get_dissipation_j(&self, rank: u64, t: f64) -> f64 {
        self.dissipation_bound_j(self.c_get(rank), t)
    }

    /// True when measured dissipation `q_j` satisfies the bound.
    pub fn bound_satisfied(&self, q_j: f64, c_op: f64, t: f64) -> bool {
        q_j >= self.dissipation_bound_j(c_op, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_get_floor_is_one() {
        let c = LandauerGetCalculator::new();
        assert_eq!(c.c_get(0), 1.0);
        assert_eq!(c.c_get(1), 1.0);
        assert_eq!(c.c_get(2), 1.0);
        assert!((c.c_get(8) - 3.0).abs() < 1e-12);
        assert!((c.c_get(33) - 33f64.log2()).abs() < 1e-12);
    }

    #[test]
    fn dissipation_bound_at_mixing_chamber() {
        let c = LandauerGetCalculator::new();
        let q = c.get_dissipation_j(8, 15.0e-3);
        assert!((q - KB_J_PER_K * 15.0e-3 * std::f64::consts::LN_2 * 3.0).abs() < 1e-30);
        assert!(c.bound_satisfied(q * 1.01, 3.0, 15.0e-3));
        assert!(!c.bound_satisfied(q * 0.5, 3.0, 15.0e-3));
    }
}
