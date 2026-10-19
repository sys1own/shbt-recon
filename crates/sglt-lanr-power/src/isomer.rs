//! 4th-Generation Stinespring-so(10)_312 Mass-Annihilation Battery-Reactor (SHBT-MAR-312).
//!
//! Replaces the legacy standalone graser power supply with the monolithic 376.99 kg
//! bipartite fuel core (33.8822 EJ gross / 30.3310 EJ net electrical via 5-stage
//! relativistic Direct Energy Conversion at eta_DEC = 89.519%, 0.015-2.700 MV HVDC).
//!
//! Physical Architecture:
//! - Visible Sector (eta_A = 10/33, 188.495 kg): 90.89 wt% ^178m2HfB2 (171.323 kg
//!   containing 5.16811e26 ^178Hf nuclei at K^pi = 16+, E_x = +2.446 MeV, t_1/2 = 31.0 yr,
//!   E_isomer = 0.20253 PJ) + 9.11 wt% ^12C CVD diamond (17.172 kg). Serves as the internal
//!   Borrmann optical trigger gateway (40.0 keV seed laser, G = 61.15, f_M = 0.782, eps_B = 0.9852).
//! - Stinespring Dark Sector (eta_D = 23/33, 188.495 kg): D_Bbar-folded 16-bar anti-baryons
//!   (33.8822 EJ gross) in quiescent stasis (C_Bbar == 0, Gamma == 0 s^-1) until optical trigger.
//! - 5-Stage Relativistic DEC: 89.519% net efficiency yielding 30.3310 EJ net electric.
//! - Three Dispatch Modes: Cruise (895.19 TW), Burst (6.805 PW), Impulse (895.19 PW).

/// Total stored annihilation energy in the monolithic SHBT-MAR-312 core (J): M_core * c^2 = 33.8822 EJ.
pub const MAR312_TOTAL_ENERGY_JOULES: f64 = 33.882173e18;
/// Total stored energy legacy alias (J).
pub const ISOMER_TOTAL_ENERGY_JOULES: f64 = MAR312_TOTAL_ENERGY_JOULES;
/// Active monolithic core mass (kg): 188.495 kg visible + 188.495 kg folded anti-baryons.
pub const MAR312_CORE_MASS_KG: f64 = 376.99;
/// Active core mass alias (kg).
pub const ISOMER_CORE_MASS_KG: f64 = MAR312_CORE_MASS_KG;
/// Visible sector mass (kg).
pub const MAR312_VISIBLE_MASS_KG: f64 = 188.495;
/// Stinespring folded anti-baryon sector mass (kg).
pub const MAR312_ANTIBARYON_MASS_KG: f64 = 188.495;
/// Core specific energy density (J/kg): c^2 = 8.987551787368176e16 J/kg = 89.8755 PJ/kg.
pub const MAR312_SPECIFIC_ENERGY_J_KG: f64 = 8.987_551_787_368_176e16;
/// Core specific energy density alias (J/kg).
pub const SPECIFIC_ENERGY_JOULES_PER_KG: f64 = MAR312_SPECIFIC_ENERGY_J_KG;

/// 5-stage relativistic DEC stage efficiencies:
/// Stage 1: Relativistic Forward-Compton Photo-Converter (eta_1 = 26.400%)
pub const DEC_STAGE1_COMPTON: f64 = 0.26400;
/// Stage 2: Bethe-Heitler Pair-Induction Matrix (eta_2 = 12.100%)
pub const DEC_STAGE2_PAIR: f64 = 0.12100;
/// Stage 3: High-beta Resistive MHD Magnetic Flux-Compression Solenoid (eta_3 = 29.800%)
pub const DEC_STAGE3_MHD: f64 = 0.29800;
pub const DEC_STAGE3_RETARDING: f64 = DEC_STAGE3_MHD; // compatibility alias
/// Stage 4A: Low-Z Beryllium Epithermal Retarding Grids (eta_4A = 7.300%)
pub const DEC_STAGE4A_RETARDING: f64 = 0.07300;
pub const DEC_STAGE4_MHD: f64 = DEC_STAGE4A_RETARDING; // compatibility alias
/// Stage 4B: Tri-Rail Venetian-Blind MV Electrostatic Collector + Sternglass-Joy Grid (eta_4B = 13.919%)
pub const DEC_STAGE4B_COLLECTOR: f64 = 0.13919;
pub const DEC_STAGE5_COLLECTOR: f64 = DEC_STAGE4B_COLLECTOR; // compatibility alias
/// Cascaded 5-stage DEC conversion efficiency: 89.519%.
pub const MAR312_DEC_EFFICIENCY: f64 =
    DEC_STAGE1_COMPTON + DEC_STAGE2_PAIR + DEC_STAGE3_MHD + DEC_STAGE4A_RETARDING + DEC_STAGE4B_COLLECTOR;
/// Total DEC efficiency alias.
pub const DEC_TOTAL_EFFICIENCY: f64 = MAR312_DEC_EFFICIENCY;

/// Net electrical energy delivered after 5-stage DEC (J): 30.3310 EJ (30,331.0 PJ).
pub const MAR312_NET_ELECTRICAL_ENERGY_JOULES: f64 =
    MAR312_TOTAL_ENERGY_JOULES * MAR312_DEC_EFFICIENCY;

/// Resonant gateway X-ray trigger seed energy (keV).
pub const GATEWAY_TRIGGER_ENERGY_KEV: f64 = 40.0;
/// Visible isomer excitation energy release per nucleus (keV): E_x = 2.446 MeV.
pub const ISOMER_ENERGY_RELEASE_KEV: f64 = 2446.0;
/// Breit-Wigner resonant excitation cross section (cm^2).
pub const BREIT_WIGNER_SIGMA_CM2: f64 = 1.48e-21;
/// Resonant gateway trigger gain: 2446.0 / 40.0 = 61.15.
pub const ISOMER_GATEWAY_GAIN: f64 =
    ISOMER_ENERGY_RELEASE_KEV / GATEWAY_TRIGGER_ENERGY_KEV;
/// Visible isomer half-life (years).
pub const ISOMER_HALF_LIFE_YEARS: f64 = 31.0;
/// Borrmann anomalous transmission factor at T <= 21.13 K.
pub const BORRMANN_EPSILON: f64 = 0.9852;
/// Mossbauer recoilless fraction at T <= 21.13 K.
pub const MOSSBAUER_FRACTION: f64 = 0.782;

/// Three canonical dispatch modes:
/// 1. CruiseMode: 1.000 PW gross -> 895.19 TW net electric (8.379e15 bps).
pub const CRUISE_GROSS_POWER_W: f64 = 1.000e15;
pub const CRUISE_NET_POWER_W: f64 = CRUISE_GROSS_POWER_W * MAR312_DEC_EFFICIENCY; // 895.19 TW
/// 2. BurstMode: 7.602 PW gross -> 6.805 PW net electric (6.370e16 bps).
pub const BURST_GROSS_POWER_W: f64 = 7.602e15;
pub const BURST_NET_POWER_W: f64 = BURST_GROSS_POWER_W * MAR312_DEC_EFFICIENCY; // 6.805 PW
/// 3. ImpulseMode: 1.000 EW gross -> 895.19 PW net electric (8.379e18 bps).
pub const IMPULSE_GROSS_POWER_W: f64 = 1.000e18;
pub const IMPULSE_NET_POWER_W: f64 = IMPULSE_GROSS_POWER_W * MAR312_DEC_EFFICIENCY; // 895.19 PW

/// Nominal gross burst power alias (W): 7.602 PW.
pub const GROSS_BURST_POWER_W: f64 = BURST_GROSS_POWER_W;
/// Net electrical burst power alias (W): 6.805 PW.
pub const NET_BURST_POWER_W: f64 = BURST_NET_POWER_W;

/// Fast PCSS optical crowbar quench latency bound (s): 2.18 ns.
pub const CROWBAR_QUENCH_TIME_SECONDS: f64 = 2.18e-9;
/// Fraction of interrupted reactive energy recovered into SMES coils (94.20%).
pub const SMES_RECOVERY_FRACTION: f64 = 0.9420;
/// Fraction shunted to external thermal dumps (5.80%).
pub const WC_DUMP_FRACTION: f64 = 1.0 - SMES_RECOVERY_FRACTION;
/// Sensible lattice heating fraction under Heegaard-Floer dark-ledger shunting:
/// f_sensible = (1 - 0.9420) * (1 - 0.9852) * (100 / 1089) = 7.88246e-5.
pub const SENSIBLE_HEAT_FRACTION: f64 =
    (1.0 - SMES_RECOVERY_FRACTION) * (1.0 - BORRMANN_EPSILON) * (100.0 / 1089.0);

/// DEC output bus voltage low/high rails (V): 15 kV to 2.700 MV DC.
pub const DEC_BUS_LOW_V: f64 = 15.0e3;
pub const DEC_BUS_HIGH_V: f64 = 2.700e6;

/// Operating temperature bounds:
pub const T_STANDBY_K: f64 = 21.13;
pub const T_PEAK_BURST_K: f64 = 28.40;
pub const T_CEILING_K: f64 = 32.92;
pub const SIGMA_MAX_MPA: f64 = 124.60;
pub const SIGMA_YIELD_MPA: f64 = 350.00;

/// 3 canonical dispatch modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DispatchMode {
    Cruise,
    Burst,
    Impulse,
}

impl DispatchMode {
    pub fn gross_power_w(&self) -> f64 {
        match self {
            Self::Cruise => CRUISE_GROSS_POWER_W,
            Self::Burst => BURST_GROSS_POWER_W,
            Self::Impulse => IMPULSE_GROSS_POWER_W,
        }
    }

    pub fn net_power_w(&self) -> f64 {
        match self {
            Self::Cruise => CRUISE_NET_POWER_W,
            Self::Burst => BURST_NET_POWER_W,
            Self::Impulse => IMPULSE_NET_POWER_W,
        }
    }
}

use crate::compacted_nacelle::{
    DarkBraidMagazine, ReactorNacelleMode,
    AUTO_HOT_SWAP_SOC_THRESHOLD, HOT_SWAP_MICRO_BATCH_LATENCY_S,
    MAR312C_SEED_GROSS_EJ, MAR312C_SEED_NET_EJ,
};

/// 6-phase SHBT-MAR-312 dispatch state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DispatchPhase {
    /// 0x01 — quiescent baseload; LANR rail only.
    StandbyStasis = 0x01,
    /// 0x02 — gateway trigger armed, awaiting fold authorization.
    TriggerArmed = 0x02,
    /// 0x04 — active mass-annihilation burst / character transmutation discharge.
    FoldingBurst = 0x04,
    /// 0x06 — bipartite coset spinor anti-baryon folding.
    CosetAntibaryonFold = 0x06,
    /// 0x08 — symplectic relabeling cooldown / symplectic crystallization.
    SymplecticCooldown = 0x08,
    /// 0x10 — PCSS crowbar tripped; output clamped to 0 W.
    EmergencyQuench = 0x10,
}

/// MAR-312 battery-reactor state: state-of-charge plus dispatch interlocks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsomerBatteryState {
    /// Remaining stored energy (J) out of `MAR312_TOTAL_ENERGY_JOULES`.
    pub energy_remaining_joules: f64,
    /// Resonant gateway trigger armed.
    pub is_armed: bool,
    /// PCSS optical crowbar tripped (hard lockout).
    pub is_crowbar_tripped: bool,
    /// Current dispatch phase.
    pub phase: DispatchPhase,
    /// Selected power mode.
    pub mode: DispatchMode,
    /// Active reactor nacelle configuration mode.
    pub nacelle_mode: ReactorNacelleMode,
    /// Active seed wafer state-of-charge (0.0 to 1.0, 3.03310 EJ net per 37.699 kg wafer).
    pub active_wafer_soc: f64,
    /// Stored reserve Core Equivalent Units in anyon braid magazine (1.0 CEU = 10 wafer refills).
    pub magazine_remaining_ceu: f64,
    /// Active dark-sector anyon braid reload magazine instance (Tier 1, 2, or 3).
    pub dark_magazine: Option<DarkBraidMagazine>,
    /// Total autonomous hot-swap refills executed.
    pub total_hot_swaps: u64,
    /// Latency of the most recent hot-swap reload (seconds).
    pub last_swap_latency_s: f64,
}

pub type Mar312BatteryState = IsomerBatteryState;

impl IsomerBatteryState {
    /// Default compacted catalytic wafer reactor in standby stasis (Tier 1 Magazine).
    pub fn new() -> Self {
        Self {
            energy_remaining_joules: MAR312_TOTAL_ENERGY_JOULES,
            is_armed: false,
            is_crowbar_tripped: false,
            phase: DispatchPhase::StandbyStasis,
            mode: DispatchMode::Cruise,
            nacelle_mode: ReactorNacelleMode::CompactedWaferMar312cIbr,
            active_wafer_soc: 1.0,
            magazine_remaining_ceu: 38.0,
            dark_magazine: Some(DarkBraidMagazine::new_tier1()),
            total_hot_swaps: 0,
            last_swap_latency_s: 0.0,
        }
    }

    /// Stationary monolithic core configuration baseline (4,096.99 kg nacelle).
    pub fn new_stationary() -> Self {
        Self {
            energy_remaining_joules: MAR312_TOTAL_ENERGY_JOULES,
            is_armed: false,
            is_crowbar_tripped: false,
            phase: DispatchPhase::StandbyStasis,
            mode: DispatchMode::Cruise,
            nacelle_mode: ReactorNacelleMode::StationaryMonolithicMar312,
            active_wafer_soc: 1.0,
            magazine_remaining_ceu: 0.0,
            dark_magazine: None,
            total_hot_swaps: 0,
            last_swap_latency_s: 0.0,
        }
    }

    /// Compacted catalytic wafer reactor with specified dark magazine tier.
    pub fn new_compacted(tier: usize) -> Self {
        let mag = match tier {
            2 => DarkBraidMagazine::new_tier2(16),
            3 => DarkBraidMagazine::new_tier3(106611.0),
            _ => DarkBraidMagazine::new_tier1(),
        };
        let ceu = mag.total_cores_stored;
        Self {
            energy_remaining_joules: MAR312_TOTAL_ENERGY_JOULES,
            is_armed: false,
            is_crowbar_tripped: false,
            phase: DispatchPhase::StandbyStasis,
            mode: DispatchMode::Cruise,
            nacelle_mode: ReactorNacelleMode::CompactedWaferMar312cIbr,
            active_wafer_soc: 1.0,
            magazine_remaining_ceu: ceu,
            dark_magazine: Some(mag),
            total_hot_swaps: 0,
            last_swap_latency_s: 0.0,
        }
    }

    /// State-of-charge fraction (0.0 to 1.0).
    pub fn state_of_charge(&self) -> f64 {
        (self.energy_remaining_joules / MAR312_TOTAL_ENERGY_JOULES).clamp(0.0, 1.0)
    }

    /// Net electric energy remaining in the active wafer die (J).
    pub fn wafer_energy_net_j(&self) -> f64 {
        self.active_wafer_soc * MAR312C_SEED_NET_EJ * 1e18
    }

    /// Gross annihilation energy remaining in the active wafer die (J).
    pub fn wafer_energy_gross_j(&self) -> f64 {
        self.active_wafer_soc * MAR312C_SEED_GROSS_EJ * 1e18
    }

    /// Discharge a burst pulse of `gross_power_watts` for
    /// `duration_seconds`; returns the net electrical power delivered after
    /// the 5-stage DEC cascade.
    pub fn discharge_pulse(
        &mut self,
        duration_seconds: f64,
        gross_power_watts: f64,
    ) -> Result<f64, &'static str> {
        if self.is_crowbar_tripped {
            return Err("Discharge blocked: PCSS crowbar tripped.");
        }
        if self.phase != DispatchPhase::FoldingBurst
            && self.phase != DispatchPhase::TriggerArmed
            && self.phase != DispatchPhase::CosetAntibaryonFold
        {
            return Err("Discharge blocked: dispatch phase is not burst-armed.");
        }
        let gross_energy = gross_power_watts * duration_seconds;
        if gross_energy > self.energy_remaining_joules {
            return Err(
                "Energy deficit: Requested pulse exceeds remaining state-of-charge."
            );
        }
        self.energy_remaining_joules -= gross_energy;

        if self.nacelle_mode == ReactorNacelleMode::CompactedWaferMar312cIbr {
            let wafer_gross_j = MAR312C_SEED_GROSS_EJ * 1e18;
            self.active_wafer_soc -= gross_energy / wafer_gross_j;
            // Execute automated hot-swap whenever active wafer SoC drops <= 5% (151.655 PJ net remaining)
            if self.active_wafer_soc <= AUTO_HOT_SWAP_SOC_THRESHOLD {
                if let Some(ref mut mag) = self.dark_magazine {
                    if mag.micro_refills_remaining > 0 {
                        mag.micro_refills_remaining -= 1;
                        mag.total_cores_stored = (mag.micro_refills_remaining as f64) / 10.0;
                        mag.total_reloads += 1;
                        mag.last_reload_latency_s = HOT_SWAP_MICRO_BATCH_LATENCY_S;
                        mag.current_wafer_soc = 1.0;
                    }
                }
                if self.magazine_remaining_ceu >= 0.1 {
                    self.magazine_remaining_ceu -= 0.1;
                }
                self.active_wafer_soc = 1.0;
                self.total_hot_swaps += 1;
                self.last_swap_latency_s = HOT_SWAP_MICRO_BATCH_LATENCY_S;
            }
        }
        Ok(gross_power_watts * MAR312_DEC_EFFICIENCY)
    }

    /// Trip the PCSS optical crowbar: burst output clamps identically to
    /// 0 W and the FSM latches into `EmergencyQuench`.
    pub fn trip_crowbar(&mut self) {
        self.is_crowbar_tripped = true;
        self.is_armed = false;
        self.phase = DispatchPhase::EmergencyQuench;
    }

    /// Net electrical output available this tick: 0 W when quenched or in
    /// any non-burst phase.
    pub fn net_output_w(&self) -> f64 {
        if self.is_crowbar_tripped || self.phase != DispatchPhase::FoldingBurst {
            0.0
        } else {
            self.mode.net_power_w()
        }
    }
}

impl Default for IsomerBatteryState {
    fn default() -> Self {
        Self::new()
    }
}

/// Borrmann cavity net graser gain bookkeeping:
/// g_net = G_isomer * epsilon_B * f_M (dimensionless).
pub fn borrmann_net_gain() -> f64 {
    ISOMER_GATEWAY_GAIN * BORRMANN_EPSILON * MOSSBAUER_FRACTION
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compacted_nacelle::*;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn inventory_and_density() {
        assert!((MAR312_CORE_MASS_KG - 376.99).abs() < 1e-4);
        assert!((MAR312_SPECIFIC_ENERGY_J_KG - 8.987551787368176e16).abs() < 1e10);
        let inv = MAR312_CORE_MASS_KG * MAR312_SPECIFIC_ENERGY_J_KG;
        assert!((inv - MAR312_TOTAL_ENERGY_JOULES).abs() < 1e13);
        assert!((MAR312_DEC_EFFICIENCY - 0.89519).abs() < 1e-9);
        assert!((MAR312_NET_ELECTRICAL_ENERGY_JOULES - 30.331002e18).abs() < 1e14);
        assert!((ISOMER_GATEWAY_GAIN - 61.15).abs() < 1e-4);
        assert!(BORRMANN_EPSILON >= 0.985 && MOSSBAUER_FRACTION >= 0.74);
        assert!(CROWBAR_QUENCH_TIME_SECONDS <= 2.18e-9);
        assert!((SENSIBLE_HEAT_FRACTION - 7.88246e-5).abs() < 1e-8);
    }

    #[test]
    fn canonical_modes_power() {
        assert!((CRUISE_NET_POWER_W - 895.19e12).abs() < 1e8);
        assert!((BURST_NET_POWER_W - 6.805e15).abs() < 1e12);
        assert!((IMPULSE_NET_POWER_W - 895.19e15).abs() < 1e11);
    }

    #[test]
    fn crowbar_lockout_outputs_zero() {
        let mut bat = IsomerBatteryState::new();
        bat.phase = DispatchPhase::FoldingBurst;
        bat.trip_crowbar();
        assert!(bat.discharge_pulse(1e-3, BURST_GROSS_POWER_W).is_err());
        assert_eq!(bat.net_output_w(), 0.0);
        assert_eq!(bat.phase, DispatchPhase::EmergencyQuench);
    }

    #[test]
    fn soc_deficit_rejected() {
        let mut bat = IsomerBatteryState::new();
        bat.phase = DispatchPhase::FoldingBurst;
        assert!(bat
            .discharge_pulse(1.0e12, BURST_GROSS_POWER_W)
            .is_err());
    }

    #[test]
    fn test_compacted_wafer_auto_reload() {
        let mut bat = IsomerBatteryState::new(); // CompactedWaferMar312cIbr, 38 CEU
        assert_eq!(bat.nacelle_mode, ReactorNacelleMode::CompactedWaferMar312cIbr);
        assert_eq!(bat.active_wafer_soc, 1.0);
        assert_eq!(bat.magazine_remaining_ceu, 38.0);
        assert_eq!(bat.total_hot_swaps, 0);

        bat.phase = DispatchPhase::FoldingBurst;
        // Discharge 96% of the 3.38822 EJ wafer: gross_energy = 0.96 * 3.38822e18 J = 3.2526912e18 J
        // In Exawatt Impulse (1.000 EW = 1e18 W), duration = 3.2526912 seconds
        let _gross_pulse = 3.2526912e18;
        let p_net = bat.discharge_pulse(3.2526912, 1.0e18).expect("pulse must succeed");
        assert!((p_net - 895.19e15).abs() < 1e11);

        // Active wafer dropped <= 5%, triggering 50.80 us hot-swap reload from magazine
        assert_eq!(bat.total_hot_swaps, 1);
        assert_eq!(bat.last_swap_latency_s, 50.80e-6);
        assert_eq!(bat.active_wafer_soc, 1.0);
        assert!((bat.magazine_remaining_ceu - 37.9).abs() < 1e-4);
        assert_eq!(bat.dark_magazine.unwrap().micro_refills_remaining, 379);
    }

    #[test]
    fn test_ceu_identity_1_to_10() {
        let base_core_gross = MAR312_TOTAL_ENERGY_JOULES;
        let wafer_gross = MAR312C_SEED_GROSS_EJ * 1e18;
        assert!((base_core_gross - 10.0 * wafer_gross).abs() < 1e14);
        assert!((MAR312_CORE_MASS_KG - 10.0 * MAR312C_SEED_WAFER_MASS_KG).abs() < 1e-4);
        assert!((MAR312_NET_ELECTRICAL_ENERGY_JOULES - 10.0 * (MAR312C_SEED_NET_EJ * 1e18)).abs() < 1e14);
    }
}
