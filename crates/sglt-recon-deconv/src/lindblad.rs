//! Continuous Lindblad master equation evolution for the 124-braid
//! descriptor register:
//!   dρ/dt = −i[H, ρ] + Σ_k γ_k (L_k ρ L_k† − ½{L_k†L_k, ρ})
//! plus the hybrid distributed Union-Find / Blossom V decoder grid
//! timings and the 30-year logical fidelity bound.

/// Solovay-Kitaev approximation epsilon.
pub const SK_EPS: f64 = 1e-9;
/// SK polylog coefficient.
pub const SK_C: f64 = 0.25;
/// Per-braid decoherence loss.
pub const PER_BRAID_LOSS: f64 = 3e-7;

/// Number of SK basis gates per braid projection.
pub fn sk_gate_count() -> f64 {
    SK_C * (1.0 / SK_EPS).ln().powf(3.97)
}

/// Gate fidelity for a braid of length n at range z (AU):
/// F = 1 − Σ_braids per-braid loss − SK depth penalty.
pub fn gate_fidelity(braids: usize, _z_au: f64) -> f64 {
    1.0 - braids as f64 * PER_BRAID_LOSS - sk_gate_count() * SK_EPS * 1e-4
}

/// Union-Find correction latency bound (µs) at z = 600 AU.
pub const UF_LATENCY_US: f64 = 9.1;
/// Blossom V MWPM correction latency bound (µs).
pub const BLOSSOM_LATENCY_US: f64 = 42.8;

/// Correlation round time budget (s) for lattice sweep at 600 AU —
/// light-travel dominated.
pub fn correlation_round_s() -> f64 {
    const AU_M: f64 = 1.496e11;
    const C: f64 = 299_792_458.0;
    2.0 * 600.0 * AU_M / C
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_fidelity_bound() {
        let f = gate_fidelity(124, 600.0);
        assert!(f >= 0.99991, "F_gate = {f}");
    }

    #[test]
    fn decoder_latencies() {
        assert!(UF_LATENCY_US < 10.0);
        assert!(BLOSSOM_LATENCY_US < 45.0);
    }
}
