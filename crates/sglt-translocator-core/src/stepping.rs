//! Boundary bit-stepping capacity engine.
//!
//! Every discrete boundary transition dissipates `P_BIT = 1.842 W/bit` at
//! the bare-metal microkernel frequency `F_TICK = 50.518 kHz`.  The
//! continuous LANR baseload (999.054 kW net, 906.00 kW entropy debt)
//! sustains only the housekeeping rate; the ^178m2Hf isomer burst rail
//! (sys1own/shbt-warp upstream) raises capacity from 50,517 bits/step to
//! 2.7114e13 bits/step, enabling N_local in [1e23, 1e28] de-rendering.

use crate::minjerk;

/// Irreducible dissipation per stepped boundary bit (W/bit).
pub const P_BIT: f64 = 1.842;
/// Bare-metal microkernel execution frequency (Hz).
pub const F_TICK_HZ: f64 = 50.518e3;
/// Continuous LANR net electrical power (W): 999.054 kW.
pub const P_LANR_NET_W: f64 = 999.054e3;
/// Non-sheddable Landauer entropy-debt floor (W): 906.00 kW.
pub const P_DEBT_W: f64 = 906.00e3;
/// Baseload stepping capacity (bits/step) on the LANR rail alone.
pub const BASELINE_BITS_PER_STEP: u64 = 50_517;
/// Burst stepping capacity (bits/step) on the isomer rail.
pub const BURST_BITS_PER_STEP: f64 = 2.7114e13;
/// Burst throughput floor (bits/s): Phi = dN * f_tick.
pub const BURST_THROUGHPUT_BPS: f64 = 1.3698e18;

/// Discrete boundary bit capacity per execution tick:
///   dN(k) = floor((P_net(k) - P_debt) / P_bit).
pub fn compute_step_capacity(p_net_watts: f64, p_debt_watts: f64) -> u64 {
    if p_net_watts <= p_debt_watts {
        return 0;
    }
    ((p_net_watts - p_debt_watts) / P_BIT).floor() as u64
}

/// Bits/step on the continuous LANR-only rail.
pub fn baseload_capacity() -> u64 {
    compute_step_capacity(P_LANR_NET_W, P_DEBT_W)
}

/// Bits/step while the isomer burst rail is discharging
/// (`net_burst_w` = net DEC electrical power in W).
pub fn burst_capacity(net_burst_w: f64) -> u64 {
    compute_step_capacity(P_LANR_NET_W + net_burst_w, P_DEBT_W)
}

/// Burst throughput (bits/s) at the microkernel tick frequency.
pub fn burst_throughput_bps(net_burst_w: f64) -> f64 {
    burst_capacity(net_burst_w) as f64 * F_TICK_HZ
}

/// Fractional bits/stepped position within one tick, coupled to the
/// 5th-order minimum-jerk trajectory s(tau) = 10t^3 - 15t^4 + 6t^5 so the
/// optical gating stays synchronized with boundary braid transitions.
pub fn jerk_synchronized_bits(bits_per_step: u64, tau: f64) -> f64 {
    bits_per_step as f64 * minjerk::s(tau)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sglt_lanr_power::isomer::NET_BURST_POWER_W;

    #[test]
    fn baseload_is_50517_bits() {
        assert_eq!(baseload_capacity(), BASELINE_BITS_PER_STEP);
    }

    #[test]
    fn burst_scales_to_2_7114e13() {
        let d_n = burst_capacity(NET_BURST_POWER_W);
        assert!((d_n as f64 - BURST_BITS_PER_STEP).abs() / BURST_BITS_PER_STEP < 5e-4);
        let phi = burst_throughput_bps(NET_BURST_POWER_W);
        assert!(phi >= 1.3697e18);
    }

    #[test]
    fn jerk_profile_bounds() {
        assert_eq!(jerk_synchronized_bits(1000, 0.0), 0.0);
        assert_eq!(jerk_synchronized_bits(1000, 1.0), 1000.0);
    }
}
