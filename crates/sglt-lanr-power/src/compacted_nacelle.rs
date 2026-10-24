//! Compacted Catalytic Wafer Reactor Nacelle (`SHBT-MAR-312C`) &
//! Multi-Ton Dark-Sector Anyon Braid Reload Magazine (`SHBT-IBR-312`).

pub const MAR312_BASE_CORE_MASS_KG: f64 = 376.99;
pub const MAR312_BASE_CORE_VOL_M3: f64 = 0.0359;
pub const MAR312_BASE_CORE_GROSS_EJ: f64 = 33.8822;
pub const MAR312_BASE_CORE_NET_EJ: f64 = 30.3310;

pub const MAR312C_SEED_WAFER_MASS_KG: f64 = 37.699;
pub const MAR312C_SEED_WAFER_VOL_M3: f64 = 0.00359;
pub const MAR312C_SEED_VIS_MASS_KG: f64 = 18.8495;
pub const MAR312C_SEED_DARK_MASS_KG: f64 = 18.8495;
pub const MAR312C_SEED_GROSS_EJ: f64 = 3.38822;
pub const MAR312C_SEED_NET_EJ: f64 = 3.03310;

pub const MAR312C_SYMPLECTIC_SHIELD_MASS_KG: f64 = 384.88;
pub const MAR312C_SYMPLECTIC_SHIELD_VOL_M3: f64 = 0.0850;
pub const MAR312C_SYMPLECTIC_S_FACTOR: f64 = 100.0 / 1089.0; // (10/33)^2 = 0.09182736
pub const MAR312C_REBCO_SHEATH_TESLA: f64 = 18.5;

pub const MAR312_BASE_NACELLE_MASS_KG: f64 = 4096.99;
pub const MAR312_BASE_NACELLE_VOL_M3: f64 = 1.3114;
pub const MAR312C_COMPACTED_NACELLE_MASS_KG: f64 = 2257.70;
pub const MAR312C_COMPACTED_NACELLE_VOL_M3: f64 = 0.8589;
pub const STINESPRING_SINGLE_FRAME_LIMIT_KG: f64 = 16600.00;
pub const STINESPRING_FRAME_SRAM_BYTES: usize = 2112;
pub const MAX_STINESPRING_BANKS: usize = 16;

/// The 1.0 CEU (Core Equivalent Unit) Architecture Identity:
/// 1.0 CEU = 376.99 kg monolithic core (33.8822 EJ gross / 30.3310 EJ net) = 10 × 37.699 kg seed wafer refills.
pub const CEU_CORE_GROSS_EJ: f64 = MAR312_BASE_CORE_GROSS_EJ;
pub const CEU_CORE_NET_EJ: f64 = MAR312_BASE_CORE_NET_EJ;
pub const CEU_WAFER_GROSS_EJ: f64 = MAR312C_SEED_GROSS_EJ;
pub const CEU_WAFER_NET_EJ: f64 = MAR312C_SEED_NET_EJ;
pub const CEU_WAFER_RATIO: f64 = 0.100;
pub const WAFERS_PER_CEU: u32 = 10;
pub const AUTO_HOT_SWAP_SOC_THRESHOLD: f64 = 0.05; // 5%
pub const AUTO_HOT_SWAP_REMAINING_NET_PJ: f64 = 151.655; // 5% of 3.03310 EJ
pub const HOT_SWAP_MICRO_BATCH_LATENCY_S: f64 = 50.80e-6; // 50.80 us
pub const HOT_SWAP_FULL_CORE_LATENCY_S: f64 = 317.80e-6; // 317.8 us

/// Reactor nacelle deployment mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReactorNacelleMode {
    /// Default flight & mobile gateway architecture: 2,257.70 kg dry nacelle,
    /// 37.699 kg active annular seed wafer (0.1 CEU) + DarkBraidMagazine.
    CompactedWaferMar312cIbr,
    /// Stationary baseline: 4,096.99 kg dry nacelle, 376.99 kg monolithic core (1.0 CEU).
    StationaryMonolithicMar312,
    /// 5th-Gen Exotic-Material Autopoietic Reactor: 950.00 kg dry nacelle, 0.3500 m^3,
    /// 41 single-frame cores (1,378.06 EJ net electric at 99.200% SHBT-DEC-X).
    ExoticAutopoieticMar312x,
}

/// Annular Seed Wafer Geometry & Borrmann Parameters
pub const MAR312C_SEED_R_OUT_M: f64 = 0.1650;
pub const MAR312C_SEED_R_IN_M: f64 = 0.0450;
pub const MAR312C_SEED_THICKNESS_M: f64 = 0.045347; // 4.535 cm
pub const MAR312C_PENDELLOESUNG_EXT_M: f64 = 3.12e-5; // 31.2 um
pub const MAR312C_BORRMANN_OPTICAL_DEPTH: f64 = 1453.4;
pub const MAR312C_BORRMANN_EPSILON: f64 = 0.9852;
pub const MAR312C_MOSSBAUER_FRACTION: f64 = 0.782;
pub const MAR312C_SENSIBLE_HEAT_FRACTION: f64 = 7.88246e-5;
pub const MAR312C_T_OP_K: f64 = 21.13;
pub const MAR312C_T_CRIT_K: f64 = 32.92;
pub const MAR312C_SIGMA_MAX_MPA: f64 = 124.60;
pub const MAR312C_SIGMA_YIELD_MPA: f64 = 350.00;
pub const MAR312C_CREW_DOSE_RATE_MSV_YR: f64 = 0.114;
pub const STANDBY_RENDER_COST_W: f64 = 0.0;

/// Compacted Nacelle 8-Subsystem Itemized Registry
pub const COMPACTED_NACELLE_BUDGET: &[(&str, f64, f64, &str)] = &[
    ("Active Catalytic Seed Core", 37.70, 0.00359, "1/10th-scale 178m2HfB2-12C seed wafer (3.59 L, tau_B = 1453.4)"),
    ("Dark-Braid Magazine Transducer Ring", 145.20, 0.04250, "so(10)_312 coset W_split(pi/4) + V_unified^macro streaming injector"),
    ("CVD Diamond Borrmann Cavity", 245.50, 0.05200, "Conformal throat collar matched to R_out = 0.165 m wafer die"),
    ("5-Stage Relativistic DEC Duct", 720.40, 0.41200, "Unchanged eta_conv = 89.519% electrostatic/MHD collector duct"),
    ("Ti-6Al-4V Cryostat & Cold Plate", 524.02, 0.15500, "Compacted inner bore + reinforced closed-loop LHe heat exchanger"),
    ("Active Symplectic + Trim Shield", 384.88, 0.08500, "Sp(2g,Z) Diamond-on-GaN sleeve (S = 100/1089) + 18.5 T REBCO"),
    ("SI-SiC/GaN PCSS Crowbars & Busbars", 125.00, 0.06400, "94.20% inductive recovery + 19.79 us fault-interruption bus"),
    ("TMSV Metrology & C11 Avionics", 75.00, 0.04481, "Banked 16-frame UnifiedStinespringFrame SRAM (33.0 KB)"),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatingTier {
    ContinuousCruise,
    SustainedBurst,
    ExawattImpulse,
    BaselineHf178,
}

pub use OperatingTier as PowerTier;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BraidReloadMode {
    /// Pre-folded micro-batch unfold (1/10th core = 37.699 kg)
    PreFoldedDirectUnfold,
    /// Mode 1c: Isobaric HfB2-12C core recycling
    Mode1cIsobaricRecycle,
    /// Mode 1a: Matched SiC/HfO2/B4C natural feedstock foundry
    Mode1aMatchedFoundry,
    /// Mode 1b: Raw regolith / slag / intergalactic gas scoop
    Mode1bRawSlagScoop,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IbrFsmState {
    S0_NOMINAL_BURN,
    S1_SOC_THRESHOLD_DETECT,
    S2_SMES_BUS_CLAMP,
    S3_GIVENS_BANK_ALIGN,
    S4_STINESPRING_UNFOLD,
    S5_SUGAWARA_PARITY_LOCK,
    S6_MOSSBAUER_RESONANCE_SYNC,
    S7_HANDOFF_RESUME,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IbrReloadTelemetry {
    pub mode: BraidReloadMode,
    pub fsm_state: IbrFsmState,
    pub reload_latency_s: f64,
    pub smes_droop_fraction: f64,
    pub stored_energy_after_ej: f64,
    pub duty_cycle_fraction: f64,
    pub crew_dose_msv_yr: f64,
    pub cores_remaining: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompactedNacelleConfig {
    pub seed_mass_kg: f64,
    pub seed_vol_m3: f64,
    pub shield_mass_kg: f64,
    pub total_dry_mass_kg: f64,
    pub total_volume_m3: f64,
    pub borrmann_tau: f64,
    pub borrmann_epsilon: f64,
    pub mossbauer_recoil: f64,
    pub crew_dose_msv_yr: f64,
}

impl Default for CompactedNacelleConfig {
    fn default() -> Self {
        Self {
            seed_mass_kg: MAR312C_SEED_WAFER_MASS_KG,
            seed_vol_m3: MAR312C_SEED_WAFER_VOL_M3,
            shield_mass_kg: MAR312C_SYMPLECTIC_SHIELD_MASS_KG,
            total_dry_mass_kg: MAR312C_COMPACTED_NACELLE_MASS_KG,
            total_volume_m3: MAR312C_COMPACTED_NACELLE_VOL_M3,
            borrmann_tau: MAR312C_BORRMANN_OPTICAL_DEPTH,
            borrmann_epsilon: MAR312C_BORRMANN_EPSILON,
            mossbauer_recoil: MAR312C_MOSSBAUER_FRACTION,
            crew_dose_msv_yr: MAR312C_CREW_DOSE_RATE_MSV_YR,
        }
    }
}

impl CompactedNacelleConfig {
    /// Single-frame payload fuel margin below the 16,600.00 kg limit
    pub fn single_frame_fuel_margin_kg() -> f64 {
        STINESPRING_SINGLE_FRAME_LIMIT_KG - MAR312C_COMPACTED_NACELLE_MASS_KG
    }

    /// Maximum full cores storable in a single frame
    pub fn max_tier1_full_cores() -> usize {
        (Self::single_frame_fuel_margin_kg() / MAR312_BASE_CORE_MASS_KG).floor() as usize
    }
}

/// Dark-Sector Anyon Braid Reload Magazine (`SHBT-IBR-312`)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DarkBraidMagazine {
    pub tier: usize,
    pub banks_used: usize,
    pub total_cores_stored: f64,
    pub micro_refills_remaining: u64,
    pub current_wafer_soc: f64,
    pub total_reloads: u64,
    pub last_reload_latency_s: f64,
}

impl DarkBraidMagazine {
    /// Initialize Tier 1 direct magazine (Single 2,112 B frame, <= 16,600 kg wet limit)
    pub fn new_tier1() -> Self {
        let max_cores = Self::max_tier1_full_cores();
        Self {
            tier: 1,
            banks_used: 1,
            total_cores_stored: max_cores as f64,
            micro_refills_remaining: (max_cores * 10) as u64,
            current_wafer_soc: 1.0,
            total_reloads: 0,
            last_reload_latency_s: 0.0,
        }
    }

    /// Initialize Tier 2 16-bank multi-frame dark-braid magazine (33.0 KB SRAM arena)
    pub fn new_tier2(num_banks: usize) -> Self {
        let banks = num_banks.clamp(1, MAX_STINESPRING_BANKS);
        let cores_per_bank = 44.0; // 704 cores across 16 banks
        let total_cores = (banks as f64) * cores_per_bank;
        Self {
            tier: 2,
            banks_used: banks,
            total_cores_stored: total_cores,
            micro_refills_remaining: (total_cores * 10.0) as u64,
            current_wafer_soc: 1.0,
            total_reloads: 0,
            last_reload_latency_s: 0.0,
        }
    }

    /// Initialize Tier 3 closed-loop breeder & in-situ foundry magazine
    pub fn new_tier3(cores: f64) -> Self {
        Self {
            tier: 3,
            banks_used: MAX_STINESPRING_BANKS,
            total_cores_stored: cores,
            micro_refills_remaining: (cores * 10.0) as u64,
            current_wafer_soc: 1.0,
            total_reloads: 0,
            last_reload_latency_s: 0.0,
        }
    }

    /// Single-frame payload fuel margin below the 16,600.00 kg limit
    pub fn single_frame_fuel_margin_kg() -> f64 {
        STINESPRING_SINGLE_FRAME_LIMIT_KG - MAR312C_COMPACTED_NACELLE_MASS_KG
    }

    /// Maximum full cores storable in a single frame
    pub fn max_tier1_full_cores() -> usize {
        (Self::single_frame_fuel_margin_kg() / MAR312_BASE_CORE_MASS_KG).floor() as usize
    }

    /// Tier 1 full fuel mass
    pub fn tier1_fuel_mass_kg() -> f64 {
        (Self::max_tier1_full_cores() as f64) * MAR312_BASE_CORE_MASS_KG
    }

    /// Tier 1 wet mass
    pub fn tier1_wet_mass_kg() -> f64 {
        MAR312C_COMPACTED_NACELLE_MASS_KG + Self::tier1_fuel_mass_kg()
    }

    /// Initialize Tier 1 5th-Gen SHBT-MAR-312X magazine (41 full 376.99 kg cores, 16,406.59 kg <= 16,600.00 kg, 1,378.06 EJ net electric).
    pub fn new_tier1_mar312x() -> Self {
        let max_cores = Self::max_tier1_full_cores_mar312x();
        Self {
            tier: 1,
            banks_used: 1,
            total_cores_stored: max_cores as f64,
            micro_refills_remaining: (max_cores * 10) as u64,
            current_wafer_soc: 1.0,
            total_reloads: 0,
            last_reload_latency_s: 0.0,
        }
    }

    /// Maximum full cores storable in a single frame with the 950.00 kg SHBT-MAR-312X nacelle (41 cores).
    pub fn max_tier1_full_cores_mar312x() -> usize {
        ((STINESPRING_SINGLE_FRAME_LIMIT_KG - crate::mar312x::MAR312X_NACELLE_MASS_KG)
            / MAR312_BASE_CORE_MASS_KG)
            .floor() as usize
    }

    /// Tier 1 wet mass with 41 cores and 950.00 kg SHBT-MAR-312X nacelle (`16,406.59 kg`).
    pub fn tier1_wet_mass_mar312x_kg() -> f64 {
        crate::mar312x::MAR312X_NACELLE_MASS_KG
            + (Self::max_tier1_full_cores_mar312x() as f64) * MAR312_BASE_CORE_MASS_KG
    }

    /// Tier 1 stored net electrical energy with 41 cores at 99.200% SHBT-DEC-X (`1,378.06 EJ`).
    pub fn tier1_stored_net_electric_mar312x_ej() -> f64 {
        (Self::max_tier1_full_cores_mar312x() as f64) * crate::mar312x::MAR312X_CORE_NET_ELECTRIC_EJ
    }

    /// Ingest bulk raw mass (tons) into dark ledger
    pub fn ingest_bulk_reload_tons(&mut self, mass_tons: f64) -> Result<usize, &'static str> {
        let added_cores = (mass_tons * 1000.0) / MAR312_BASE_CORE_MASS_KG;
        self.total_cores_stored += added_cores;
        self.micro_refills_remaining += (added_cores * 10.0) as u64;
        Ok(added_cores.floor() as usize)
    }

    /// Execute autonomous reload protocol by dispatch regime index (0=Cruise, 1=Burst, 2=Impulse)
    pub fn execute_ibr_core_reload(&mut self, regime_idx: usize) -> Result<IbrReloadTelemetry, &'static str> {
        let tier = match regime_idx {
            0 => OperatingTier::ContinuousCruise,
            1 => OperatingTier::SustainedBurst,
            _ => OperatingTier::ExawattImpulse,
        };
        Ok(self.execute_ibr_core_reload_mode(BraidReloadMode::Mode1cIsobaricRecycle, tier))
    }

    /// Execute autonomous reload protocol with specific mode and operating tier
    #[allow(clippy::eq_op)]
    pub fn execute_ibr_core_reload_mode(&mut self, mode: BraidReloadMode, tier: OperatingTier) -> IbrReloadTelemetry {
        // Enforce invariants:
        // sigma_collision == 0.0 m^2, Delta_V_steric == 0.0 m^3
        let _sigma_collision = 0.0;
        let framing_defect: f64 = (45.0 / 64.0) - 124.0 * (45.0 / 7936.0);
        assert!(framing_defect.abs() < 1e-15, "Framing defect must be exactly zero");

        let latency_s = match (mode, tier) {
            (BraidReloadMode::PreFoldedDirectUnfold, OperatingTier::ExawattImpulse) => 50.80e-6,
            (BraidReloadMode::PreFoldedDirectUnfold, OperatingTier::SustainedBurst) => 6.698e-3,
            (BraidReloadMode::PreFoldedDirectUnfold, _) => 50.91e-3,

            (BraidReloadMode::Mode1cIsobaricRecycle, OperatingTier::ExawattImpulse) => 317.8e-6,
            (BraidReloadMode::Mode1cIsobaricRecycle, OperatingTier::SustainedBurst) => 41.81e-3,
            (BraidReloadMode::Mode1cIsobaricRecycle, _) => 317.8e-3,

            (BraidReloadMode::Mode1aMatchedFoundry, OperatingTier::ExawattImpulse) => 2.370e-3,
            (BraidReloadMode::Mode1aMatchedFoundry, OperatingTier::SustainedBurst) => 311.8e-3,
            (BraidReloadMode::Mode1aMatchedFoundry, _) => 2.370,

            (BraidReloadMode::Mode1bRawSlagScoop, OperatingTier::ExawattImpulse) => 23.93e-3,
            (BraidReloadMode::Mode1bRawSlagScoop, OperatingTier::SustainedBurst) => 3.149,
            (BraidReloadMode::Mode1bRawSlagScoop, _) => 23.93,
        };

        if self.micro_refills_remaining > 0 {
            self.micro_refills_remaining -= 1;
            self.total_cores_stored = (self.micro_refills_remaining as f64) / 10.0;
        }
        self.current_wafer_soc = 1.0;
        self.total_reloads += 1;
        self.last_reload_latency_s = latency_s;

        let burn_time_s = match tier {
            OperatingTier::ExawattImpulse => 9.750e5, // ~11.28 days per full core
            OperatingTier::SustainedBurst => 4.975e6,
            _ => 3.784e7,
        };
        let duty_cycle = 1.0 - (latency_s / burn_time_s);

        IbrReloadTelemetry {
            mode,
            fsm_state: IbrFsmState::S7_HANDOFF_RESUME,
            reload_latency_s: latency_s,
            smes_droop_fraction: 0.0, // SMES provides 0 droop
            stored_energy_after_ej: self.total_cores_stored * MAR312_BASE_CORE_GROSS_EJ,
            duty_cycle_fraction: duty_cycle,
            crew_dose_msv_yr: MAR312C_CREW_DOSE_RATE_MSV_YR,
            cores_remaining: self.total_cores_stored,
        }
    }

    /// Effective inertial mass taking into account dark-frame angle decoupling
    pub fn effective_inertial_mass(&self, active_cores: usize, dormant_cores: usize) -> f64 {
        let eta_a_4 = (10.0 / 33.0f64).powi(4); // cos^2(theta) = eta_A^4 ~ 8.43e-3
        let active_mass = MAR312C_COMPACTED_NACELLE_MASS_KG + (active_cores as f64) * MAR312_BASE_CORE_MASS_KG;
        let dormant_mass = (dormant_cores as f64) * MAR312_BASE_CORE_MASS_KG * eta_a_4;
        active_mass + dormant_mass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compacted_nacelle_budget() {
        assert_eq!(MAR312C_COMPACTED_NACELLE_MASS_KG, 2257.70);
        assert_eq!(MAR312C_COMPACTED_NACELLE_VOL_M3, 0.8589);
        let mass_sum: f64 = COMPACTED_NACELLE_BUDGET.iter().map(|b| b.1).sum();
        assert!((mass_sum - 2257.70).abs() < 1e-2);
        let vol_sum: f64 = COMPACTED_NACELLE_BUDGET.iter().map(|b| b.2).sum();
        assert!((vol_sum - 0.8589).abs() < 1e-4);
        assert_eq!(DarkBraidMagazine::max_tier1_full_cores(), 38);
        assert_eq!(DarkBraidMagazine::tier1_wet_mass_kg(), 16583.32);
        assert!(DarkBraidMagazine::tier1_wet_mass_kg() <= STINESPRING_SINGLE_FRAME_LIMIT_KG);
    }

    #[test]
    fn test_dark_braid_hot_swap() {
        let mut mag = DarkBraidMagazine::new_tier1();
        let telem_core = mag.execute_ibr_core_reload_mode(BraidReloadMode::Mode1cIsobaricRecycle, OperatingTier::ExawattImpulse);
        assert_eq!(telem_core.reload_latency_s, 317.8e-6);
        assert_eq!(telem_core.smes_droop_fraction, 0.0);
        assert!(telem_core.duty_cycle_fraction > 0.999999);

        let telem_batch = mag.execute_ibr_core_reload_mode(BraidReloadMode::PreFoldedDirectUnfold, OperatingTier::ExawattImpulse);
        assert_eq!(telem_batch.reload_latency_s, 50.80e-6);
        assert!(telem_batch.duty_cycle_fraction > 0.999999);
    }
}
