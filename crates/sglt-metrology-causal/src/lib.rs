//! sglt-metrology-causal: 2PN relativistic spacetime metric evaluation,
//! dual-wavelength heterodyne laser metrology, Yb optical-lattice clock
//! reference, and hardware-enforced causal lightcone interlock.

pub mod pn2;
pub mod tmsv_sqz;

pub use shbt_recon_metrology::{lightcone, tmsv};

/// Ytterbium optical lattice clock fractional jitter bound (s).
pub const YB_CLOCK_SIGMA_T: f64 = 1.0e-18;
/// Emergency GaN shunt quench latency bound (ns).
pub const TAU_QUENCH_NS: f64 = 2.50;
