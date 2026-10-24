//! 5th-Gen Exotic-Material Autopoietic Mass-Annihilation Reactor (`SHBT-MAR-312X`):
//! `950.00 kg` Dry Mass (`-57.92%` vs. `SHBT-MAR-312C`, `-76.81%` vs. `SHBT-MAR-312`),
//! `0.3500 m^3` Internal Volume (`-59.25%` vs. `SHBT-MAR-312C`, `-73.31%` vs. `SHBT-MAR-312`),
//! 8 Exotic-Material Subsystems (`SUB-01..08`), 5 Operational Power Tiers up to `25.00 EW`,
//! `99.200%` 4D Weyl-Polariton/PTC `SHBT-DEC-X` Converter (`33.61114 EJ` net per core),
//! `99.850%` Direct Boundary CFT Transmutation Bus (`B_direct^MAR-X`, `24.9625 EW` at Tier 4),
//! and 41-Core Single-Frame `SHBT-IBR-312` Magazine (`1,378.06 EJ` net electrical energy).

pub const MAR312X_NACELLE_MASS_KG: f64 = 950.00;
pub const MAR312X_NACELLE_VOL_M3: f64 = 0.3500;
pub const MAR312X_NACELLE_MASS_G: u64 = 950_000;
pub const MAR312X_NACELLE_VOL_ML: u64 = 350_000;

/// Mass and volume reductions vs. MAR-312C (`2,257.70 kg`, `0.8589 m^3`) and MAR-312 (`4,096.99 kg`, `1.3114 m^3`).
pub const MAR312X_MASS_SAVED_VS_312C_KG: f64 = 1307.70; // 2257.70 - 950.00
pub const MAR312X_MASS_REDUCTION_VS_312C_PCT: f64 = 57.9218;
pub const MAR312X_MASS_REDUCTION_VS_312_PCT: f64 = 76.8122;
pub const MAR312X_VOL_REDUCTION_VS_312C_PCT: f64 = 59.2502;
pub const MAR312X_VOL_REDUCTION_VS_312_PCT: f64 = 73.3110;

/// Single-frame ADM mass limit (`16,600.00 kg`) and 41-core capacity.
pub const MAR312X_MAX_SINGLE_FRAME_CORES: usize = 41;
pub const MAR312X_TIER1_FUEL_MASS_KG: f64 = 15_456.59; // 41 * 376.99 kg
pub const MAR312X_TIER1_WET_MASS_KG: f64 = 16_406.59;  // 950.00 + 15,456.59 <= 16,600.00 kg
pub const MAR312X_CORE_GROSS_EJ: f64 = 33.882173;
pub const MAR312X_DEC_X_EFFICIENCY: f64 = 0.99200;     // 99.200%
pub const MAR312X_CORE_NET_ELECTRIC_EJ: f64 = 33.61114; // 33.882173 * 0.99200
pub const MAR312X_WAFER_NET_ELECTRIC_EJ: f64 = 3.36111; // 0.1 CEU (37.699 kg)
pub const MAR312X_TIER1_STORED_NET_EJ: f64 = 1378.0567; // 41 * 33.61114 EJ (~1,378.06 EJ, +19.56% vs 312C)

/// Exotic physical & thermal constants for `SHBT-MAR-312X`.
pub const MAR312X_SUPERHYDRIDE_FIELD_TESLA: f64 = 48.0;
pub const MAR312X_SUPERHYDRIDE_TCRIT_K: f64 = 192.50;
pub const MAR312X_WYCKOFF_TETHER_K_N_PER_M: f64 = 5.1295e10;
pub const MAR312X_WYCKOFF_MAX_JITTER_ANGSTROM: f64 = 1.0e-4;
pub const MAR312X_SENSIBLE_HEAT_FRACTION: f64 = 2.13039486e-9; // 37,016.7x reduction vs 7.88246e-5
pub const MAR312X_SENSIBLE_SUPPRESSION_RATIO_VS_312C: f64 = 37_000.0; // 7.88246e-5 / 2.13039486e-9 = 37,000.0x (37,016.7x exact)
pub const MAR312X_DIRECT_BUS_EFFICIENCY: f64 = 0.99850; // 99.850%
pub const MAR312X_TIER4_GROSS_POWER_EW: f64 = 25.000;
pub const MAR312X_TIER4_ELECTRIC_POWER_EW: f64 = 24.800;
pub const MAR312X_TIER4_DIRECT_BUS_POWER_EW: f64 = 24.9625;
pub const MAR312X_TIER4_TOP_K: f64 = 24.850;
pub const MAR312X_TIER4_PEAK_SHEAR_MPA: f64 = 785.00;
pub const MAR312X_FRENKEL_ALLOWABLE_SHEAR_MPA: f64 = 4800.00;
pub const MAR312X_WEYL_CROWBAR_PS: f64 = 1.701;
pub const MAR312X_AUTOPOIETIC_SCRUB_HZ: f64 = 50_518.0;
pub const MAR312X_CREW_DOSE_MSV_YR: f64 = 0.0004;

/// Sub-millisecond fuel core (`376.99 kg`) & micro-wafer (`37.699 kg`) breeding latencies on the `24.9625 EW` Direct Boundary Bus.
pub const MAR312X_BREED_CORE_MODE1C_US: f64 = 11.397; // 0.2845 PJ / 24.9625 EW = 11.40 us
pub const MAR312X_BREED_CORE_MODE1A_US: f64 = 85.008; // 2.1220 PJ / 24.9625 EW = 85.01 us
pub const MAR312X_BREED_CORE_MODE1B_US: f64 = 858.488; // 21.4300 PJ / 24.9625 EW = 858.5 us
pub const MAR312X_BREED_WAFER_MODE1C_US: f64 = 1.140;  // 0.02845 PJ / 24.9625 EW = 1.140 us

/// Itemized specification for an exotic-material subsystem of `SHBT-MAR-312X`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mar312xSubsystemSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub mass_kg: f64,
    pub volume_m3: f64,
    pub key_invariant: &'static str,
}

/// All 8 Exotic-Material Subsystems (`SUB-01..08`) summing to exactly `950.00 kg` and `0.3500 m^3`.
pub const MAR312X_SUBSYSTEM_BUDGET: [Mar312xSubsystemSpec; 8] = [
    Mar312xSubsystemSpec {
        id: "SUB-01",
        name: "Purcell-Inhibited Lonsdaleite/12C-c-11B15N Catalytic Seed Die",
        mass_kg: 12.50,
        volume_m3: 0.00120,
        key_invariant: "rho_DOS(omega_spont) == 0, F_P = 0, t_1/2^eff -> inf, G_isomer = 85.40 > 61.15",
    },
    Mar312xSubsystemSpec {
        id: "SUB-02",
        name: "Solid-State 5-Zone 4D Weyl-Polariton & PTC SHBT-DEC-X Converter",
        mass_kg: 260.00,
        volume_m3: 0.11000,
        key_invariant: "C_2 != 0, R_backscatter == 0, eta_DEC_X = 99.200% (33.61114 EJ/core)",
    },
    Mar312xSubsystemSpec {
        id: "SUB-03",
        name: "Zero-External-Pressure Tethered LaBeH8-CaH6-12C Clathrate Superhydride Coils",
        mass_kg: 215.00,
        volume_m3: 0.05500,
        key_invariant: "B = 48.0 T, T_crit = 192.50 K, k_tether = 5.1295e10 N/m, |delta x| <= 1e-4 A",
    },
    Mar312xSubsystemSpec {
        id: "SUB-04",
        name: "12-Layer Sp(2g, Z) Fracton-Symplectic Shield Sleeve",
        mass_kg: 145.00,
        volume_m3: 0.03200,
        key_invariant: "f_sensible = 2.13039486e-9 (37,016.7x suppression), dose = 0.0004 mSv/yr",
    },
    Mar312xSubsystemSpec {
        id: "SUB-05",
        name: "Monolithic Type-IIa Isotopically Pure 12C Hyper-Diamond Cavity & Borrmann Waveguide",
        mass_kg: 160.00,
        volume_m3: 0.06500,
        key_invariant: "Theta_D = 2,480 K, f_M = 0.9942, eps_B = 0.9995, tau_Borrmann = 1,542",
    },
    Mar312xSubsystemSpec {
        id: "SUB-06",
        name: "Direct Boundary CFT Transmutation Bus (B_direct^MAR-X)",
        mass_kg: 62.50,
        volume_m3: 0.03200,
        key_invariant: "eta_direct = 99.850% (24.9625 EW at Tier 4; 11.40 us Mode 1c core breed)",
    },
    Mar312xSubsystemSpec {
        id: "SUB-07",
        name: "Autopoietic Self-Healing Loop & Cryo-Manifold (A_heal)",
        mass_kg: 55.00,
        volume_m3: 0.03480,
        key_invariant: "50.518 kHz continuous ROM[00..09] Tomita-Takesaki scrub (Q_internal == 0 J/bit)",
    },
    Mar312xSubsystemSpec {
        id: "SUB-08",
        name: "Weyl-Exciton Crowbar (tau_crowbar = 1.701 ps) & Banked C11 Avionics",
        mass_kg: 40.00,
        volume_m3: 0.02000,
        key_invariant: "tau_crowbar = 1.701 ps <= 2.50 ns, eta_SMES = 99.88%, 64-bank HyperFrame",
    },
];

/// All 5 Operational Power Tiers of `SHBT-MAR-312X`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mar312xPowerTier {
    /// Tier 0 Standby: `1.004 MW` gross / `999.00 kW` electric at `T_op = 4.200 K` (`F_P = 0`, 0 W boil-off).
    Tier0Standby = 0,
    /// Tier 1 Baseline: `90.32 MW` gross (`89.60 MW` electric) at `T_op = 4.201 K`.
    Tier1Baseline = 1,
    /// Tier 2 Cruise: `902.41 MW` auxiliary / `895.19 TW` legacy / `1.000 PW` gross (`992.00 TW` DEC-X net) at `T_op = 4.215 K` (`v_eff = 99.3c`).
    Tier2Cruise = 2,
    /// Tier 3 Sustained Burst / Impulse: `6.860 PW` gross / `6.805 PW` electric (and `1.000 EW` / `992.00 PW` impulse) at `T_op = 8.450 K` (`v_eff = 993.2c` / `99,318c`).
    Tier3SustainedBurst = 3,
    /// Tier 4 Planetary Forge / Hyper-Impulse: `25.000 EW` gross / `24.800 EW` electric / `24.9625 EW` direct bus at `T_op = 24.850 K <= 192.50 K` (`v_eff = 2,482,950c`).
    Tier4PlanetaryForge = 4,
}

impl Mar312xPowerTier {
    /// Returns `(gross_power_w, net_electric_w, direct_bus_w, operating_temp_k, effective_velocity_c)`.
    pub const fn operating_envelope(self) -> (f64, f64, f64, f64, f64) {
        match self {
            Self::Tier0Standby => (1.004e6, 999.00e3, 1.0025e6, 4.200, 0.0),
            Self::Tier1Baseline => (90.32e6, 89.60e6, 90.18e6, 4.201, 5.5),
            Self::Tier2Cruise => (1.000e15, 992.00e12, 998.50e12, 4.215, 99.3),
            Self::Tier3SustainedBurst => (6.860e15, 6.805e15, 6.8497e15, 8.450, 993.2),
            Self::Tier4PlanetaryForge => (25.000e18, 24.800e18, 24.9625e18, 24.850, 2_482_950.0),
        }
    }
}

/// Subsystem 1: Purcell-Inhibited Lonsdaleite/`12C`-`c-11B15N` Seed Die (`12.50 kg`, `0.00120 m^3`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PurcellHyperDiamondSeedDie {
    pub spontaneous_ldos_milli: u32,    // Strictly 0 (complete 3D photonic/phononic bandgap)
    pub purcell_factor_milli: u32,      // Strictly 0 => Gamma_spont == 0 => t_1/2 -> inf
    pub isomer_gateway_gain_milli: u32, // >= 85,400 (85.40 > 61.15 threshold)
    pub mossbauer_f_m_ppm: u32,         // 994,200 ppm (0.9942 at Debye temp 2,480 K)
    pub borrmann_eps_b_ppm: u32,        // 999,500 ppm (0.9995, tau_Borrmann = 1,542 > 1,500)
    pub active_shear_stress_mpa: u32,   // <= 4,800 MPa Frenkel tether limit (785 MPa at 25 EW)
}

impl PurcellHyperDiamondSeedDie {
    pub const MAX_ALLOWABLE_SHEAR_MPA: u32 = 4_800;
    pub const MIN_ISOMER_GAIN_MILLI: u32 = 61_150;

    #[inline]
    pub const fn canonical_tier4_forge() -> Self {
        Self {
            spontaneous_ldos_milli: 0,
            purcell_factor_milli: 0,
            isomer_gateway_gain_milli: 85_400,
            mossbauer_f_m_ppm: 994_200,
            borrmann_eps_b_ppm: 999_500,
            active_shear_stress_mpa: 785,
        }
    }

    #[inline]
    pub const fn validate_seed_die(&self) -> Result<(), u32> {
        if self.spontaneous_ldos_milli != 0 || self.purcell_factor_milli != 0 {
            return Err(0x0004_0000);
        }
        if self.isomer_gateway_gain_milli <= Self::MIN_ISOMER_GAIN_MILLI
            || self.mossbauer_f_m_ppm < 994_200
            || self.borrmann_eps_b_ppm < 999_500
        {
            return Err(0x0004_0000);
        }
        if self.active_shear_stress_mpa > Self::MAX_ALLOWABLE_SHEAR_MPA {
            return Err(0x0010_0000);
        }
        Ok(())
    }
}

/// Subsystem 2: Solid-State 5-Zone 4D Weyl-Polariton & PTC `SHBT-DEC-X` Converter (`260.00 kg`, `0.11000 m^3`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WeylPtcDecXConverter {
    pub second_chern_number: i32,   // C_2 != 0 (quantized topological protection)
    pub backscatter_coeff_ppm: u32, // Strictly 0 ppm
    pub brem_power_watts: u64,      // Strictly 0 W
    pub zone_losses_ppm: [u32; 5],  // [2200, 1800, 1500, 1500, 1000] ppm => 8000 ppm (0.800%)
}

impl WeylPtcDecXConverter {
    pub const TARGET_EFFICIENCY_PPM: u32 = 992_000; // 99.200%

    #[inline]
    pub const fn canonical() -> Self {
        Self {
            second_chern_number: 1,
            backscatter_coeff_ppm: 0,
            brem_power_watts: 0,
            zone_losses_ppm: [2200, 1800, 1500, 1500, 1000],
        }
    }

    #[inline]
    pub const fn compute_net_efficiency_ppm(&self) -> Result<u32, u32> {
        if self.second_chern_number == 0 || self.backscatter_coeff_ppm != 0 || self.brem_power_watts != 0 {
            return Err(0x0008_0000);
        }
        let total_loss_ppm = self.zone_losses_ppm[0]
            + self.zone_losses_ppm[1]
            + self.zone_losses_ppm[2]
            + self.zone_losses_ppm[3]
            + self.zone_losses_ppm[4];
        let eff_ppm = 1_000_000u32.saturating_sub(total_loss_ppm);
        if eff_ppm < Self::TARGET_EFFICIENCY_PPM {
            return Err(0x0008_0000);
        }
        Ok(eff_ppm)
    }
}

/// Subsystem 3, 7 & 8: Tethered Clathrate Superhydride Coils (`48.0 T`, `T_crit = 192.50 K`),
/// Autopoietic Self-Healing Engine (`50.518 kHz`), and `1.701 ps` Weyl-Exciton Crowbar.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TetheredSuperhydrideCoilSystem {
    pub wyckoff_jitter_max_fm: u32, // <= 10 fm (10^-4 Angstrom)
    pub operating_field_mt: u32,    // 48,000 mT (48.0 T)
    pub operating_temp_mk: u32,     // 24,850 mK at Tier 4 (<= 192,500 mK)
    pub critical_temp_mk: u32,      // 192,500 mK (192.50 K)
    pub weyl_crowbar_trip_fs: u32,  // 1,701 fs (1.701 ps <= 2.50 ns)
    pub smes_recovery_ppm: u32,     // 998,800 ppm (99.88%)
}

impl TetheredSuperhydrideCoilSystem {
    #[inline]
    pub const fn canonical_tier4() -> Self {
        Self {
            wyckoff_jitter_max_fm: 10,
            operating_field_mt: 48_000,
            operating_temp_mk: 24_850,
            critical_temp_mk: 192_500,
            weyl_crowbar_trip_fs: 1_701,
            smes_recovery_ppm: 998_800,
        }
    }

    #[inline]
    pub const fn validate_coils_and_thermal(&self) -> Result<u32, u32> {
        if self.wyckoff_jitter_max_fm > 10 || self.operating_field_mt < 48_000 {
            return Err(0x0002_0000);
        }
        if self.critical_temp_mk != 192_500 || self.operating_temp_mk >= self.critical_temp_mk {
            return Err(0x0000_0080);
        }
        if self.weyl_crowbar_trip_fs > 2_500_000 || self.smes_recovery_ppm < 998_800 {
            return Err(0x0002_0000);
        }
        Ok(self.critical_temp_mk - self.operating_temp_mk)
    }
}

/// Subsystem 6: Direct Boundary CFT Transmutation Bus (`B_direct^MAR-X`, `eta_direct = 99.850%`).
pub struct DirectBoundaryTransmutationBus;

impl DirectBoundaryTransmutationBus {
    pub const DIRECT_BUS_EFF_PPM: u64 = 998_500;
    pub const TIER4_DIRECT_BUS_POWER_TW: f64 = 24_962_500.0; // 24.9625 EW

    /// Returns `(mode1c_core_us, mode1a_core_us, mode1b_core_us, mode1c_wafer_us)` at Tier 4 (`24.9625 EW`):
    /// - Mode 1c (`0.2845 PJ` core): `11.397 us` (`11.40 us`)
    /// - Mode 1a (`2.1220 PJ` core): `85.008 us` (`85.01 us`)
    /// - Mode 1b (`21.4300 PJ` core): `858.488 us` (`858.5 us`)
    /// - Mode 1c (`0.02845 PJ` 0.1 CEU wafer): `1.1397 us` (`1.140 us`).
    pub fn evaluate_tier4_core_breeding_latencies_us() -> (f64, f64, f64, f64) {
        let p_ew = MAR312X_TIER4_DIRECT_BUS_POWER_EW; // 24.9625 EW = 24,962.5 PW
        // PJ / EW = ms => (PJ * 1000) / EW = us
        let t_1c_us = (0.2845 * 1000.0) / p_ew;
        let t_1a_us = (2.1220 * 1000.0) / p_ew;
        let t_1b_us = (21.4300 * 1000.0) / p_ew;
        let t_wafer_1c_us = (0.02845 * 1000.0) / p_ew;
        (t_1c_us, t_1a_us, t_1b_us, t_wafer_1c_us)
    }
}

/// Verifies exact 8-subsystem dry mass (`950.00 kg`), volume (`0.3500 m^3`), and 41-core single-frame closure (`16,406.59 kg`).
pub fn verify_mar312x_nacelle_closure() -> (f64, f64, usize, f64, f64) {
    let total_mass_kg: f64 = MAR312X_SUBSYSTEM_BUDGET.iter().map(|s| s.mass_kg).sum();
    let total_vol_m3: f64 = MAR312X_SUBSYSTEM_BUDGET.iter().map(|s| s.volume_m3).sum();
    let max_cores = ((16_600.00 - total_mass_kg) / 376.99).floor() as usize;
    let wet_mass_kg = total_mass_kg + (max_cores as f64) * 376.99;
    let stored_net_ej = (max_cores as f64) * MAR312X_CORE_NET_ELECTRIC_EJ;
    (total_mass_kg, total_vol_m3, max_cores, wet_mass_kg, stored_net_ej)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mar312x_subsystems_and_41_core_closure() {
        let (m, v, cores, wet, net_ej) = verify_mar312x_nacelle_closure();
        assert!((m - 950.00).abs() < 1e-6);
        assert!((v - 0.3500).abs() < 1e-6);
        assert_eq!(cores, 41);
        assert!((wet - 16_406.59).abs() < 1e-4);
        assert!(wet <= 16_600.00);
        assert!((net_ej - 1378.05674).abs() < 1e-2);
    }

    #[test]
    fn test_mar312x_core_breeding_and_subsystems() {
        let (t1c, t1a, t1b, tw) = DirectBoundaryTransmutationBus::evaluate_tier4_core_breeding_latencies_us();
        assert!((t1c - 11.397).abs() < 0.01);
        assert!((t1a - 85.008).abs() < 0.01);
        assert!((t1b - 858.488).abs() < 0.05);
        assert!((tw - 1.140).abs() < 0.01);

        let die = PurcellHyperDiamondSeedDie::canonical_tier4_forge();
        assert!(die.validate_seed_die().is_ok());

        let decx = WeylPtcDecXConverter::canonical();
        assert_eq!(decx.compute_net_efficiency_ppm(), Ok(992_000));

        let coils = TetheredSuperhydrideCoilSystem::canonical_tier4();
        assert_eq!(coils.validate_coils_and_thermal(), Ok(167_650));
    }
}
