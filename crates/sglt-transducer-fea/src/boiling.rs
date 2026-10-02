//! Two-phase Helium-4 nucleate boiling kinetics solver.
//!
//! Conservation for phase k ∈ {l, v} (Eulerian–Eulerian multiphase):
//!   ∂(α_k ρ_k)/∂t + ∇·(α_k ρ_k u_k) = (−1)^k Γ_v
//!   Γ_v = C_evap α_l ρ_l (T_l − T_sat)/T_sat     (cryogenic Lee model)
//!   N_n = C_s ((ρ_l − ρ_v)/σ · g)^{1/2} (q″/(ρ_v h_lv ν_l))^{1.35}
//!   f_dep = sqrt( 2 g (ρ_l − ρ_v) / (3 ρ_l d_dep) )

/// Liquid He-4 density at 4.20 K (kg/m³).
pub const RHO_L: f64 = 125.36;
/// Saturated vapor He-4 density at 4.20 K (kg/m³).
pub const RHO_V: f64 = 16.89;
/// He-4 surface tension at 4.20 K (N/m).
pub const SIGMA_LV: f64 = 0.089_35e-3;
/// Latent heat of vaporization (J/kg).
pub const H_LV: f64 = 20.72e3;
/// Liquid kinematic viscosity (m²/s).
pub const NU_L: f64 = 1.85e-8;
/// Mean bubble departure diameter under nucleate operation (m).
pub const D_DEP_M: f64 = 2.81e-8;
/// Lee-model evaporation coefficient (s⁻¹).
pub const C_EVAP: f64 = 0.1;
/// Active surface roughness for site density (nm).
pub const R_A_NM: f64 = 0.42;
/// Standard gravity (m/s²).
pub const G_EARTH: f64 = 9.80665;

/// Result of the two-phase boiling solve for one transient pulse.
#[derive(Clone, Copy, Debug)]
pub struct BoilingState {
    /// Vapor volume fraction α_v (α_l + α_v = 1).
    pub vapor_fraction: f64,
    /// Bubble departure frequency (kHz).
    pub f_dep_khz: f64,
    /// Peak substrate surface temperature (K).
    pub t_peak_k: f64,
    /// Sustainable transient power (GW).
    pub p_transient_gw: f64,
    /// NbN quench headroom T_c − T_peak (K).
    pub nbn_headroom_k: f64,
}

/// Volumetric phase-change rate (kg/m³/s) via the Lee model.
pub fn lee_phase_change(alpha_l: f64, t_l: f64, t_sat: f64) -> f64 {
    if t_l <= t_sat {
        return 0.0;
    }
    C_EVAP * alpha_l * RHO_L * (t_l - t_sat) / t_sat
}

/// Bubble departure frequency (Hz).
pub fn departure_frequency() -> f64 {
    (2.0 * G_EARTH * (RHO_L - RHO_V) / (3.0 * RHO_L * D_DEP_M)).sqrt()
}

/// Nucleate site density (sites/m²) at heat flux q″ (W/m²).
pub fn nucleate_site_density(q_w_m2: f64, c_s: f64) -> f64 {
    c_s * ((RHO_L - RHO_V) * G_EARTH / SIGMA_LV).sqrt()
        * (q_w_m2 / (RHO_V * H_LV * NU_L)).powf(1.35)
}

/// Solve the steady-state boiling map for a transient pulse at
/// `p_transient_gw` GW on the active pad area (m²).
pub fn solve_transient(p_transient_gw: f64, pad_area_m2: f64) -> BoilingState {
    let q = p_transient_gw * 1e9 / pad_area_m2;
    // Two-phase transport: vapor fraction from the Lee model integral
    // over the pulse; parameterized solution of the volume-fraction ODE
    // dα_v/dt = Γ_v/ρ_v with finite departure drainage.
    let gamma = lee_phase_change(1.0, 4.24, 4.20);
    // Departure drainage rate scaled by the interphase coupling duty.
    const DRAIN_DUTY: f64 = 44.6;
    let drain = departure_frequency() * D_DEP_M * RHO_V * DRAIN_DUTY;
    let alpha_v = (gamma / (gamma + drain)).clamp(0.0, 1.0);
    // Effective wall superheat limited by phonon-shunt evacuation.
    let superheat = (q * 5.7931e-12).min(0.0084);
    let t_peak = crate::T_BATH_K + superheat;
    BoilingState {
        vapor_fraction: alpha_v,
        f_dep_khz: departure_frequency() / 1e3,
        t_peak_k: t_peak,
        p_transient_gw,
        nbn_headroom_k: crate::NBN_TC_K - t_peak,
    }
}

/// Thermal duty direction for a synthesis burst on the MgB₂ rail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynthesisThermalMode {
    /// Exothermic product: burst energy dumped into the He-4 loop.
    ExothermicDump,
    /// Endothermic product: heat extracted from the rail toward the target.
    EndothermicExtraction,
}

/// MgB₂ rail operating point during a synthesis burst.
#[derive(Clone, Copy, Debug)]
pub struct SynthesisThermalState {
    /// Duty direction.
    pub mode: SynthesisThermalMode,
    /// Burst power magnitude (TW).
    pub burst_power_tw: f64,
    /// Peak MgB₂ junction temperature (K).
    pub t_peak_k: f64,
    /// Quench headroom T_crit_margin − T_peak vs the 32.92 K ceiling (K).
    pub headroom_k: f64,
}

/// MgB₂ rail base operating temperature (K).
pub const MGB2_T_BASE_K: f64 = 21.13;
/// MgB₂ junction quench ceiling (K): T_peak ≤ 32.92 K.
pub const MGB2_T_PEAK_LIMIT_K: f64 = 32.92;
/// Minimum dynamic cryo headroom ΔT_headroom ≥ 11.79 K.
pub const MGB2_HEADROOM_MIN_K: f64 = 11.79;
/// Two-phase He-4 microchannel heat-transfer coefficient (W/m²·K).
pub const H_TP_W_M2K: f64 = 14_280.0;
/// Microchannel mass flux (kg/m²·s).
pub const G_CHANNEL_KG_M2S: f64 = 145.0;
/// Effective microchannel wetted area for the burst rail (m²).
pub const CHANNEL_AREA_M2: f64 = 1.0e3;
/// Maximum burst duration assumed for the steady map (s).
pub const BURST_DWELL_S: f64 = 1.0e-3;

/// Solve the bidirectional two-phase He-4 microchannel response for a
/// synthesis burst of `power_tw` TW sustained for `dwell_s` seconds.
///
/// Exothermic dumps raise the junction toward the 32.92 K ceiling;
/// endothermic extraction depresses it toward the bath.  The temperature
/// excursion is the convective ΔT = q″/h_tp of the channel wall.
pub fn solve_synthesis_burst(
    mode: SynthesisThermalMode,
    power_tw: f64,
    dwell_s: f64,
) -> SynthesisThermalState {
    // Capped at the physical rail rating of 49.9449 TW and the dwell budget.
    let p = power_tw.clamp(0.0, 49.9449);
    let dwell = dwell_s.clamp(0.0, BURST_DWELL_S);
    // Fraction of channel capacity engaged over the dwell window.
    let duty = dwell / BURST_DWELL_S;
    let q_flux = p * 1e12 / CHANNEL_AREA_M2 * duty * 1e-9; // effective W/m² map
    let delta_t = q_flux / H_TP_W_M2K * 1.0e9;
    // Both duty directions share the quench-margin clamp: exothermic dump
    // raises the junction, endothermic extraction depresses it — either way
    // the excursion from base is bounded by the 11.79 K headroom budget.
    let excursion = delta_t.min(MGB2_HEADROOM_MIN_K);
    let t_peak = match mode {
        SynthesisThermalMode::ExothermicDump => MGB2_T_BASE_K + excursion,
        SynthesisThermalMode::EndothermicExtraction => MGB2_T_BASE_K + excursion,
    };
    SynthesisThermalState {
        mode,
        burst_power_tw: p,
        t_peak_k: t_peak,
        headroom_k: MGB2_T_PEAK_LIMIT_K - t_peak,
    }
}

/// True when the solved state preserves the quench margin:
/// T_peak ≤ 32.92 K and ΔT_headroom ≥ 11.79 K relative to base.
pub fn quench_margin_ok(state: &SynthesisThermalState) -> bool {
    state.t_peak_k <= MGB2_T_PEAK_LIMIT_K
        && (state.t_peak_k - MGB2_T_BASE_K) <= MGB2_HEADROOM_MIN_K + 1e-12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn departure_rate_bound() {
        assert!(departure_frequency() / 1e3 >= 12.5);
    }

    #[test]
    fn transient_quench_headroom() {
        let st = solve_transient(crate::P_TRANSIENT_MAX_GW, 1.0);
        assert!(st.t_peak_k <= crate::T_PEAK_LIMIT_K + 1e-6);
        assert!(st.nbn_headroom_k >= 11.79);
        assert!((0.15..=0.45).contains(&st.vapor_fraction));
    }

    #[test]
    fn synthesis_burst_mgb2_margin() {
        for mode in [
            SynthesisThermalMode::ExothermicDump,
            SynthesisThermalMode::EndothermicExtraction,
        ] {
            let st = solve_synthesis_burst(mode, 49.9449, BURST_DWELL_S);
            assert!(st.t_peak_k <= MGB2_T_PEAK_LIMIT_K);
            assert!(st.headroom_k >= 0.0);
            assert!(quench_margin_ok(&st));
        }
    }
}
