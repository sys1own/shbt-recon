//! shbt-recon-core — metric control, translocation, kinematics, causal
//! history projection, Stinespring dilation, and zero-copy telemetry.
//!
//! Modules transferred per `rec1.txt`:
//! - `adm`:        ADMMetricAuditor / ADM3Plus1ShiftField (sys1own/shbt-exotic)
//! - `translocate`: ModularStateTranslocator / GramPositivityVerifier (exotic)
//! - `kinematics`: WakeTensorComp μ_comp(t) (sys1own/shbt-exotic)
//! - `causal`:     CausalPointMemory / Π_{A,ι} rank-one projector (precision)
//! - `telemetry`:  SPSC POSIX shm ring, `#[repr(C, align(64))]` frames (shbt-cf)
//! - `stinespring`: 10/33 active · 23/33 dark exact-capacity dilation.

pub mod adm;
pub mod causal;
pub mod kinematics;
pub mod stinespring;
pub mod telemetry;
pub mod translocate;
