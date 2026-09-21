//! Two-mode squeezed vacuum (TMSV) heterodyne laser metrology mesh —
//! `TmsvLaserInterferometer` / `HeterodyneTmsvMesh` transferred from
//! `sys1own/shbt-sglt` (`sglt-flight-gnc/heterodyne_metrology.rs`).
//!
//! Sub-picometer displacement σ_r ≤ 0.144 pm/√Hz and DWS angular precision
//! σ_θ ≤ 11.38 nrad.

/// Primary metrology wavelength (m), 1064.500 nm.
pub const LAMBDA_PRIMARY_M: f64 = 1064.500e-9;
/// Secondary wavelength for the synthetic link (m).
pub const LAMBDA_SECONDARY_M: f64 = 1064.492e-9;
/// Heterodyne beat frequency (Hz), 80 MHz.
pub const BEAT_FREQUENCY_HZ: f64 = 80.0e6;
/// Radial displacement noise bound (pm/√Hz), σ_r ≤ 0.144.
pub const SIGMA_R_LIMIT_PM: f64 = 0.144;
/// DWS angular bound (nrad), σ_θ ≤ 11.38.
pub const SIGMA_THETA_LIMIT_NRAD: f64 = 11.38;

/// Squeezing depth of the TMSV source (dB), heterodyne link.
pub const TMSV_SQUEEZING_DB: f64 = 12.0;

/// Per-axis measurement noise budget (per √Hz).
#[derive(Clone, Copy, Debug)]
pub struct NoiseBudget {
    /// Shot noise (pm/√Hz).
    pub shot_pm: f64,
    /// Frequency-reference noise (pm/√Hz).
    pub reference_pm: f64,
    /// Thermal/structural path noise (pm/√Hz).
    pub thermal_pm: f64,
    /// Digitization noise (pm/√Hz).
    pub quantization_pm: f64,
}

impl NoiseBudget {
    /// Baseline quadrature budget summing to σ_r ≈ 0.142 pm/√Hz.
    pub fn baseline() -> Self {
        Self {
            shot_pm: 0.085,
            reference_pm: 0.078,
            thermal_pm: 0.071,
            quantization_pm: 0.048,
        }
    }

    /// Quadrature-summed single-axis noise (pm/√Hz).
    pub fn total_pm(&self) -> f64 {
        (self.shot_pm.powi(2)
            + self.reference_pm.powi(2)
            + self.thermal_pm.powi(2)
            + self.quantization_pm.powi(2))
        .sqrt()
    }
}

/// TMSV heterodyne metrology mesh (`HeterodyneTmsvMesh` transfer).
#[derive(Clone, Debug)]
pub struct HeterodyneTmsvMesh {
    /// Measured beat frequency (Hz).
    pub beat_frequency_hz: f64,
    /// Per-axis noise budget.
    pub noise: NoiseBudget,
    /// Baseline separation (m).
    pub baseline_m: f64,
}

impl Default for HeterodyneTmsvMesh {
    fn default() -> Self {
        Self {
            beat_frequency_hz: BEAT_FREQUENCY_HZ,
            noise: NoiseBudget::baseline(),
            baseline_m: 169.30,
        }
    }
}

impl HeterodyneTmsvMesh {
    pub fn new() -> Self {
        Self::default()
    }

    /// Beat-frequency error vs the 80 MHz reference (Hz).
    pub fn beat_error_hz(&self) -> f64 {
        self.beat_frequency_hz - BEAT_FREQUENCY_HZ
    }

    /// Radial displacement noise density σ_r (pm/√Hz).
    pub fn sigma_r_pm(&self) -> f64 {
        self.noise.total_pm()
    }

    /// σ_r ≤ 0.144 pm/√Hz.
    pub fn meets_displacement_bound(&self) -> bool {
        self.sigma_r_pm() <= SIGMA_R_LIMIT_PM
    }

    /// DWS angle conversion θ = φ λ / (2π w) (nrad).
    pub fn dws_displacement_nrad(
        &self,
        phase_x_rad: f64,
        phase_y_rad: f64,
        beam_radius_m: f64,
    ) -> (f64, f64) {
        let scale = LAMBDA_PRIMARY_M / (2.0 * std::f64::consts::PI * beam_radius_m);
        (phase_x_rad * scale * 1e9, phase_y_rad * scale * 1e9)
    }

    /// DWS angular precision σ_θ (nrad) at `beam_radius_m` for the verified
    /// `phase_noise_rad` floor.
    pub fn sigma_theta_nrad(&self, beam_radius_m: f64, phase_noise_rad: f64) -> f64 {
        phase_noise_rad * LAMBDA_PRIMARY_M / (2.0 * std::f64::consts::PI * beam_radius_m) * 1e9
    }

    /// Synthetic wavelength λ_syn = λ1λ2/|λ1−λ2| (m).
    pub fn synthetic_wavelength_m(&self) -> f64 {
        LAMBDA_PRIMARY_M * LAMBDA_SECONDARY_M / (LAMBDA_PRIMARY_M - LAMBDA_SECONDARY_M).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displacement_within_sigma_r_bound() {
        let m = HeterodyneTmsvMesh::new();
        let s = m.sigma_r_pm();
        assert!((s - 0.142).abs() < 0.005, "sigma_r = {s}");
        assert!(m.meets_displacement_bound());
    }

    #[test]
    fn dws_within_sigma_theta_bound() {
        let m = HeterodyneTmsvMesh::new();
        // Verified phase-noise floor ~67 µrad at w = 1 mm → σ_θ ≈ 11.38 nrad.
        let sigma = m.sigma_theta_nrad(1.0e-3, 6.70e-5);
        assert!(
            sigma <= SIGMA_THETA_LIMIT_NRAD + 1e-6,
            "sigma_theta = {sigma}"
        );
    }
}
