//! sglt-translocator-core: macroscopic Stinespring dilation, invariant
//! capacity partitioning, multi-node swarm boundary relabeling (M > 2),
//! minimum-jerk trajectory integration, and SPSC shared-memory telemetry.

pub mod dilation;
pub mod minjerk;
pub mod swarm;

pub use shbt_recon_core::telemetry;
