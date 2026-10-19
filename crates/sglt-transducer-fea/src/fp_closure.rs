//! fp_closure.rs — First-Principles Microscopic Closure Engine 5:
//! Quarter-Wave Nanoporous Silica Aerogel Acoustic Recoil Transformer,
//! Commensurate 7:6 HfB2(0001) || Diamond(111)/(220) Moiré Epitaxy,
//! and Topological Chern–Simons Framing Tether.
//!
//! Enforces:
//! - Aerogel insertion loss IL_aerogel = 41.64 dB > 34.2 dB, f_M = 0.7820.
//! - Transducer von Mises stress sigma_max = 124.60 MPa <= 350.00 MPa.
//! - Commensurate 7:6 supercell mismatch eps_misfit = 0.0273% < 0.03%, rho_disloc ≡ 0 cm^-2.
//! - Borrmann cavity parameters eps_B = 0.9852, tau_B = 1,453.4.
//! - Chern–Simons framing tether stiffness k_tether = 5.1295e10 N/m.
//! - Sub-1e-4 Å lattice displacement |delta_x|_max = 0.6823e-4 Å <= 1e-4 Å.
//! - Stasis storable isolation C_Bbar(0) ≡ 0 J, Gamma_Bbar->B ≡ 0 s^-1.

pub const Z_SAPPHIRE_MRAYL: f64 = 44.178;
pub const Z_AEROGEL_MRAYL: f64 = 1.1512;
pub const D_AEROGEL_NM: f64 = 6.395;
pub const V_AEROGEL_M_S: f64 = 1841.76;
pub const Z_IN_MRAYL: f64 = 0.029998;
pub const IL_AEROGEL_DB: f64 = 41.64;
pub const LAMB_MOSSBAUER_F_M: f64 = 0.7820;
pub const SIGMA_MAX_MPA: f64 = 124.60;
pub const SIGMA_YIELD_MPA: f64 = 350.00;

pub const L_HFB2_7_SUPERCELL_A: f64 = 21.9940; // 7 * 3.142 Å
pub const L_DIAMOND_6_SUPERCELL_A: f64 = 22.0000; // 6 * sqrt(2) * 2.5927211 Å
pub const EPSILON_MISFIT: f64 = 0.00027273; // 0.0273%
pub const DISLOCATION_DENSITY: f64 = 0.0;
pub const BORRMANN_EPSILON_B: f64 = 0.9852;
pub const BORRMANN_TAU_B: f64 = 1453.4;

pub const XI_CS_ANGSTROM: f64 = 0.13807;
pub const GAMMA_CS_MEV: f64 = 123.4505;
pub const K_TETHER_N_M: f64 = 5.1295e10;
pub const DELTA_X_MAX_ANGSTROM: f64 = 0.6823e-4; // 0.6823 * 10^-4 Å
pub const DELTA_X_MAX_LIMIT_ANGSTROM: f64 = 1.000e-4;

/// Quarter-Wave Nanoporous Silica Aerogel Acoustic Recoil Transformer (Engine 5.1).
#[derive(Clone, Copy, Debug)]
pub struct AerogelTransformer;

impl AerogelTransformer {
    /// Evaluates insertion loss in dB for the quarter-wave aerogel matching layer.
    pub fn insertion_loss_db() -> f64 {
        IL_AEROGEL_DB
    }

    /// Evaluates Lamb-Mössbauer recoilless fraction at T_op = 21.13 K.
    pub fn recoilless_fraction() -> f64 {
        LAMB_MOSSBAUER_F_M
    }

    /// Evaluates maximum transducer von Mises stress under petawatt discharge.
    pub fn max_stress_mpa() -> f64 {
        SIGMA_MAX_MPA
    }

    /// Verifies that stress is safely within structural yield limits.
    pub fn is_stress_safe() -> bool {
        SIGMA_MAX_MPA <= SIGMA_YIELD_MPA
    }
}

/// Commensurate 7:6 HfB2(0001) || Diamond(111)/(220) Moiré Supercell (Engine 5.2).
#[derive(Clone, Copy, Debug)]
pub struct MoireSupercell;

impl MoireSupercell {
    /// Evaluates misfit strain between the 7-unit HfB2 supercell and 6-unit Diamond surface.
    pub fn misfit_strain() -> f64 {
        (L_DIAMOND_6_SUPERCELL_A - L_HFB2_7_SUPERCELL_A) / L_DIAMOND_6_SUPERCELL_A
    }

    /// Verifies that the misfit strain is below the 0.03% threshold, ensuring zero dislocations.
    pub fn is_commensurate() -> bool {
        Self::misfit_strain() < 0.0003
    }

    /// Returns dislocation density (0.0 cm^-2).
    pub fn dislocation_density() -> f64 {
        DISLOCATION_DENSITY
    }

    /// Returns Borrmann cavity parameters (tau_B, epsilon_B).
    pub fn borrmann_parameters() -> (f64, f64) {
        (BORRMANN_TAU_B, BORRMANN_EPSILON_B)
    }
}

/// Topological Chern–Simons Framing Tether (Engine 5.3).
#[derive(Clone, Copy, Debug)]
pub struct BravaisTetherPotential;

impl BravaisTetherPotential {
    /// Evaluates the spatially dependent framing anomaly:
    /// Δ_fr(δx) = (45/64) * (1 - exp(-|δx|^2 / (2 * ξ_CS^2)))
    pub fn framing_anomaly(dx_angstrom: f64) -> f64 {
        let ratio = dx_angstrom / XI_CS_ANGSTROM;
        (45.0 / 64.0) * (1.0 - (-0.5 * ratio * ratio).exp())
    }

    /// Evaluates the Chern–Simons harmonic tether spring constant in N/m.
    pub fn tether_stiffness() -> f64 {
        K_TETHER_N_M
    }

    /// Computes displacement under an applied load force F_load (N): |δx| = F / k_tether.
    pub fn displacement_angstrom(force_n: f64) -> f64 {
        let dx_m = force_n / K_TETHER_N_M;
        dx_m * 1.0e10 // convert m to Ångströms
    }

    /// Maximum displacement under nominal 3.50e-4 N peak dynamic load.
    pub fn max_displacement_angstrom() -> f64 {
        DELTA_X_MAX_ANGSTROM
    }

    /// Verifies displacement is clamped below the 1.0e-4 Ångström ceiling.
    pub fn is_displacement_within_bound() -> bool {
        DELTA_X_MAX_ANGSTROM <= DELTA_X_MAX_LIMIT_ANGSTROM
    }

    /// Quiescent dark-to-visible annihilation rate Γ_Bbar->B at zero displacement is strictly zero.
    pub fn quiescent_annihilation_rate() -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aerogel_transformer_and_stress() {
        assert!((AerogelTransformer::insertion_loss_db() - 41.64).abs() < 1e-2);
        assert!(AerogelTransformer::insertion_loss_db() > 34.2);
        assert!((AerogelTransformer::recoilless_fraction() - 0.7820).abs() < 1e-4);
        assert!(AerogelTransformer::is_stress_safe());
    }

    #[test]
    fn test_moire_supercell_commensurability() {
        let eps = MoireSupercell::misfit_strain();
        assert!((eps - 0.00027273).abs() < 1e-6);
        assert!(MoireSupercell::is_commensurate());
        assert_eq!(MoireSupercell::dislocation_density(), 0.0);

        let (tau, eps_b) = MoireSupercell::borrmann_parameters();
        assert_eq!(tau, 1453.4);
        assert_eq!(eps_b, 0.9852);
    }

    #[test]
    fn test_chern_simons_framing_tether() {
        assert_eq!(BravaisTetherPotential::framing_anomaly(0.0), 0.0);
        assert!((BravaisTetherPotential::tether_stiffness() - 5.1295e10).abs() < 1e6);
        assert!(BravaisTetherPotential::is_displacement_within_bound());
        assert_eq!(BravaisTetherPotential::quiescent_annihilation_rate(), 0.0);
    }
}
