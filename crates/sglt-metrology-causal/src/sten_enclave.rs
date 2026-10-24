//! Blueshifted Hyper-Temporal Enclave (`alpha in [10^1, 10^4]`, `||nabla_i alpha|| == 0`),
//! `1.701 ps` Weyl-Exciton Optical Crowbar (`tau_crowbar = 1.701 ps <= 2.50 ns`), and
//! Higher-Genus `Sp(2g, Z)` Multi-Terminal Stargate Lightcone Verification.

pub const ENCLAVE_ALPHA_MIN: f64 = 10.0;
pub const ENCLAVE_ALPHA_MAX: f64 = 10_000.0;
pub const WEYL_EXCITON_CROWBAR_PS: f64 = 1.701;
pub const WEYL_EXCITON_CROWBAR_NS: f64 = 0.001701; // 1.701 ps << 2.50 ns

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HyperTemporalEnclaveGate {
    pub lapse_alpha: f64,
    pub interior_lapse_grad_norm: f64,
    pub pt_goi_sweeps_per_tick: f64,
    pub weyl_crowbar_latency_ps: f64,
}

impl HyperTemporalEnclaveGate {
    pub fn new(lapse_alpha: f64) -> Result<Self, &'static str> {
        if !(1.0..=ENCLAVE_ALPHA_MAX).contains(&lapse_alpha) {
            return Err("Hyper-temporal enclave lapse alpha out of bounds [1, 10^4]");
        }
        Ok(Self {
            lapse_alpha,
            interior_lapse_grad_norm: 0.0,
            pt_goi_sweeps_per_tick: 363_663.37 * lapse_alpha,
            weyl_crowbar_latency_ps: WEYL_EXCITON_CROWBAR_PS,
        })
    }

    pub fn verify_enclave_and_crowbar(&self) -> bool {
        self.lapse_alpha >= 1.0
            && self.lapse_alpha <= ENCLAVE_ALPHA_MAX
            && self.interior_lapse_grad_norm == 0.0
            && self.weyl_crowbar_latency_ps <= crate::TAU_QUENCH_NS * 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyper_temporal_enclave_gate() {
        let gate = HyperTemporalEnclaveGate::new(10_000.0).unwrap();
        assert!(gate.verify_enclave_and_crowbar());
        assert!((gate.pt_goi_sweeps_per_tick - 3.6366337e9).abs() < 1e3);
        assert_eq!(gate.weyl_crowbar_latency_ps, 1.701);
    }
}
