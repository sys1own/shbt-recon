//! CVD diamond Debye T^3 cryogenic headroom integration.
//!
//!   u(T) = (3 pi^4 n R / (5 Theta_D^3)) (T^4 - T0^4)
//!
//! with molar density n = rho/M_C = 292,639.8 mol/m^3, Theta_D = 2230 K.
//! Integrated energy densities: u(4.21 K) = 4.0285 J/m^3,
//! u(16.0 K) = 840.4201 J/m^3, giving the NbN quench headroom
//! 16.0 - 4.21 = 11.79 K against the 142.08 MW / 1 ms field-collapse
//! transient.

pub const R_GAS: f64 = 8.314462618;
pub const THETA_D_K: f64 = 2230.0;
pub const RHO_DIAMOND_KG_M3: f64 = 3515.0;
pub const MOLAR_MASS_C_KG_MOL: f64 = 0.012011;
/// Cryogenic baseline operating temperature (K).
pub const T0_K: f64 = 0.1;
/// Transient peak envelope temperature (K).
pub const T_PEAK_K: f64 = 4.21;
/// NbN critical temperature (K).
pub const T_C_NBN_K: f64 = 16.0;
/// MgB2 critical temperature (K).
pub const T_C_MGB2_K: f64 = 39.0;
/// Maximum field-collapse transient power (MW over 1 ms).
pub const FIELD_COLLAPSE_MW: f64 = 142.08;
/// CVD diamond thermal conductivity (W/m*K).
pub const K_DIAMOND_W_MK: f64 = 2250.0;
/// GaN HEMT thermal boundary resistance (m^2*K/W).
pub const TBR_GAN_M2K_W: f64 = 6.2e-9;
/// Cryo thermal shock cycle durability.
pub const THERMAL_CYCLES: u32 = 1500;

/// Molar density of crystalline diamond (mol/m^3).
pub fn molar_density() -> f64 {
    RHO_DIAMOND_KG_M3 / MOLAR_MASS_C_KG_MOL
}

/// Debye T^3 volumetric energy density at temperature T (J/m^3),
/// integrated from T0.
pub fn diamond_volumetric_energy(t_k: f64) -> f64 {
    let coeff = 3.0 * std::f64::consts::PI.powi(4) * molar_density() * R_GAS
        / (5.0 * THETA_D_K.powi(3));
    coeff * (t_k.powi(4) - T0_K.powi(4))
}

/// Absolute NbN quench headroom (K).
pub fn quench_headroom(t_peak: f64) -> f64 {
    T_C_NBN_K - t_peak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn energy_at_peak_matches_spec() {
        assert!((diamond_volumetric_energy(T_PEAK_K) - 4.0285).abs() < 0.01);
    }

    #[test]
    fn energy_at_nbn_tc() {
        assert!((diamond_volumetric_energy(T_C_NBN_K) - 840.42).abs() < 1.0);
    }

    #[test]
    fn headroom_margin() {
        assert!((quench_headroom(T_PEAK_K) - 11.79).abs() < 0.01);
        assert!(quench_headroom(T_PEAK_K) >= 10.0);
    }
}
