// crates/sglt-lanr-power/src/shunt.rs
#![forbid(unsafe_code)]

pub const SYMPLECTIC_SUPPRESSION_S: f64 = 100.0 / 1089.0;
pub const DARK_LEDGER_REJECTION_FRAC: f64 = 989.0 / 1089.0;
pub const BORRMANN_EFFICIENCY: f64 = 0.9852;
pub const SMES_RECOVERY_EFFICIENCY: f64 = 0.9420;
pub const SENSIBLE_HEAT_FRACTION: f64 =
    SYMPLECTIC_SUPPRESSION_S * (1.0 - BORRMANN_EFFICIENCY) * (1.0 - SMES_RECOVERY_EFFICIENCY);

pub const BASELINE_CRYO_TEMP_K: f64 = 21.13;
pub const CRITICAL_TEMP_LIMIT_K: f64 = 32.92;
pub const MAX_LHE_MASS_FLOW_KG_S: f64 = 112.5;
pub const LHE_SPECIFIC_HEAT_J_KG_K: f64 = 5200.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LandauerShuntTelemetry {
    pub gross_landauer_power_w: f64,
    pub dark_ledger_shunted_w: f64,
    pub sensible_heat_w: f64,
    pub operating_temp_k: f64,
    pub lhe_mass_flow_kg_s: f64,
}

pub fn evaluate_symplectic_landauer_shunt(
    gross_landauer_power_w: f64,
    lhe_mass_flow_kg_s: f64,
) -> Result<LandauerShuntTelemetry, &'static str> {
    if lhe_mass_flow_kg_s <= 0.0 || lhe_mass_flow_kg_s > MAX_LHE_MASS_FLOW_KG_S {
        return Err("LHe mass flow rate outside (0.0, 112.5] kg/s operating envelope");
    }

    let dark_ledger_shunted_w = gross_landauer_power_w * DARK_LEDGER_REJECTION_FRAC;
    let sensible_heat_w = gross_landauer_power_w * SENSIBLE_HEAT_FRACTION;
    let delta_t_k = sensible_heat_w / (lhe_mass_flow_kg_s * LHE_SPECIFIC_HEAT_J_KG_K);
    let operating_temp_k = BASELINE_CRYO_TEMP_K + delta_t_k;

    if operating_temp_k > CRITICAL_TEMP_LIMIT_K {
        return Err("Cryogenic operating temperature exceeded 32.92 K ceiling");
    }

    Ok(LandauerShuntTelemetry {
        gross_landauer_power_w,
        dark_ledger_shunted_w,
        sensible_heat_w,
        operating_temp_k,
        lhe_mass_flow_kg_s,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shunt_rejection_and_fractions() {
        assert!((SYMPLECTIC_SUPPRESSION_S - 100.0 / 1089.0).abs() < 1e-12);
        assert!((DARK_LEDGER_REJECTION_FRAC - 989.0 / 1089.0).abs() < 1e-12);
        assert!((SENSIBLE_HEAT_FRACTION - 7.882460973370064e-5).abs() < 1e-10);
    }

    #[test]
    fn test_cryo_operating_envelope() {
        let telem = evaluate_symplectic_landauer_shunt(1.5e6, 112.5).unwrap();
        assert!(telem.operating_temp_k <= 28.40);
        assert!(telem.operating_temp_k < CRITICAL_TEMP_LIMIT_K);
    }
}
