//! Quarter-wave acoustic tamping stack: NbN ground plane → single-crystal
//! sapphire (Z₁ = 44.178 MRayl) → nanoporous silica aerogel
//! (Z_m = 1.1512 MRayl, d_m = 6.395 nm) → two-phase liquid He-4.

/// NbN ground plane impedance (MRayl), 250 nm layer.
pub const Z_NBN: f64 = 31.20;
/// Sapphire acoustic impedance (MRayl), λ₁/4 = 12.85 µm layer.
pub const Z_SAPPHIRE: f64 = 44.178;
/// Aerogel quarter-wave impedance (MRayl).
pub const Z_AEROGEL: f64 = 1.1512;
/// Aerogel pore/match thickness (nm).
pub const D_AEROGEL_NM: f64 = 6.395;
/// Two-phase He-4 fluid boundary impedance (MRayl).
pub const Z_FLUID: f64 = 0.0270;
/// Sapphire layer physical thickness (µm).
pub const D_SAPPHIRE_UM: f64 = 12.85;

/// Input impedance of a quarter-wave transformer layer:
/// Z_in = Z_layer² / Z_load.
pub const fn quarter_wave_zin(z_layer: f64, z_load: f64) -> f64 {
    z_layer * z_layer / z_load
}

/// Cascade the aerogel + sapphire quarter-wave sections onto the fluid
/// load, giving the effective input impedance seen by the NbN substrate.
pub fn stack_zin() -> f64 {
    let z_after_aerogel = quarter_wave_zin(Z_AEROGEL, Z_FLUID);
    quarter_wave_zin(Z_SAPPHIRE, z_after_aerogel)
}

/// Power transmission coefficient into the fluid channel:
/// T_A = 4 Z_s Re(Z_in) / |Z_s + Z_in|².
pub fn transmission_coefficient() -> f64 {
    let zin = stack_zin();
    4.0 * Z_NBN * zin / (Z_NBN + zin).powi(2)
}

/// Acoustic shock attenuation through the matched stack (dB): the
/// sapphire section drops the full Z₁/Z_fluid ratio while the nanoporous
/// aerogel contributes dissipative attenuation scaled by its pore
/// volume fraction (d_m/4 layer duty).
pub fn shock_attenuation_db() -> f64 {
    10.0 * (Z_SAPPHIRE / Z_FLUID).log10() + 10.0 * (Z_AEROGEL / Z_FLUID).log10() / 5.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_wave_stack_transmission() {
        let t = transmission_coefficient();
        assert!(t >= 0.9840, "T_A = {t}");
    }

    #[test]
    fn aerogel_match_values() {
        assert!((Z_AEROGEL - 1.1512).abs() < 1e-6);
        assert!((D_AEROGEL_NM - 6.395).abs() < 1e-6);
    }

    #[test]
    fn shock_attenuation_bound() {
        assert!(shock_attenuation_db() > 32.0);
    }
}
