//! shbt-recon-thermo — Kapitza cryo boundary resistance, LANR power plant
//! ledger, Landauer GET accounting, and 5th-Gen `SHBT-MAR-312X` sensible
//! heat suppression & superhydride thermal margin audit.
//!
//! - `cryo`:     `CryoCoolingModel` / `KapitzaInterfaceLayer` (shbt-exotic)
//! - `lanr`:     `LANRPowerLedger` 1,800-module plant (shbt-cf)
//! - `landauer`: `LandauerGetCalculator` GET/erasure costs (shbt-precision)

pub mod cryo;
pub mod landauer;
pub mod lanr;

/// 5th-Gen `SHBT-MAR-312X` 12-Layer Fracton-Symplectic Sensible Heating Fraction (`2.13039486e-9`).
pub const MAR312X_SENSIBLE_HEAT_FRACTION: f64 = 2.13039486e-9;
/// `SHBT-MAR-312C` Sensible Heating Fraction (`7.88246e-5`).
pub const MAR312C_SENSIBLE_HEAT_FRACTION: f64 = 7.88246e-5;
/// Sensible heat suppression improvement factor (`37,016.7x` / `37,000x` nominal).
pub const MAR312X_SENSIBLE_HEAT_SUPPRESSION_FACTOR: f64 = 37_016.7;
/// Tethered `LaBeH8-CaH6-12C` Clathrate Superhydride critical temperature (`192.50 K`).
pub const MAR312X_SUPERHYDRIDE_TCRIT_K: f64 = 192.50;
/// Peak operating temperature at Tier 4 `25.00 EW` Planetary Forge (`24.850 K`).
pub const MAR312X_TIER4_OPERATING_TEMP_K: f64 = 24.850;

/// Evaluates the thermal headroom (`167.65 K`) of `SHBT-MAR-312X` at Tier 4 `25.00 EW`.
pub const fn mar312x_tier4_thermal_headroom_k() -> f64 {
    MAR312X_SUPERHYDRIDE_TCRIT_K - MAR312X_TIER4_OPERATING_TEMP_K
}
