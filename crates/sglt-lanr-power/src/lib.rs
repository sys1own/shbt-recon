//! sglt-lanr-power: 1,800-module LANR cold-fusion power plant grid ledger
//! and entropy-debt power balancing for the translocator swarm, plus the
//! pulsed ^178m2Hf isomer graser burst rail (upstream: sys1own/shbt-warp).

pub mod compacted_nacelle;
pub mod dec_transient;
pub mod enthalpy;
pub mod fp_closure;
pub mod isomer;
pub mod mar312x;
pub mod shunt;

pub use compacted_nacelle::*;
pub use dec_transient::*;
pub use fp_closure::*;
pub use mar312x::*;
pub use shunt::*;

/// Active LANR module count.
pub const MODULE_COUNT: u32 = 1800;
/// Single-module thermal output (kW): nickel–hydrogen exothermic core.
pub const MODULE_THERMAL_KW: f64 = 1.50072;
/// Solid-state TEG conversion efficiency.
pub const TEG_EFFICIENCY: f64 = 0.33804;
/// Per-module net electrical output (W) — 94.20 % efficient SiC crowbar
/// magnetic energy recovery reduces the parasitic drive load to 490.55 W.
pub const MODULE_NET_W: f64 = 555.03;
/// Per-module parasitic drive load after crowbar recovery (W).
pub const MODULE_PARASITIC_W: f64 = 490.55;
/// Entropy-debt demand per solar-mass payload (GW/M_☉).
pub const DEBT_BASE_GW: f64 = 906.0;
/// Nominal array net electrical output (kW).
pub const ARRAY_NET_KW: f64 = 999.054;
/// Quiescent SHBT-MAR-312 TEG recovery (kW): spontaneous decay of the visible
/// 178m2HfB2 host lattice (354.27 kW_th at eta_TEG = 13.55%).
pub const TEG_QUIESCENT_RECOVERY_KW: f64 = 48.000;
/// Total Tier-1 continuous DC bus generation (kW): 999.054 + 48.000 = 1,047.054 kW.
pub const TIER1_TOTAL_DC_KW: f64 = ARRAY_NET_KW + TEG_QUIESCENT_RECOVERY_KW;
/// Total continuous non-sheddable station housekeeping load (kW):
/// 906.000 kW (Landauer floor) + 42.150 kW (LHe cryo) + 12.800 kW (RF/PIC) + 5.000 kW (controller) = 965.950 kW.
pub const HOUSEKEEPING_TOTAL_LOAD_KW: f64 = 965.950;
/// Net continuous margin on LANR alone (kW): 999.054 - 965.950 = +33.104 kW.
pub const LANR_ONLY_MARGIN_KW: f64 = ARRAY_NET_KW - HOUSEKEEPING_TOTAL_LOAD_KW;
/// Net continuous margin on Tier-1 bus with TEG recovery (kW): 1,047.054 - 965.950 = +81.104 kW.
pub const TIER1_NET_MARGIN_KW: f64 = TIER1_TOTAL_DC_KW - HOUSEKEEPING_TOTAL_LOAD_KW;
/// Minimum module operating floor sustaining the 906.00 kW non-sheddable
/// entropy-debt threshold: ceil(906,000 W / 555.03 W).
pub const MIN_MODULE_FLOOR: u32 = 1633;
/// Active zero-derating module reserve: 1,800 − 1,633.
pub const RESERVE_MODULE_COUNT: u32 = 167;

/// Aggregate LANR plant ledger.
#[derive(Clone, Copy, Debug)]
pub struct LanrPlant {
    pub active_modules: u32,
}

impl LanrPlant {
    pub fn new() -> Self {
        Self {
            active_modules: MODULE_COUNT,
        }
    }

    /// Gross thermal output of the module array (MW).
    pub fn total_thermal_mw(&self) -> f64 {
        self.active_modules as f64 * MODULE_THERMAL_KW / 1e3
    }

    /// Net electrical output routed to swarm optics (kW).
    pub fn net_electrical_kw(&self) -> f64 {
        self.active_modules as f64 * MODULE_NET_W / 1e3
    }

    /// Entropy-debt power demand for a payload of mass m (W).
    pub fn entropy_debt_w(&self, payload_mass_kg: f64) -> f64 {
        payload_mass_kg / shbt_recon_thermo::lanr::SOLAR_MASS_KG * DEBT_BASE_GW * 1e9
    }

    /// True when the plant covers the N-modular demand (shbt DEMAND_W).
    pub fn meets_demand(&self) -> bool {
        self.net_electrical_kw() * 1e3 >= shbt_recon_thermo::lanr::DEMAND_W
    }
}

impl Default for LanrPlant {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_ledger_balances() {
        let p = LanrPlant::new();
        assert!((p.total_thermal_mw() - 2.7013).abs() < 1e-3);
        assert!((p.net_electrical_kw() - ARRAY_NET_KW).abs() < 0.01);
        assert!(p.meets_demand());
        assert_eq!(MODULE_COUNT, 1800);
        assert!((p.net_electrical_kw() - 999.054).abs() < 1e-3);
        assert_eq!(MIN_MODULE_FLOOR, 1633);
        assert_eq!(RESERVE_MODULE_COUNT, 167);
        assert_eq!(MODULE_COUNT - MIN_MODULE_FLOOR, RESERVE_MODULE_COUNT);
        assert!((TIER1_TOTAL_DC_KW - 1047.054).abs() < 1e-6);
        assert!((LANR_ONLY_MARGIN_KW - 33.104).abs() < 1e-6);
        assert!((TIER1_NET_MARGIN_KW - 81.104).abs() < 1e-6);
    }

    #[test]
    fn entropy_debt_scaling() {
        let p = LanrPlant::new();
        let one_solar = p.entropy_debt_w(shbt_recon_thermo::lanr::SOLAR_MASS_KG);
        assert!((one_solar - 906.0e9).abs() < 1.0);
    }
}
