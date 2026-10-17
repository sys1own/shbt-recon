//! 1,800-module LANR cold-fusion power plant ledger — `LANRPowerLedger`
//! transferred from `sys1own/shbt-cf` (via `sglt-lanr-power/module_ledger.rs`).
//!
//! Gross output 1,800 × 555.03 W = 999.054 kW against a 906.00 GW→kW-class
//! continuous entropy-debt demand baseline (999.054 kW net, 33.804 % TEG).

/// Number of LANR reactor modules.
pub const MODULE_COUNT: u32 = 1800;
/// Net electrical output per module (W) — includes 94.20 % efficient
/// SiC crowbar magnetic energy recovery reducing parasitic drive load.
pub const MODULE_NET_W: f64 = 555.03;
/// Minimum module operating floor sustaining the 906.00 kW demand:
/// ceil(906,000 W / 555.03 W) = 1,633.
pub const MIN_MODULE_FLOOR: u32 = 1633;
/// Active zero-derating module reserve: 1,800 − 1,633 = 167.
pub const RESERVE_MODULE_COUNT: u32 = 167;
/// Continuous entropy-debt demand (W).
pub const DEMAND_W: f64 = 906.00e3;
/// Quiescent SHBT-MAR-312 TEG recovery (W): 48.000 kW DC from visible host lattice decay.
pub const TEG_QUIESCENT_RECOVERY_W: f64 = 48.000e3;
/// Total continuous non-sheddable station housekeeping load (W):
/// 906 kW (Landauer) + 42.15 kW (cryo) + 12.8 kW (RF/PIC) + 5 kW (controller) = 965.950 kW.
pub const HOUSEKEEPING_TOTAL_LOAD_W: f64 = 965.950e3;
/// Total Tier-1 continuous DC bus generation (W): 999.054 kW + 48.000 kW = 1,047.054 kW.
pub const TIER1_TOTAL_DC_W: f64 = 1047.054e3;
/// Net continuous margin on LANR alone (W): 999.054 kW - 965.950 kW = +33.104 kW.
pub const LANR_ONLY_MARGIN_W: f64 = 33.104e3;
/// Net continuous margin on Tier-1 bus with TEG recovery (W): 1,047.054 kW - 965.950 kW = +81.104 kW.
pub const TIER1_NET_MARGIN_W: f64 = 81.104e3;
/// Thermoelectric generator efficiency.
pub const TEG_EFFICIENCY: f64 = 0.33804;
/// Radiator temperature (K).
pub const RADIATOR_TEMP_K: f64 = 600.0;
/// Normative radiator area (m²).
pub const RADIATOR_AREA_M2: f64 = 688.520;
/// Stefan–Boltzmann constant (W/m²K⁴).
pub const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;
/// Radiator emissivity.
pub const RADIATOR_EMISSIVITY: f64 = 0.92;
/// Maximum survivable module failures (10 % residual power).
pub const K_MAX_SURVIVABLE: u32 = 1607;
/// Solar mass (kg) for the entropy-debt scaling.
pub const SOLAR_MASS_KG: f64 = 1.98892e30;
/// Entropy-debt power scaling P_base = 906.00 GW.
pub const DEBT_BASE_W: f64 = 906.00e9;

/// LANR power plant ledger (`LANRPowerLedger` transfer).
#[derive(Clone, Debug, Default)]
pub struct LanrPowerLedger {
    /// Currently failed modules.
    pub failed_modules: u32,
}

impl LanrPowerLedger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inject `k` module failures.
    pub fn inject_failures(&mut self, k: u32) {
        self.failed_modules = k.min(MODULE_COUNT);
    }

    /// Gross electrical output of surviving modules (W).
    pub fn gross_output_w(&self) -> f64 {
        (MODULE_COUNT - self.failed_modules) as f64 * MODULE_NET_W
    }

    /// Nominal gross output, zero failures (W) — 999.054 kW.
    pub fn nominal_gross_w(&self) -> f64 {
        MODULE_COUNT as f64 * MODULE_NET_W
    }

    /// Net balance: gross − continuous entropy-debt demand (W).
    pub fn net_entropy_balance_w(&self) -> f64 {
        self.gross_output_w() - DEMAND_W
    }

    /// Active reserve-module equivalent (+167 nominal).
    pub fn reserve_modules(&self) -> i64 {
        (self.net_entropy_balance_w() / MODULE_NET_W).floor() as i64
    }

    /// Effective TEG efficiency (nominal 33.804 %).
    pub fn teg_efficiency(&self) -> f64 {
        TEG_EFFICIENCY * (1.0 + 1.2e-4 * (self.failed_modules as f64 / MODULE_COUNT as f64))
    }

    /// Required radiator area (m²) at 600 K: A = Q/(εσT⁴).
    pub fn radiator_area_m2(&self, rejected_w: f64) -> f64 {
        rejected_w / (RADIATOR_EMISSIVITY * STEFAN_BOLTZMANN * RADIATOR_TEMP_K.powi(4))
    }

    /// Rejected-heat load (W).
    pub fn rejected_heat_w(&self) -> f64 {
        let electrical = self.gross_output_w();
        electrical / TEG_EFFICIENCY - electrical
    }

    /// Entropy-debt draw for a translocation payload:
    /// P_debt = (M_payload / M_⊙) · 906 GW.
    pub fn entropy_debt_w(&self, payload_mass_kg: f64) -> f64 {
        payload_mass_kg / SOLAR_MASS_KG * DEBT_BASE_W
    }

    /// True while the array still meets demand.
    pub fn meets_demand(&self) -> bool {
        self.gross_output_w() >= DEMAND_W
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominal_gross_is_999kw() {
        let l = LanrPowerLedger::new();
        assert!((l.nominal_gross_w() - 999.054e3).abs() < 5.0);
    }

    #[test]
    fn net_margin_and_reserves() {
        let l = LanrPowerLedger::new();
        assert!((l.net_entropy_balance_w() - 93.054e3).abs() < 10.0);
        assert_eq!(l.reserve_modules(), 167);
        assert_eq!(MIN_MODULE_FLOOR, 1633);
        assert_eq!(RESERVE_MODULE_COUNT, 167);
        assert!((l.teg_efficiency() - 0.33804).abs() < 1e-6);
        assert!((TIER1_TOTAL_DC_W - 1047054.0).abs() < 1e-3);
        assert!((LANR_ONLY_MARGIN_W - 33104.0).abs() < 1e-3);
        assert!((TIER1_NET_MARGIN_W - 81104.0).abs() < 1e-3);
    }

    #[test]
    fn k1607_survivable_limit() {
        let mut l = LanrPowerLedger::new();
        l.inject_failures(K_MAX_SURVIVABLE);
        let residual = l.gross_output_w() / l.nominal_gross_w();
        assert!((residual - 193.0 / 1800.0).abs() < 1e-9);
    }

    #[test]
    fn entropy_debt_scales_with_mass() {
        let l = LanrPowerLedger::new();
        let p = l.entropy_debt_w(SOLAR_MASS_KG);
        assert!((p - DEBT_BASE_W).abs() < 1e-3);
    }
}
