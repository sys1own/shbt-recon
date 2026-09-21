//! sglt-recon-deconv: hierarchical non-Abelian Fibonacci fusion-tree
//! compression (τ ⊗ τ = 1 ⊕ τ, d_τ = φ, D = √(2 + φ)), 1,472-byte SRAM
//! dark-ledger TQEC frame parsing, 124 Fibonacci anyon braid descriptor
//! tracking, continuous Lindblad evolution, and the hybrid distributed
//! Union-Find / Blossom V decoder grid.

pub mod decoder;
pub mod frame;
pub mod fusion;
pub mod lindblad;

/// Golden ratio — quantum dimension of the Fibonacci anyon.
pub const PHI: f64 = 1.618_033_988_749_894_9;
/// Total quantum dimension D = √(2 + φ).
pub const TOTAL_DIM: f64 = 1.902_113_032_590_307_1;
/// Dark-ledger capacity partition η_D = 23/33.
pub const ETA_D_NUM: u64 = 23;
pub const CAPACITY_DENOM: u64 = 33;
/// Number of braid descriptors in the frame.
pub const BRAID_COUNT: usize = 124;
/// Dark ledger frame size (bytes).
pub const FRAME_BYTES: usize = 1472;
/// Braid descriptor payload (124 × 8 B = 992 B).
pub const BRAID_PAYLOAD_BYTES: usize = 992;
/// SECDED/checkpoint metadata (bytes).
pub const METADATA_BYTES: usize = FRAME_BYTES - BRAID_PAYLOAD_BYTES;
/// Surface code distance.
pub const CODE_DISTANCE: u32 = 17;
/// Physical error rate.
pub const P_PHYS: f64 = 1e-4;
/// Fault-tolerance threshold.
pub const P_TH: f64 = 1e-2;
/// TQEC scaling prefactor.
pub const PL_PREFACTOR: f64 = 0.031;
/// Logical error rate P_L = α (p/p_th)^((d+1)/2).
pub fn logical_error_rate() -> f64 {
    PL_PREFACTOR * (P_PHYS / P_TH).powi(((CODE_DISTANCE + 1) / 2) as i32)
}
/// 30-year service interval (s).
pub const SERVICE_S: f64 = 9.46e8;
/// Effective decoder correction cycle rate (Hz) over the service window.
pub const DECODE_CYCLE_HZ: f64 = 10.0;
/// Logical fidelity F_logical = 1 − P_L · f_cycle · t.
pub fn logical_fidelity() -> f64 {
    1.0 - logical_error_rate() * DECODE_CYCLE_HZ * SERVICE_S
}
