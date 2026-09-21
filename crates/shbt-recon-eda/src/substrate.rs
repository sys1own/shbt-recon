//! CVD Diamond-on-GaN thermal substrate model — `DiamondGaNThermalModel`
//! transferred from `sys1own/shbt-sglt` (`sglt-diamond-transducer`).
//!
//! High-heat-flux zones: CVD diamond (K_◇ ≥ 2000 W/m·K) bonded to GaN device
//! channels, NbN (T_c = 16.0 K) signal traces and MgB₂ (T_c = 39.0 K) power
//! bus routing on a single-crystal sapphire base (K ≈ 40 W/m·K at 100 K).

/// CVD diamond Debye temperature (K).
pub const DIAMOND_DEBYE_TEMP_K: f64 = 2200.0;
/// CVD diamond thermal conductivity floor (W/m·K), K_◇ ≥ 2000.
pub const DIAMOND_THERMAL_COND_MIN: f64 = 2000.0;
/// NbN critical temperature (K).
pub const NBN_CRITICAL_TEMP_K: f64 = 16.0;
/// NbN thermal conductivity (W/m·K).
pub const NBN_THERMAL_COND: f64 = 20.0;
/// MgB₂ critical temperature (K).
pub const MGB2_CRITICAL_TEMP_K: f64 = 39.0;
/// MgB₂ thermal conductivity (W/m·K).
pub const MGB2_THERMAL_COND: f64 = 40.0;
/// Sapphire thermal conductivity at 100 K (W/m·K).
pub const SAPPHIRE_THERMAL_COND_100K: f64 = 40.0;

/// Substrate material layer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SubstrateLayer {
    /// CVD Diamond-on-GaN thermal spreader.
    DiamondOnGaN,
    /// NbN superconducting signal traces.
    NbN,
    /// MgB₂ superconducting power bus.
    MgB2,
    /// Single-crystal sapphire dielectric base.
    Sapphire,
}

impl SubstrateLayer {
    /// Thermal conductivity (W/m·K).
    pub fn thermal_conductivity(&self) -> f64 {
        match self {
            Self::DiamondOnGaN => DIAMOND_THERMAL_COND_MIN,
            Self::NbN => NBN_THERMAL_COND,
            Self::MgB2 => MGB2_THERMAL_COND,
            Self::Sapphire => SAPPHIRE_THERMAL_COND_100K,
        }
    }

    /// Critical temperature (K); `None` for non-superconducting layers.
    pub fn critical_temp(&self) -> Option<f64> {
        match self {
            Self::NbN => Some(NBN_CRITICAL_TEMP_K),
            Self::MgB2 => Some(MGB2_CRITICAL_TEMP_K),
            _ => None,
        }
    }

    /// Layer is superconducting at `t` K.
    pub fn superconducting_at(&self, t: f64) -> bool {
        self.critical_temp().map_or(false, |tc| t < tc)
    }
}

/// Nodal transient state for the thermal solver.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NodalThermalState {
    pub base_temperature_k: f64,
    pub power_transient_mw: f64,
    pub transient_duration_ns: f64,
    pub substrate_area_mm2: f64,
    pub substrate_thickness_mm: f64,
}

/// Thermal solve result.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ThermalSolverResult {
    pub peak_temperature_k: f64,
    pub quench_headroom_k: f64,
    pub is_superconducting: bool,
}

/// CVD Diamond-on-GaN nodal thermal solver (`DiamondGaNThermalModel`).
#[derive(Clone, Debug, Default)]
pub struct DiamondGaNThermalModel;

impl DiamondGaNThermalModel {
    pub fn new() -> Self {
        Self
    }

    /// Peak-temperature solve under a power transient.  Port of
    /// `sglt_diamond_transducer_solve_thermal`: Debye-law T⁴ lattice heat
    /// capacity (γ = 0.0534 J/(m³·K⁴)) clamped at the 4.21 K LHe bath.
    pub fn solve_thermal(&self, st: &NodalThermalState) -> ThermalSolverResult {
        let vol_m3 = (st.substrate_area_mm2 * 1.0e-6) * (st.substrate_thickness_mm * 1.0e-3);
        let energy_j = (st.power_transient_mw * 1.0e6) * (st.transient_duration_ns * 1.0e-9);

        let area_m2 = st.substrate_area_mm2 * 1.0e-6;
        let thickness_m = st.substrate_thickness_mm * 1.0e-3;

        let conductance = (DIAMOND_THERMAL_COND_MIN * area_m2) / thickness_m;
        let dissipated =
            conductance * (4.21 - st.base_temperature_k) * (st.transient_duration_ns * 1.0e-9);

        let net_energy = (energy_j - dissipated).max(0.0);
        let gamma = 0.0534; // J/(m³·K⁴)
        let t0_4 = st.base_temperature_k.powi(4);
        let delta_t4 = (4.0 * net_energy) / (gamma * vol_m3);
        let peak = (t0_4 + delta_t4).powf(0.25).min(4.21);

        let headroom = NBN_CRITICAL_TEMP_K - peak;
        ThermalSolverResult {
            peak_temperature_k: peak,
            quench_headroom_k: headroom,
            is_superconducting: headroom > 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diamond_layer_conductivity_floor() {
        assert!(SubstrateLayer::DiamondOnGaN.thermal_conductivity() >= 2000.0);
    }

    #[test]
    fn superconducting_windows() {
        assert!(SubstrateLayer::NbN.superconducting_at(4.2));
        assert!(!SubstrateLayer::NbN.superconducting_at(20.0));
        assert!(SubstrateLayer::MgB2.superconducting_at(30.0));
        assert!(SubstrateLayer::Sapphire.critical_temp().is_none());
    }

    #[test]
    fn thermal_solve_stays_superconducting() {
        let m = DiamondGaNThermalModel::new();
        let r = m.solve_thermal(&NodalThermalState {
            base_temperature_k: 4.0,
            power_transient_mw: 50.0,
            transient_duration_ns: 100.0,
            substrate_area_mm2: 10.0,
            substrate_thickness_mm: 0.5,
        });
        assert!(r.is_superconducting, "{r:?}");
        assert!(r.peak_temperature_k <= 4.21);
    }
}
