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

// ---------------------------------------------------------------------------
// Thermodynamic and hardware-synthesis constants
// ---------------------------------------------------------------------------

/// Boltzmann constant in J/K.
pub const KB_J_PER_K: f64 = 1.380_649e-23;

/// Natural logarithm of 2.
pub const LN2: f64 = 0.693_147_180_559_945_3;

/// Canonical operating temperature for the hardware-synthesis audit (15.4 mK).
pub const TEMPERATURE_K: f64 = 15.4e-3;

/// Local boundary register size used for thermodynamic costing (bits).
pub const N_LOCAL_BITS: f64 = 1.20e72;

/// Saturated holographic horizon register size (bits).
pub const N_SAT_BITS: f64 = 3.31e122;

/// Acceptable phase-jitter limit for the hardware-synthesis auditor (rad).
pub const PHASE_JITTER_THRESHOLD_RAD: f64 = 5.05e-5;

// ---------------------------------------------------------------------------
// Metric nullification grid constants
// ---------------------------------------------------------------------------

/// Maximum number of points in the 1-D metric-nullification audit grid.
pub const MAX_METRIC_GRID: usize = 129;

/// Default metric audit domain radius in metres.
pub const METRIC_DOMAIN_RADIUS_M: f64 = 10.0;

/// Default metric bubble radius in metres.
pub const METRIC_BUBBLE_RADIUS_M: f64 = 2.0;

/// Default metric wall steepness parameter (1/m).
pub const METRIC_WALL_STEEPNESS_PER_M: f64 = 2.0;
