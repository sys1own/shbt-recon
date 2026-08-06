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


def test_causal_violation_raises_anomaly_closure():
    """A target outside the future light cone raises AnomalyClosureError."""
    engine = shbt_recon.DerenderingEngine()
    residual = unit_residual()
    engine.execute_stinespring_map(0, residual)

    src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(-1.0, 0.0, 0.0, 0.0)
    with pytest.raises(shbt_recon.AnomalyClosureError):
        engine.reconstruct(src, tar, 0.0, 0, 1)


def test_spacelike_violation_raises_anomaly_closure():
    """A spacelike separated target is rejected."""
    engine = shbt_recon.DerenderingEngine()
    residual = unit_residual()
    engine.execute_stinespring_map(0, residual)

    src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(0.0, 2.0, 0.0, 0.0)
    with pytest.raises(shbt_recon.AnomalyClosureError):
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


def test_modular_state_translocator_translocate():
    """The production translocator runs the full pipeline end-to-end."""
    trans = shbt_recon.ModularStateTranslocator()
    src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(1.0, 0.0, 0.0, 0.0)
    result = trans.translocate(unit_residual(), src, tar, 0.421, 0, 1, 1.071186)
    assert len(result) == 8
    amp2 = sum(re**2 + im**2 for re, im in result)
    assert abs(amp2 - (23.0 / 33.0) ** 2) < 1.0e-12


def test_modular_state_translocator_rejects_spacelike():
    """The translocator raises AnomalyClosureError for spacelike targets."""
    trans = shbt_recon.ModularStateTranslocator()
    src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(0.0, 2.0, 0.0, 0.0)
    with pytest.raises(shbt_recon.AnomalyClosureError):
        trans.translocate(unit_residual(), src, tar, 0.0, 0, 1, 0.0)


def test_modular_state_translocator_audit_sections():
    """The translocator audit contains all production sections."""
    trans = shbt_recon.ModularStateTranslocator()
    audit = trans.audit()
    assert "engine" in audit
    assert "active_metric" in audit
    assert "nullified_metric" in audit
    assert "hardware" in audit
    assert "thermodynamics" in audit
    assert audit["active_metric"]["passed"]
    assert audit["hardware"]["phase_jitter_passes"]
    assert audit["hardware"]["thermal_noise_passes"]


def test_metric_nullification_auditor():
    """The metric nullification auditor confirms det = -1.0."""
    auditor = shbt_recon.MetricNullificationAuditor()
    result = auditor.audit(1.071186)
    assert result["passed"]
    assert abs(result["determinant_error"]) < 1.0e-12
    assert result["minimum_abs_determinant"] > 0.99


def test_hardware_synthesis_auditor():
    """The hardware-synthesis auditor stays within jitter and thermal limits."""
    hsa = shbt_recon.HardwareSynthesisAuditor()
    assert hsa.phase_jitter_passes()
    assert hsa.thermal_noise_passes()
    assert hsa.c_get_j < hsa.thermal_noise_limit_j


def test_thermodynamic_cost():
    """The thermodynamic cost uses the benchmark N_local, N_sat and temperature."""
    tc = shbt_recon.ThermodynamicCost()
    assert tc.n_local_bits == pytest.approx(1.20e72, rel=1.0e-15)
    assert tc.n_sat_bits == pytest.approx(3.31e122, rel=1.0e-15)
    assert tc.temperature_k == pytest.approx(15.4e-3, rel=1.0e-15)
    assert tc.c_get_j > 0.0
    assert tc.c_get_j < tc.landauer_limit_j
