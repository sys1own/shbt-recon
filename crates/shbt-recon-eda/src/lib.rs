//! shbt-recon-eda — automated CAD/EDA exporters and substrate modeling.
//!
//! - `gdsii`:     `GdsiiMaskExporter` 8×8 InP/InGaAs SHBT array (shbt-exotic)
//! - `step`:      `StepSolidModelExporter` ISO 10303-21 B-Rep sapphire
//!                waveguide (shbt-exotic)
//! - `substrate`: `DiamondGaNThermalModel` CVD Diamond-on-GaN + NbN/MgB₂
//!                routing (shbt-sglt)

pub mod gdsii;
pub mod step;
pub mod substrate;
pub mod s2p;
