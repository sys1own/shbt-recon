//! Rigid constants for the SHBT (26, 8, 312) reconstruction kernel.

/// Bit precision used by the high-precision state vectors.
pub const PREC: u32 = 512;

/// Number of visible character blocks in the boundary register.
pub const VISIBLE_STATE_DIM: usize = 16;

/// Dark-ledger subspace dimension attached to each visible block.
pub const DARK_LEDGER_DIM: usize = 8;

/// Canonical anomaly-free boundary kernel.
pub const BENCHMARK_KERNEL: (u32, u32, u32) = (26, 8, 312);

/// Exact residual dark capacity fraction: 10/33.
pub const RESIDUAL_FRAC: (u32, u32) = (10, 33);

/// Exact completed dark capacity fraction: 23/33.
pub const COMPLETED_FRAC: (u32, u32) = (23, 33);

/// Critical eigenvector-rigidity detuning tolerance.
pub const EIGENVECTOR_DETUNING_TOLERANCE: f64 = 1.0e-12;

/// Holographic noise floor for amplitude validation.
pub const HOLOGRAPHIC_NOISE_FLOOR: f64 = 1.0e-122;
