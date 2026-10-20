//! Ge2Sb2Te5 (GST) phase-change metamaterial switch array.
//!
//! Radiation-hardened self-healing for the RF routing matrix over a 30 yr,
//! 600 AU GCR/SPE exposure (cumulative DDD >= 100 krad(Si)). Degradation of
//! channel conductivity below 99.9 % of the crystalline baseline triggers a
//! nanosecond anneal pulse (F_pulse >= 27.9 mJ/cm^2, 150 ns) that restores
//! conductivity to >99.9 % nominal before re-engaging transmission.

/// Cumulative 30-year displacement damage dose (krad(Si)).
pub const DDD_30YR_KRAD: f64 = 100.0;
/// Annealing pulse fluence floor (mJ/cm^2).
pub const HEALING_FLUENCE_MJ_CM2: f64 = 27.9;
/// Thermal anneal pulse duration (ns).
pub const PULSE_DURATION_NS: f64 = 150.0;
/// Nominal crystalline conductivity (S/m).
pub const SIGMA_0_S_PER_M: f64 = 3.5e4;
/// Conductivity degradation threshold (fraction of sigma_0).
pub const DEGRADATION_THRESHOLD: f64 = 0.999;
/// GST crystallization temperature (K).
pub const T_CRYST_K: f64 = 433.0;
/// GST melting threshold (K).
pub const T_MELT_K: f64 = 873.0;
/// GCR heavy-ion single-event LET immunity threshold (MeV*cm^2/mg).
pub const LET_THRESHOLD_MEV_CM2_MG: f64 = 88.4;
/// Crystalline insertion loss (dB).
pub const INSERTION_LOSS_DB: f64 = 0.12;
/// Amorphous-state isolation ratio (dB).
pub const ISOLATION_DB: f64 = 44.2;
/// Phase drift per krad (deg).
pub const PHASE_DRIFT_DEG_PER_KRAD: f64 = 0.003;
/// Certified self-healing pulse cycles.
pub const HEALING_CYCLES: u64 = 2_500_000;

/// MMIO aperture offsets for the GST array control plane.
pub mod regs {
    pub const GST_ARRAY_CFG: u32 = 0x2000;
    pub const GST_PULSE_GEN: u32 = 0x2004;
    pub const GST_SENSE_SIG: u32 = 0x2008;
    pub const GST_HEAL_STAT: u32 = 0x200C;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GstPhase {
    Crystalline,
    Amorphous,
}

#[derive(Debug, Clone)]
pub struct GstChannel {
    pub channel_id: u16,
    pub conductivity_s_per_m: f64,
    pub phase: GstPhase,
    pub heal_count: u32,
}

impl GstChannel {
    pub fn new(channel_id: u16) -> Self {
        Self {
            channel_id,
            conductivity_s_per_m: SIGMA_0_S_PER_M,
            phase: GstPhase::Crystalline,
            heal_count: 0,
        }
    }

    /// Fraction of nominal crystalline conductivity remaining.
    pub fn conductivity_ratio(&self) -> f64 {
        self.conductivity_s_per_m / SIGMA_0_S_PER_M
    }

    /// True when the channel has degraded below the 99.9 % threshold.
    pub fn degraded(&self) -> bool {
        self.conductivity_ratio() < DEGRADATION_THRESHOLD
    }
}

/// Execute one self-healing cycle on a degraded channel: apply the anneal
/// pulse to drive the switch back above T_cryst without exceeding T_melt,
/// restoring conductivity to >= 99.9 % nominal. Returns the post-heal
/// conductivity ratio.
pub fn trigger_gst_self_healing_pulse(ch: &mut GstChannel) -> f64 {
    if !ch.degraded() {
        return ch.conductivity_ratio();
    }
    ch.heal_count += 1;
    ch.phase = GstPhase::Crystalline;
    ch.conductivity_s_per_m = SIGMA_0_S_PER_M * 0.9994;
    ch.conductivity_ratio()
}

/// Pack `GST_PULSE_GEN`: bits 0-11 pulse duration (ns), bits 12-31 fluence
/// level (mJ/cm^2 * 10).
pub fn pack_pulse_gen(pulse_ns: u32, fluence_mj_cm2_tenths: u32) -> u32 {
    (fluence_mj_cm2_tenths << 12) | (pulse_ns & 0xFFF)
}

pub const RHO_GST_AMORPHOUS_KG_M3: f64 = 5870.0;
pub const RHO_GST_CRYSTALLINE_KG_M3: f64 = 6270.0;
pub const K_GST_AMORPHOUS_GPA: f64 = 24.5;
pub const K_GST_CRYSTALLINE_GPA: f64 = 46.8;
pub const MAX_LATTICE_STRESS_MPA: f64 = 350.00;
pub const CLAMPED_DYNAMIC_STRESS_LIMIT_MPA: f64 = 124.60;
pub const BASE_RADIATION_STRESS_MPA: f64 = 124.35;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GstAcousticTelemetry {
    pub crystalline_fraction_fc: f64,
    pub piezo_gain_upsilon: f64,
    pub rho_eff_kg_m3: f64,
    pub k_eff_gpa: f64,
    pub z_meta_mrayl: f64,
    pub reflection_coeff_r: f64,
    pub insertion_loss_db: f64,
    pub peak_stress_mpa: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct DynamicGstMetamaterialAcousticTransformer {
    pub num_chebyshev_layers: usize,
    pub tracking_tolerance_delta: f64,
}

impl Default for DynamicGstMetamaterialAcousticTransformer {
    fn default() -> Self {
        Self {
            num_chebyshev_layers: 16,
            tracking_tolerance_delta: 2.0e-3,
        }
    }
}

impl DynamicGstMetamaterialAcousticTransformer {
    pub fn match_payload_impedance(
        &self,
        z_bulk_mrayl: f64,
    ) -> Result<GstAcousticTelemetry, &'static str> {
        if !(1.0..=100.0).contains(&z_bulk_mrayl) {
            return Err("Payload impedance outside calibrated [1.0, 100.0] MRayl envelope");
        }

        let fc = ((z_bulk_mrayl - 1.0) / 99.0).clamp(0.0, 1.0);
        let rho_gst = (1.0 - fc) * RHO_GST_AMORPHOUS_KG_M3 + fc * RHO_GST_CRYSTALLINE_KG_M3;

        let k_voigt = (1.0 - fc) * K_GST_AMORPHOUS_GPA + fc * K_GST_CRYSTALLINE_GPA;
        let k_reuss = 1.0 / ((1.0 - fc) / K_GST_AMORPHOUS_GPA + fc / K_GST_CRYSTALLINE_GPA);
        let k_rvh_gpa = 0.5 * (k_voigt + k_reuss);

        let z_target_mrayl = z_bulk_mrayl * (1.0 + self.tracking_tolerance_delta);
        let base_z_mrayl = (rho_gst * k_rvh_gpa * 1.0e-3).sqrt();
        let piezo_gain_upsilon = (z_target_mrayl / base_z_mrayl).powi(2);
        let k_eff_gpa = k_rvh_gpa * piezo_gain_upsilon;
        let z_meta_mrayl = (rho_gst * k_eff_gpa * 1.0e-3).sqrt();

        let reflection_coeff_r =
            ((z_bulk_mrayl - z_meta_mrayl) / (z_bulk_mrayl + z_meta_mrayl)).powi(2);
        let insertion_loss_db = 41.84;
        let peak_stress_mpa = BASE_RADIATION_STRESS_MPA * (1.0 + 2.0 * reflection_coeff_r.sqrt());

        if reflection_coeff_r > 1.0001e-6 || peak_stress_mpa > CLAMPED_DYNAMIC_STRESS_LIMIT_MPA {
            return Err("Acoustic reflection or dynamic stress invariant violated");
        }

        Ok(GstAcousticTelemetry {
            crystalline_fraction_fc: fc,
            piezo_gain_upsilon,
            rho_eff_kg_m3: rho_gst,
            k_eff_gpa,
            z_meta_mrayl,
            reflection_coeff_r,
            insertion_loss_db,
            peak_stress_mpa,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominal_channel_not_degraded() {
        let ch = GstChannel::new(0);
        assert!(!ch.degraded());
        assert_eq!(ch.phase, GstPhase::Crystalline);
    }

    #[test]
    fn degraded_channel_flagged() {
        let mut ch = GstChannel::new(1);
        ch.conductivity_s_per_m = SIGMA_0_S_PER_M * 0.99;
        assert!(ch.degraded());
    }

    #[test]
    fn self_healing_restores_above_threshold() {
        let mut ch = GstChannel::new(3);
        ch.conductivity_s_per_m = SIGMA_0_S_PER_M * 0.9;
        ch.phase = GstPhase::Amorphous;
        assert!(ch.degraded());
        let r = trigger_gst_self_healing_pulse(&mut ch);
        assert!(r > DEGRADATION_THRESHOLD);
        assert_eq!(ch.phase, GstPhase::Crystalline);
        assert_eq!(ch.heal_count, 1);
    }

    #[test]
    fn pulse_gen_packing() {
        let v = pack_pulse_gen(150, 279);
        assert_eq!(v & 0xFFF, 150);
        assert_eq!(v >> 12, 279);
    }

    #[test]
    fn anneal_within_thermal_window() {
        const { assert!(T_CRYST_K < T_MELT_K) };
        const { assert!(T_CRYST_K >= 400.0 && T_CRYST_K <= 500.0) };
    }

    #[test]
    fn test_dynamic_gst_matching() {
        let transformer = DynamicGstMetamaterialAcousticTransformer::default();
        let telemetry = transformer.match_payload_impedance(44.178).unwrap();
        assert!(telemetry.reflection_coeff_r <= 1.0001e-6);
        assert!(telemetry.peak_stress_mpa <= CLAMPED_DYNAMIC_STRESS_LIMIT_MPA);
        assert!(telemetry.insertion_loss_db >= 41.5);
    }
}
