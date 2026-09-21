//! Kapitza acoustic boundary resistance + entropic refrigeration —
//! `CryoCoolingModel` / `KapitzaInterfaceLayer` transferred from
//! `sys1own/shbt-exotic` (`refrigeration.rs`, `acoustic_impedance.rs`).
//!
//! P_cool = Γ_de · k_B · T_c · ln 2 at T_c = 15 mK, with a quarter-wave
//! matching layer Z₁ = √(Z_sapphire·Z_fluid) = 44.178 MRayl and an
//! intermediate polycrystalline Al₂O₃ layer Z_m = 1.1512 MRayl; quench
//! suppression for ≤ 142.08 MW field-collapse transients.

/// Boltzmann constant (J/K).
pub const KB_J_PER_K: f64 = 1.380_649e-23;
/// ln 2.
pub const LN2: f64 = std::f64::consts::LN_2;
/// Mixing-chamber operating temperature (K).
pub const MIXING_CHAMBER_K: f64 = 15.0e-3;
/// Sapphire-side acoustic impedance target (MRayl).
pub const Z1_MRAYL: f64 = 44.178;
/// Intermediate matching-layer impedance (MRayl).
pub const ZM_MRAYL: f64 = 1.1512;
/// Field-collapse transient rating (W).
pub const QUENCH_TRANSIENT_W: f64 = 142.08e6;

/// Quarter-wave Kapitza matching layer.
#[derive(Clone, Copy, Debug)]
pub struct KapitzaInterfaceLayer {
    /// Substrate (sapphire) impedance, MRayl.
    pub z_sapphire_mrayl: f64,
    /// Helium-bath (fluid) impedance, MRayl.
    pub z_fluid_mrayl: f64,
}

impl KapitzaInterfaceLayer {
    /// Optimal quarter-wave impedance Z₁ = √(Z_s · Z_f) (MRayl).
    pub fn z1_mrayl(&self) -> f64 {
        (self.z_sapphire_mrayl * self.z_fluid_mrayl).sqrt()
    }

    /// Power transmission coefficient across the matched boundary,
    /// T = 4 Z_s Z_f / (Z_s + Z_f)².
    pub fn transmission(&self) -> f64 {
        let (zs, zf) = (self.z_sapphire_mrayl, self.z_fluid_mrayl);
        4.0 * zs * zf / (zs + zf).powi(2)
    }
}

/// Entropic refrigeration model (`CryoCoolingModel` transfer).
#[derive(Clone, Debug)]
pub struct CryoCoolingModel {
    /// Operating cold temperature T_c (K).
    pub t_c: f64,
}

impl CryoCoolingModel {
    pub fn new() -> Self {
        Self {
            t_c: MIXING_CHAMBER_K,
        }
    }

    /// Energy shed per de-rendered bit at T_c: E = k_B T_c ln 2 (J).
    pub fn energy_per_bit(&self) -> f64 {
        KB_J_PER_K * self.t_c * LN2
    }

    /// Cooling power P_cool = Γ_de · k_B · T_c · ln 2 (W).
    pub fn cooling_power(&self, gamma_de: f64) -> f64 {
        gamma_de * self.energy_per_bit()
    }

    /// De-rendering rate required to sink `p_w` watts at T_c.
    pub fn required_rate(&self, p_w: f64) -> f64 {
        p_w / self.energy_per_bit()
    }

    /// Quench suppression: required Γ_de to absorb a field-collapse
    /// transient of `transient_w` watts (≤ 142.08 MW rating).
    pub fn quench_suppression_rate(&self, transient_w: f64) -> Result<f64, &'static str> {
        if transient_w > QUENCH_TRANSIENT_W {
            return Err("transient exceeds 142.08 MW suppression rating");
        }
        Ok(self.required_rate(transient_w))
    }
}

impl Default for CryoCoolingModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matched_layer_impedance() {
        // Z₁ = 44.178 MRayl against Z_m = 1.1512 fluid side:
        // Z_sapphire = Z₁² / Z_m ≈ 1694.8 MRayl.
        let layer = KapitzaInterfaceLayer {
            z_sapphire_mrayl: Z1_MRAYL * Z1_MRAYL / ZM_MRAYL,
            z_fluid_mrayl: ZM_MRAYL,
        };
        assert!((layer.z1_mrayl() - Z1_MRAYL).abs() < 1e-9);
        assert!(layer.transmission() > 0.0 && layer.transmission() <= 1.0);
    }

    #[test]
    fn cooling_power_scales_linearly() {
        let m = CryoCoolingModel::new();
        assert!((m.cooling_power(2.0) / m.cooling_power(1.0) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn quench_rating_enforced() {
        let m = CryoCoolingModel::new();
        assert!(m.quench_suppression_rate(QUENCH_TRANSIENT_W).is_ok());
        assert!(m.quench_suppression_rate(QUENCH_TRANSIENT_W * 2.0).is_err());
    }
}
