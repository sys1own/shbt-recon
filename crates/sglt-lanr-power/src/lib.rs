//! sglt-lanr-power: 1,800-module LANR cold-fusion power plant grid ledger
//! and entropy-debt power balancing for the translocator swarm.

/// Active LANR module count.
pub const MODULE_COUNT: u32 = 1800;
/// Single-module thermal output (kW): nickel–hydrogen exothermic core.
pub const MODULE_THERMAL_KW: f64 = 1.50072;
/// Solid-state TEG conversion efficiency.
pub const TEG_EFFICIENCY: f64 = 0.33804;
/// Per-module net electrical output (W).
pub const MODULE_NET_W: f64 = 507.32;
/// Entropy-debt demand per solar-mass payload (GW/M_☉).
pub const DEBT_BASE_GW: f64 = 906.0;
/// Nominal array net electrical output (kW).
pub const ARRAY_NET_KW: f64 = 913.18;

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
    }

    #[test]
    fn entropy_debt_scaling() {
        let p = LanrPlant::new();
        let one_solar = p.entropy_debt_w(shbt_recon_thermo::lanr::SOLAR_MASS_KG);
        assert!((one_solar - 906.0e9).abs() < 1.0);
    }
}
