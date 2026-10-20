//! 8×8 InP/InGaAs Photonic & HBT Boundary State Manipulator — PIC array topology,
//! RF spatial phase gradients, spatial coherence factor, and electronic beam steering.

#![forbid(unsafe_code)]

use core::f64::consts::PI;

pub const SPEED_OF_LIGHT_MPS: f64 = 299_792_458.0;
pub const DFB_LAMBDA_0_M: f64 = 1550.00e-9;
pub const DFB_NU_0_HZ: f64 = 193.414489e12;
pub const CW_PROBE_POWER_MW: f64 = 15.0;
pub const MMI_SPLITTER_MAX_LOSS_DB: f64 = 0.42;

pub const NUM_EMITTERS: usize = 64;
pub const ARRAY_ROWS: usize = 8;
pub const ARRAY_COLS: usize = 8;

pub const RF_SYNTH_FREQ_HZ: f64 = 40.0e9;
pub const V_PI_VOLTS: f64 = 1.65;
pub const INTERACTION_LENGTH_MM: f64 = 3.20;
pub const EO_TRANSMISSION_DB: f64 = -2.18;

pub const INTERPOSER_PITCH_M: f64 = 127.0e-6;
pub const ACTIVE_MESA_PITCH_M: f64 = 50.0e-6;

pub const SSB_PHASE_NOISE_10KHZ_DBC_HZ: f64 = -119.42;
pub const MAX_PHASE_VARIANCE_RAD2: f64 = 1.84e-7;
pub const MIN_SPATIAL_COHERENCE: f64 = 0.9995;

pub const V_ON_VOLTS: f64 = 3.800;
pub const V_SAT_VOLTS: f64 = 7.400;
pub const PID_KP_V_PER_RAD: f64 = 1.85;
pub const ACOUSTIC_DECAY_TIME_NS: f64 = 110.00;
pub const RESTORATION_TIME_NS: f64 = 9.24;

/// Computes the spatial phase gradient for beam steering angle theta_steer (rad).
#[inline]
pub fn spatial_phase_gradient(theta_steer_rad: f64) -> f64 {
    (2.0 * PI * RF_SYNTH_FREQ_HZ / SPEED_OF_LIGHT_MPS) * INTERPOSER_PITCH_M * theta_steer_rad.sin()
}

/// Quantizes an arbitrary phase angle into a 16-bit hardware IQ word.
#[inline]
pub fn phase_to_iq16(phi_rad: f64) -> u16 {
    let normalized = phi_rad.rem_euclid(2.0 * PI) / (2.0 * PI);
    (normalized * 65536.0).floor() as u16
}

/// Computes the optical array spatial coherence factor:
/// C = exp(-0.5 * Var(phi)).
#[inline]
pub fn spatial_coherence_factor(var_phi_rad2: f64) -> f64 {
    (-0.5 * var_phi_rad2).exp()
}

/// Computes the integrated timing jitter in femtoseconds:
/// sigma_jitter = sqrt(Var(phi)) / (2 * pi * f_RF).
#[inline]
pub fn integrated_timing_jitter_fs(var_phi_rad2: f64) -> f64 {
    (var_phi_rad2.sqrt() / (2.0 * PI * RF_SYNTH_FREQ_HZ)) * 1.0e15
}

/// Returns the centered spatial coordinate (x_m, y_n) in meters for emitter (m, n).
#[inline]
pub fn emitter_centered_coords_m(m: usize, n: usize) -> (f64, f64) {
    let xm = (m as f64 - 3.5) * ACTIVE_MESA_PITCH_M;
    let yn = (n as f64 - 3.5) * ACTIVE_MESA_PITCH_M;
    (xm, yn)
}

/// Computes element phase command:
/// theta_mn = -k0 * (x_m * sin(theta_x) + y_n * sin(theta_y)) + phi_init.
#[inline]
pub fn element_phase_command(
    m: usize,
    n: usize,
    theta_x_rad: f64,
    theta_y_rad: f64,
    phi_init_rad: f64,
) -> f64 {
    let (xm, yn) = emitter_centered_coords_m(m, n);
    let k0 = 2.0 * PI / DFB_LAMBDA_0_M;
    -k0 * (xm * theta_x_rad.sin() + yn * theta_y_rad.sin()) + phi_init_rad
}

/// Linearly maps the commanded element phase to the HBT driving voltage:
/// V_mn = V_on + (V_sat - V_on)/(2*pi) * mod(theta_mn, 2*pi).
#[inline]
pub fn element_drive_voltage(phase_cmd_rad: f64) -> f64 {
    let phase_mod = phase_cmd_rad.rem_euclid(2.0 * PI);
    V_ON_VOLTS + ((V_SAT_VOLTS - V_ON_VOLTS) / (2.0 * PI)) * phase_mod
}

/// Checks metric detuning interlock: trips if max |delta_mu_i| > 1.0e-12.
#[inline]
pub fn check_metric_detuning_interlock(max_delta_mu: f64) -> bool {
    max_delta_mu > 1.0e-12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn emitter_phase_gradient_and_jitter() {
        let theta_test = 0.15; // rad
        let grad = spatial_phase_gradient(theta_test);
        let expected_grad = (2.0 * PI * 40.0e9 / SPEED_OF_LIGHT_MPS) * 127.0e-6 * theta_test.sin();
        assert!((grad - expected_grad).abs() < 1e-12);

        let iq0 = phase_to_iq16(0.0);
        assert_eq!(iq0, 0);
        let iq_half = phase_to_iq16(PI);
        assert_eq!(iq_half, 32768);

        let coherence = spatial_coherence_factor(MAX_PHASE_VARIANCE_RAD2);
        assert!(coherence >= MIN_SPATIAL_COHERENCE);
        assert!((coherence - 0.999999908).abs() < 1e-6);

        let jitter_fs = integrated_timing_jitter_fs(MAX_PHASE_VARIANCE_RAD2);
        assert!((jitter_fs - 1.707).abs() < 0.05);

        for m in 0..8 {
            for n in 0..8 {
                let cmd = element_phase_command(m, n, 0.05, -0.05, 0.1);
                let v = element_drive_voltage(cmd);
                assert!((V_ON_VOLTS..=V_SAT_VOLTS).contains(&v));
            }
        }

        assert!(check_metric_detuning_interlock(1.5e-12));
        assert!(!check_metric_detuning_interlock(5.0e-13));
    }
}
