//! sglt-translocator-core: macroscopic Stinespring dilation, invariant
//! capacity partitioning, multi-node swarm boundary relabeling (M > 2),
//! minimum-jerk trajectory integration, and SPSC shared-memory telemetry.

#![forbid(unsafe_code)]

pub mod braid;
pub mod crystallization_kinetics;
pub mod derender;
pub mod dilation;
pub mod dynkin_router;
pub mod fp_closure;
pub mod intergalactic;
pub mod lattice_projector;
pub mod minjerk;
pub mod quine_cloner;
pub mod stepping;
pub mod swarm;
pub mod synthesis;
pub mod transmutation;
pub mod universal_foundry;
pub mod wavefront;

pub use braid::*;
pub use crystallization_kinetics::*;
pub use derender::*;
pub use dynkin_router::{
    BoundaryState346, DynamicFramingCompensator, DynkinRouter346, FramingCompensationReport,
    FramingFault, So10Weight, Su2Weight, Su3Weight, C_BRAID_BASE, C_PER_CHANNEL_BASE,
    DENOM_SO10, DENOM_SU2, DENOM_SU3, GOLDEN_RATIO_PHI, G_VEE_L, G_VEE_PARENT, G_VEE_Q,
    K_L, K_PARENT, K_Q, NUM_DYNKIN_SLOTS, NUM_FIBONACCI_CHANNELS, SO10_SLOT_END,
    SO10_SLOT_START, SU2_SLOT_END, SU2_SLOT_START, SU3_SLOT_END, SU3_SLOT_START,
};
pub use fp_closure::*;
pub use intergalactic::*;
pub use lattice_projector::*;
pub use quine_cloner::*;
pub use sglt_lanr_power::compacted_nacelle::*;
pub use shbt_recon_core::telemetry;
pub use transmutation::*;
pub use universal_foundry::*;
pub use wavefront::*;
