//! sglt-translocator-core: macroscopic Stinespring dilation, invariant
//! capacity partitioning, multi-node swarm boundary relabeling (M > 2),
//! minimum-jerk trajectory integration, and SPSC shared-memory telemetry.

pub mod braid;
pub mod crystallization_kinetics;
pub mod derender;
pub mod dilation;
pub mod fp_closure;
pub mod intergalactic;
pub mod minjerk;
pub mod stepping;
pub mod swarm;
pub mod synthesis;
pub mod wavefront;

pub use braid::*;
pub use crystallization_kinetics::*;
pub use derender::*;
pub use fp_closure::*;
pub use intergalactic::*;
pub use sglt_lanr_power::compacted_nacelle::*;
pub use shbt_recon_core::telemetry;
pub use wavefront::*;
