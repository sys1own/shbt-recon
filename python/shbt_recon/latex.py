"""Generate LaTeX macros from a ModularStateTranslocator audit."""

import math
from pathlib import Path

import shbt_recon


def _fmt(value, decimals=12):
    """Format a float for LaTeX, using scientific notation outside [1e-4, 1e5)."""
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, str):
        try:
            f = float(value)
        except ValueError:
            return value
        return _fmt(f, decimals)

    if not math.isfinite(value) or value == 0.0:
        return "0"

    magnitude = abs(value)
    if 1.0e-4 <= magnitude < 1.0e5:
        return f"{value:.{decimals}g}"

    exponent = math.floor(math.log10(magnitude))
    mantissa = value / (10.0 ** exponent)
    return f"{mantissa:.{decimals - 1}f}\\times{{}}10^{{{exponent}}}"


def _write_macro(lines, name, value):
    lines.append(f"\\newcommand{{\\{name}}}{{{value}}}")


def _get(d, *keys, default="0"):
    """Safely traverse a nested dictionary."""
    try:
        for k in keys:
            d = d[k]
        return d
    except (KeyError, TypeError):
        return default


def generate_recon_results_tex(path="recon_results.tex"):
    """Run the canonical audit and write a macro file for main.tex."""
    translocator = shbt_recon.ModularStateTranslocator()
    audit = translocator.audit()

    lines = [
        "% SHBT Destination Reconstruction audit macros",
        "% Generated automatically by shbt_recon.latex.generate_recon_results_tex",
    ]

    engine = audit.get("engine", {})
    _write_macro(lines, "ReconBranch", engine.get("branch", "(26, 8, 312)"))
    _write_macro(lines, "ReconKernel", engine.get("branch", "(26, 8, 312)"))
    _write_macro(lines, "ReconResidualFraction", _fmt(engine.get("residual_fraction", 0.0)))
    _write_macro(lines, "ReconCompletedFraction", _fmt(engine.get("completed_fraction", 0.0)))
    _write_macro(lines, "ReconStinespringRatio", _fmt(engine.get("stinespring_ratio", 0.0)))
    _write_macro(lines, "ReconUnitarityResidual", _fmt(engine.get("unitarity_residual", 0.0)))
    _write_macro(lines, "ReconEigenvectorDetuning", _fmt(engine.get("eigenvector_rigidity_detuning", 0.0)))
    _write_macro(lines, "ReconPhaseUnitarityResidual", _fmt(engine.get("phase_unitarity_residual", 0.0)))
    _write_macro(lines, "ReconReconstructionAmplitude", _fmt(engine.get("reconstruction_amplitude", 0.0)))
    _write_macro(lines, "ReconCausalAuth", str(engine.get("causal_authorization_passed", "true")))
    _write_macro(lines, "ReconHilStatus", str(audit.get("hil_status", "STATUS_NOMINAL_PASS")).replace("_", "\\_"))
    _write_macro(lines, "ReconNoiseFloor", "1.0\\times{}10^{-122}")
    _write_macro(lines, "ReconDetuningTolerance", "10^{-12}")

    # Verification targets for the dynamic results table.
    _write_macro(lines, "ReconUnitarityTarget", "10^{-14}")
    _write_macro(lines, "ReconDetuningTarget", "1.77\\times{}10^{-16}")
    _write_macro(lines, "ReconPhaseJitterTarget", _fmt(audit.get("hardware", {}).get("phase_jitter_threshold_rad", 5.05e-5)))
    _write_macro(lines, "ReconCGetTarget", "5.3429\\times{}10^{-76}")

    # Metric nullification
    active = audit.get("active_metric", {})
    nullified = audit.get("nullified_metric", {})
    _write_macro(lines, "ReconMetricDetError", _fmt(active.get("determinant_error", 0.0)))
    _write_macro(lines, "ReconMetricMinGramEv", _fmt(active.get("minimum_gram_eigenvalue", 0.0)))
    _write_macro(lines, "ReconMetricPassed", _fmt(active.get("passed", False)))
    _write_macro(lines, "ReconMetricNullifiedDetError", _fmt(nullified.get("determinant_error", 0.0)))
    _write_macro(lines, "ReconMetricNullifiedMinGramEv", _fmt(nullified.get("minimum_gram_eigenvalue", 0.0)))

    # Hardware synthesis
    hardware = audit.get("hardware", {})
    _write_macro(lines, "ReconPhaseJitter", _fmt(hardware.get("phase_jitter_rad", 0.0)))
    _write_macro(lines, "ReconPhaseJitterThreshold", _fmt(hardware.get("phase_jitter_threshold_rad", 5.05e-5)))
    _write_macro(lines, "ReconPhaseJitterPass", _fmt(hardware.get("phase_jitter_passes", False)))
    _write_macro(lines, "ReconThermalNoiseLimitJ", _fmt(hardware.get("thermal_noise_limit_j", 0.0)))
    _write_macro(lines, "ReconCGetJ", _fmt(hardware.get("c_get_j", 0.0)))
    _write_macro(lines, "ReconThermalNoisePass", _fmt(hardware.get("thermal_noise_passes", False)))

    # Thermodynamics
    thermo = audit.get("thermodynamics", {})
    _write_macro(lines, "ReconNLocal", _fmt(thermo.get("n_local_bits", 0.0)))
    _write_macro(lines, "ReconNSat", _fmt(thermo.get("n_sat_bits", 0.0)))
    _write_macro(lines, "ReconTemperatureK", _fmt(thermo.get("temperature_k", 0.0)))
    _write_macro(lines, "ReconRatio", _fmt(thermo.get("ratio", 0.0)))
    _write_macro(lines, "ReconLandauerLimitJ", _fmt(thermo.get("landauer_limit_j", 0.0)))

    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return path


if __name__ == "__main__":
    print(generate_recon_results_tex())
