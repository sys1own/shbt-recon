//! shbt-recon-thermo — Kapitza cryo boundary resistance, LANR power plant
//! ledger, and Landauer GET accounting.
//!
//! - `cryo`:     `CryoCoolingModel` / `KapitzaInterfaceLayer` (shbt-exotic)
//! - `lanr`:     `LANRPowerLedger` 1,800-module plant (shbt-cf)
//! - `landauer`: `LandauerGetCalculator` GET/erasure costs (shbt-precision)

pub mod cryo;
pub mod landauer;
pub mod lanr;
