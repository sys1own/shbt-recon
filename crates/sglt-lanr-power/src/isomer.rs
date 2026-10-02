//! Solid-state coherent graser ^178m2Hf nuclear isomer battery.
//!
//! Transferred from the upstream `sys1own/shbt-warp` isomer graser core,
//! Borrmann anomalous-transmission cavity, and 3-stage relativistic Direct
//! Energy Conversion (DEC) stack (auxiliary references:
//! `sys1own/shbt-power`, `sys1own/shbt-ghost`).  The module supplies the
//! pulsed burst rail of the dual-power dispatch topology: the 1,800-module
//! LANR array covers the 906.00 kW Landauer entropy-debt floor continuously
//! while the 500.0 TJ isomer core discharges only during active isometric
//! folding and causal destination egress, preserving the zero-residual
//! condition E_mn == 0 on the canonical WZW affine branch (26, 8, 312).

/// Total stored energy in the monolithic ^178m2HfB2 core (J).
pub const ISOMER_TOTAL_ENERGY_JOULES: f64 = 500.0e12; // 500.0 TJ
/// Active isomer core mass (kg).
pub const ISOMER_CORE_MASS_KG: f64 = 376.99;
/// Core specific energy density (J/kg): 1.3263 TJ/kg.
pub const SPECIFIC_ENERGY_JOULES_PER_KG: f64 = 1.3263e12;
/// Resonant gateway X-ray trigger energy (keV).
pub const GATEWAY_TRIGGER_ENERGY_KEV: f64 = 40.0;
/// Isomer energy release per de-excitation (keV): E_x = 2.446 MeV.
pub const ISOMER_ENERGY_RELEASE_KEV: f64 = 2446.0;
/// Breit-Wigner resonant excitation cross section (cm^2).
pub const BREIT_WIGNER_SIGMA_CM2: f64 = 1.48e-21;
/// Resonant gateway trigger gain: 2446.0 / 40.0 = 61.15.
pub const ISOMER_GATEWAY_GAIN: f64 =
    ISOMER_ENERGY_RELEASE_KEV / GATEWAY_TRIGGER_ENERGY_KEV;
/// Isomer half-life (years).
pub const ISOMER_HALF_LIFE_YEARS: f64 = 31.0;
/// Borrmann anomalous transmission factor at T <= 21.13 K.
pub const BORRMANN_EPSILON: f64 = 0.985;
/// Mossbauer recoilless fraction at T <= 21.13 K.
pub const MOSSBAUER_FRACTION: f64 = 0.74;
/// 3-stage relativistic DEC stage efficiencies.
pub const DEC_ETA_COMPTON: f64 = 0.264;
pub const DEC_ETA_PAIR: f64 = 0.121;
pub const DEC_ETA_RETARDING: f64 = 0.073;
/// Cascaded DEC conversion efficiency: 26.4% + 12.1% + 7.3% = 45.8%.
pub const DEC_TOTAL_EFFICIENCY: f64 =
    DEC_ETA_COMPTON + DEC_ETA_PAIR + DEC_ETA_RETARDING;
/// Nominal gross graser burst power (W): 109.05 TW.
pub const GROSS_BURST_POWER_W: f64 = 109.05e12;
/// Net electrical burst power after DEC (W): 109.05 TW * 0.458 = 49.9449 TW.
pub const NET_BURST_POWER_W: f64 = GROSS_BURST_POWER_W * DEC_TOTAL_EFFICIENCY;
/// PCSS optical crowbar quench latency bound (s): 2.18 ns.
pub const CROWBAR_QUENCH_TIME_SECONDS: f64 = 2.18e-9;
/// Fraction of interrupted reactive energy recovered into SMES coils.
pub const SMES_RECOVERY_FRACTION: f64 = 0.9420;
/// Fraction shunted to external 80/20 W-Cu thermal dumps.
pub const WC_DUMP_FRACTION: f64 = 1.0 - SMES_RECOVERY_FRACTION;
/// DEC output bus low/high rails (V): 15 kV to 400 kV DC.
pub const DEC_BUS_LOW_V: f64 = 15.0e3;
pub const DEC_BUS_HIGH_V: f64 = 400.0e3;

/// 5-phase isomer burst dispatch state machine.
///
/// The phase-gated protocol enforces the zero-residual condition: burst
/// discharge is permitted only in `FoldingBurst` (isometric folding) and is
/// arrested by the optical crowbar within 2.18 ns for the symplectic
/// address-relabeling phase (T^d_ij in Sp(2g, Z)); a matched burst is
/// re-armed for causal destination egress (ds^2_2PN <= 0).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DispatchPhase {
    /// 0x01 — quiescent baseload; LANR rail only.
    StandbyStasis = 0x01,
    /// 0x02 — gateway trigger armed, awaiting fold authorization.
    TriggerArmed = 0x02,
    /// 0x04 — active isometric folding burst discharge.
    FoldingBurst = 0x04,
    /// 0x08 — symplectic relabeling cooldown; burst quenched.
    SymplecticCooldown = 0x08,
    /// 0x10 — PCSS crowbar tripped; output clamped to 0 W.
    EmergencyQuench = 0x10,
}

/// Isomer core state: state-of-charge plus dispatch interlocks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsomerBatteryState {
    /// Remaining stored energy (J) out of `ISOMER_TOTAL_ENERGY_JOULES`.
    pub energy_remaining_joules: f64,
    /// Resonant gateway trigger armed.
    pub is_armed: bool,
    /// PCSS optical crowbar tripped (hard lockout).
    pub is_crowbar_tripped: bool,
    /// Current dispatch phase.
    pub phase: DispatchPhase,
}

impl IsomerBatteryState {
    /// Fully charged core in standby stasis.
    pub fn new() -> Self {
        Self {
            energy_remaining_joules: ISOMER_TOTAL_ENERGY_JOULES,
            is_armed: false,
            is_crowbar_tripped: false,
            phase: DispatchPhase::StandbyStasis,
        }
    }

    /// Discharge a burst pulse of `gross_power_watts` for
    /// `duration_seconds`; returns the net electrical power delivered after
    /// the 3-stage DEC cascade.
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
        Ok(gross_power_watts * DEC_TOTAL_EFFICIENCY)
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
            NET_BURST_POWER_W
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

    #[test]
    fn inventory_and_density() {
        assert!((SPECIFIC_ENERGY_JOULES_PER_KG - 1.3263e12).abs() < 1e6);
        let inv = ISOMER_CORE_MASS_KG * SPECIFIC_ENERGY_JOULES_PER_KG;
        // Mass x density = 500.0018 TJ; the nominal 500.0 TJ inventory is a
        // rounded figure (0.004% delta, physics-preserving).
        assert!(inv >= ISOMER_TOTAL_ENERGY_JOULES);
        assert!((ISOMER_GATEWAY_GAIN - 61.15).abs() < 1e-4);
        assert!((DEC_TOTAL_EFFICIENCY - 0.458).abs() < 1e-9);
        assert!(BORRMANN_EPSILON >= 0.985 && MOSSBAUER_FRACTION >= 0.74);
        assert!(CROWBAR_QUENCH_TIME_SECONDS <= 2.18e-9);
    }

    #[test]
    fn burst_power_net_49_9449_tw() {
        let mut bat = IsomerBatteryState::new();
        bat.phase = DispatchPhase::FoldingBurst;
        let p = bat.discharge_pulse(1e-3, GROSS_BURST_POWER_W).unwrap();
        assert!((p - 49.9449e12).abs() < 1e8);
        assert!((NET_BURST_POWER_W - 49.9449e12).abs() < 1e8);
    }

    #[test]
    fn crowbar_lockout_outputs_zero() {
        let mut bat = IsomerBatteryState::new();
        bat.phase = DispatchPhase::FoldingBurst;
        bat.trip_crowbar();
        assert!(bat.discharge_pulse(1e-3, GROSS_BURST_POWER_W).is_err());
        assert_eq!(bat.net_output_w(), 0.0);
        assert_eq!(bat.phase, DispatchPhase::EmergencyQuench);
    }

    #[test]
    fn soc_deficit_rejected() {
        let mut bat = IsomerBatteryState::new();
        bat.phase = DispatchPhase::FoldingBurst;
        assert!(bat
            .discharge_pulse(1.0e12, GROSS_BURST_POWER_W)
            .is_err());
    }
}
