import math
import pytest
import shbt_recon


def unit_residual():
    return [1.0 / math.sqrt(8.0)] * 8


def test_kernel_locked():
    """Only the benchmark (26, 8, 312) kernel is accepted."""
    with pytest.raises(shbt_recon.AnomalyClosureError):
        shbt_recon.DerenderingEngine.with_kernel((1, 1, 1))


def test_exact_dark_fractions():
    """Dark ledger fractions are exact 10/33 and 23/33."""
    ledger = shbt_recon.DarkLedger()
    tol = 1.0e-15
    assert abs(ledger.residual_fraction() - 10.0 / 33.0) < tol
    assert abs(ledger.completed_fraction() - 23.0 / 33.0) < tol
    assert abs(ledger.stinespring_amplitude() - 23.0 / 33.0) < tol


def test_stinespring_map_and_reconstruct():
    """De-render and re-render a unit residual state."""
    engine = shbt_recon.DerenderingEngine()
    residual = unit_residual()
    engine.execute_stinespring_map(0, residual)

    src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(1.0, 0.0, 0.0, 0.0)
    theta = 0.421

    result = engine.reconstruct(src, tar, theta, 0, 1)
    assert len(result) == 8

    # The Stinespring amplitude scales the state; after phase rotation the
    # squared magnitude is (23/33)^2 for a unit-norm residual.
    amp2 = sum(re**2 + im**2 for re, im in result)
    assert abs(amp2 - (23.0 / 33.0) ** 2) < 1.0e-12


def test_causal_violation():
    """A target outside the future light cone raises CausalViolationError."""
    engine = shbt_recon.DerenderingEngine()
    residual = unit_residual()
    engine.execute_stinespring_map(0, residual)

    src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(-1.0, 0.0, 0.0, 0.0)
    with pytest.raises((shbt_recon.CausalViolationError, ValueError)):
        engine.reconstruct(src, tar, 0.0, 0, 1)


def test_spacelike_violation():
    """A spacelike separated target is rejected."""
    engine = shbt_recon.DerenderingEngine()
    residual = unit_residual()
    engine.execute_stinespring_map(0, residual)

    src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(0.0, 2.0, 0.0, 0.0)
    with pytest.raises((shbt_recon.CausalViolationError, ValueError)):
        engine.reconstruct(src, tar, 0.0, 0, 1)


def test_anomaly_closure_detuning():
    """A detuned residual state triggers AnomalyClosureError."""
    engine = shbt_recon.DerenderingEngine()
    bad = [0.5] * 8  # norm is sqrt(2), not 1
    with pytest.raises(shbt_recon.AnomalyClosureError):
        engine.execute_stinespring_map(0, bad)


def test_hil_safety_monitor():
    """HIL monitor triggers anomaly closure on excessive eigenvector detuning."""
    monitor = shbt_recon.HilSafetyMonitor()
    assert (
        monitor.audit_hil_step(0.5, 1.0e-13, 1.0e-13, 1.0e60, 1.0e70)
        == "STATUS_NOMINAL_PASS"
    )
    assert (
        monitor.audit_hil_step(0.5, 1.0e-13, 1.0e-11, 1.0e60, 1.0e70)
        == "EMERGENCY_ANOMALY_CLOSURE"
    )


def test_run_pipeline_helper():
    """The one-shot helper produces the same result as the engine API."""
    residual = unit_residual()
    src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(1.0, 0.0, 0.0, 0.0)
    result = shbt_recon.run_reconstruction(residual, src, tar, 0.0)
    assert len(result) == 8
    amp2 = sum(re**2 + im**2 for re, im in result)
    assert abs(amp2 - (23.0 / 33.0) ** 2) < 1.0e-12
