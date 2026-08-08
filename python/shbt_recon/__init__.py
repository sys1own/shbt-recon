"""SHBT Destination Reconstruction & State Decoupling Simulator — Python package."""

from shbt_recon._core import (
    AnomalyClosureError,
    BoundaryRelabeling,
    CausalCoordinate,
    CausalViolationError,
    DarkLedger,
    DarkLedgerTraceLoss,
    DerenderingEngine,
    HardwareSynthesisAuditor,
    HilSafetyMonitor,
    MetricNullificationAuditor,
    ModularStateTranslocator,
    PhaseLockedExcitation,
    ReconstructionOperator,
    ThermalDissipationAuditor,
    ThermodynamicCost,
    TopologicalEdgeNoise,
    TopologicalProtectionAuditor,
)

__version__ = "0.1.0"
__all__ = [
    "AnomalyClosureError",
    "BoundaryRelabeling",
    "CausalCoordinate",
    "CausalViolationError",
    "DarkLedger",
    "DarkLedgerTraceLoss",
    "DerenderingEngine",
    "HardwareSynthesisAuditor",
    "HilSafetyMonitor",
    "MetricNullificationAuditor",
    "ModularStateTranslocator",
    "PhaseLockedExcitation",
    "ReconstructionOperator",
    "ThermalDissipationAuditor",
    "ThermodynamicCost",
    "TopologicalEdgeNoise",
    "TopologicalProtectionAuditor",
    "run_reconstruction",
]


def run_reconstruction(residual_state, src, tar, theta, source_index=0, target_index=1):
    """One-shot de-render then re-render pipeline.

    Parameters
    ----------
    residual_state : list[float]
        8-component residual dark state, normalized to unit norm.
    src, tar : CausalCoordinate
        Source and target boundary coordinates.
    theta : float
        Phase-locking angle.
    source_index, target_index : int
        Visible character-block indices.

    Returns
    -------
    list[tuple[float, float]]
        8-component reconstructed visible complex amplitude.
    """
    engine = DerenderingEngine()
    engine.execute_stinespring_map(source_index, residual_state)
    return engine.reconstruct(src, tar, theta, source_index, target_index)
