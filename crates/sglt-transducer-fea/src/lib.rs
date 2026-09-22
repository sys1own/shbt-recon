//! sglt-transducer-fea: two-phase liquid-to-gas Helium-4 nucleate boiling
//! kinetics, multigigawatt thermomechanics, CVD Diamond-on-GaN substrate,
//! NbN/MgB₂ superconducting routing, and quarter-wave acoustic tamping.

pub mod boiling;
pub mod debye;
pub mod tamping;

/// Saturated He-4 bath temperature (K).
pub const T_BATH_K: f64 = 4.20;
/// Hard peak-temperature ceiling enforced by the phonon shunt (K).
pub const T_PEAK_LIMIT_K: f64 = 4.2100;
/// NbN critical temperature (K).
pub const NBN_TC_K: f64 = 16.0;
/// MgB₂ critical temperature (K).
pub const MGB2_TC_K: f64 = 39.0;
/// CVD diamond thermal conductivity floor (W/m·K).
pub const K_DIAMOND_MIN: f64 = 2000.0;
/// Rated field-collapse transient power (GW).
pub const P_TRANSIENT_GW: f64 = 1.4208;
/// Sustainable transient measured by the solver (GW).
pub const P_TRANSIENT_MAX_GW: f64 = 1.4500;
