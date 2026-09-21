//! 5th-order minimum-jerk trajectory profile for swarm node maneuvering:
//!   s(τ) = 10τ³ − 15τ⁴ + 6τ⁵,  τ = t / T_alloc ∈ [0, 1].

/// Normalized minimum-jerk position profile.
pub fn s(tau: f64) -> f64 {
    let t2 = tau * tau;
    let t3 = t2 * tau;
    10.0 * t3 - 15.0 * t2 * t2 + 6.0 * t3 * t2
}

/// Normalized velocity profile: ṡ(τ) = 30τ² − 60τ³ + 30τ⁴.
pub fn ds(tau: f64) -> f64 {
    let t2 = tau * tau;
    30.0 * t2 - 60.0 * t2 * tau + 30.0 * t2 * t2
}

/// Normalized acceleration profile: s̈(τ) = 60τ − 180τ² + 120τ³.
pub fn dds(tau: f64) -> f64 {
    60.0 * tau - 180.0 * tau * tau + 120.0 * tau * tau * tau
}

/// Peak normalized velocity, attained at τ = 0.5: ṡ_max = 1.875.
pub const MAX_VELOCITY_COEFF: f64 = 1.875;

/// Peak normalized acceleration magnitude,
/// |s̈(0.5 ± 1/√12)| = √300 / 3 ≈ 5.773502.
pub const MAX_ACCEL_COEFF: f64 = 5.773_502_691_896_257; // sqrt(300)/3

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoints_and_symmetry() {
        assert_eq!(s(0.0), 0.0);
        assert_eq!(s(1.0), 1.0);
        assert!((ds(0.5) - MAX_VELOCITY_COEFF).abs() < 1e-12);
    }

    #[test]
    fn peak_acceleration() {
        let tau = 0.5 - 1.0_f64 / 12.0_f64.sqrt();
        assert!((dds(tau).abs() - MAX_ACCEL_COEFF).abs() < 1e-9);
    }
}
