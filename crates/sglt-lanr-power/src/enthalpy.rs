//! Enthalpy balance ledger for synthetic matter re-rendering.
//!
//! Tracks nuclear binding-energy differentials (ΔB_nuc) and chemical
//! formation enthalpy (ΔH_form) for each synthesis run, and prices the
//! Landauer configurational-entropy cost of mapping disordered feedstock
//! Ω_feedstock onto ordered target Ω_target:
//!
//!   P_Landauer_synth = k_B T_base ln(2) · Ṅ_atoms · ln(Ω_feedstock/Ω_target)/ln(2)
//!
//! Power balance: the 1,800-module LANR array covers the 906.00 kW baseline
//! entropy debt continuously; endothermic burst injection (up to the
//! 49.9449 TW net graser rail) is drawn from the 500.0 TJ isomer battery.

use crate::isomer;
use crate::ARRAY_NET_KW;

/// Boltzmann constant (J/K).
pub const K_B: f64 = 1.380649e-23;
/// Cryostat base temperature during synthesis (K).
pub const T_BASE_K: f64 = 21.13;
/// Continuous entropy-debt floor covered by LANR (W): 906.00 kW.
pub const BASELINE_DEBT_W: f64 = 906.00e3;
/// Peak net burst injection available from the isomer rail (W): 49.9449 TW.
pub const MAX_BURST_INJECTION_W: f64 = isomer::NET_BURST_POWER_W;

/// Per-run enthalpy ledger entry.
#[derive(Clone, Copy, Debug)]
pub struct EnthalpyBalanceLedger {
    /// Nuclear binding-energy differential ΔB_nuc (J for the whole run).
    pub binding_delta_j: f64,
    /// Chemical formation enthalpy ΔH_form (J for the whole run).
    pub formation_delta_j: f64,
    /// Landauer configurational-entropy cost (J).
    pub landauer_cost_j: f64,
    /// Atom count processed (Ṅ_atoms · τ).
    pub atoms_processed: f64,
    /// Run duration (s).
    pub duration_s: f64,
}

impl EnthalpyBalanceLedger {
    /// Build a ledger for `atom_count` atoms of a target product over
    /// `duration_s` seconds.
    ///
    /// * `binding_delta_mev` — ΔB_nuc per nucleus (MeV).
    /// * `formation_kj_mol`  — ΔH_form per mole (kJ/mol; < 0 exothermic).
    /// * `omega_ratio`       — Ω_feedstock / Ω_target (≥ 1 for purification).
    pub fn new(
        atom_count: f64,
        duration_s: f64,
        binding_delta_mev: f64,
        formation_kj_mol: f64,
        omega_ratio: f64,
    ) -> Self {
        const MEV_J: f64 = 1.602176634e-13;
        const AVOGADRO: f64 = 6.02214076e23;
        let binding_delta_j = binding_delta_mev * MEV_J * atom_count;
        let formation_delta_j = formation_kj_mol * 1e3 * atom_count / AVOGADRO;
        let landauer_cost_j = landauer_cost(atom_count, duration_s, omega_ratio) * duration_s;
        Self {
            binding_delta_j,
            formation_delta_j,
            landauer_cost_j,
            atoms_processed: atom_count,
            duration_s,
        }
    }

    /// Net endothermic power demand during the run (W).  Positive means the
    /// run draws power from the isomer burst rail; negative means it returns
    /// energy via the DEC capture channel.
    pub fn net_power_demand_w(&self) -> f64 {
        (self.binding_delta_j + self.formation_delta_j + self.landauer_cost_j)
            / self.duration_s
    }

    /// First-law residual ΔE_net = (ΔB + ΔH + Landauer) − supplied energy.
    /// Zero when the routed supply exactly covers the ledger.
    pub fn first_law_residual_j(&self, supplied_j: f64) -> f64 {
        self.binding_delta_j + self.formation_delta_j + self.landauer_cost_j
            - supplied_j
    }

    /// True when the run is serviceable: baseline covered by LANR continuous
    /// output and any burst component fits the 49.9449 TW isomer rail.
    pub fn power_balance_ok(&self) -> bool {
        let demand = self.net_power_demand_w();
        if demand <= 0.0 {
            // Exothermic: DEC returns energy; LANR still covers the floor.
            return ARRAY_NET_KW * 1e3 >= BASELINE_DEBT_W;
        }
        demand <= BASELINE_DEBT_W + MAX_BURST_INJECTION_W
    }
}

/// Landauer configurational-entropy power (W):
///   P = k_B T_base ln 2 · Ṅ_atoms · log2(Ω_feedstock / Ω_target).
pub fn landauer_cost(atom_rate_per_s: f64, _duration_s: f64, omega_ratio: f64) -> f64 {
    if omega_ratio <= 1.0 || atom_rate_per_s <= 0.0 {
        return 0.0;
    }
    K_B * T_BASE_K * std::f64::consts::LN_2 * atom_rate_per_s * omega_ratio.log2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isomer::NET_BURST_POWER_W;

    #[test]
    fn baseline_covered_by_lanr() {
        const { assert!(ARRAY_NET_KW * 1e3 >= BASELINE_DEBT_W) };
    }

    #[test]
    fn burst_headroom() {
        assert!((NET_BURST_POWER_W - 49.9449e12).abs() < 1e9);
    }

    #[test]
    fn endothermic_run_serviceable() {
        // ^178m2HfB2 isomer loading over a 10 ms burst.
        let led = EnthalpyBalanceLedger::new(1.0e24, 1.0e-2, 2.446, -334.0, 1.0);
        assert!(led.net_power_demand_w() > 0.0);
        assert!(led.power_balance_ok());
        assert_eq!(led.first_law_residual_j(
            led.binding_delta_j + led.formation_delta_j + led.landauer_cost_j
        ), 0.0);
    }

    #[test]
    fn landauer_purification_positive() {
        let p = landauer_cost(1.0e24, 1.0, 32.0);
        assert!(p > 0.0);
        // Natural-abundance feedstock (Ω ratio 1) costs nothing.
        assert_eq!(landauer_cost(1.0e24, 1.0, 1.0), 0.0);
    }
}
