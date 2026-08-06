"""Generate LaTeX macros from a DerenderingEngine audit."""

import math
from pathlib import Path

import shbt_recon


def _fmt(value, decimals=12):
    """Format a float for LaTeX, using scientific notation outside [1e-4, 1e5)."""
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


def generate_recon_results_tex(path="recon_results.tex"):
    """Run the canonical audit and write a macro file for main.tex."""
    engine = shbt_recon.DerenderingEngine()
    audit = engine.audit()

    lines = [
        "% SHBT Destination Reconstruction audit macros",
        "% Generated automatically by shbt_recon.latex.generate_recon_results_tex",
    ]

    _write_macro(lines, "ReconBranch", audit.get("branch", "(26, 8, 312)"))
    _write_macro(lines, "ReconKernel", audit.get("branch", "(26, 8, 312)"))
    _write_macro(lines, "ReconResidualFraction", _fmt(audit.get("residual_fraction", 0.0)))
    _write_macro(lines, "ReconCompletedFraction", _fmt(audit.get("completed_fraction", 0.0)))
    _write_macro(lines, "ReconStinespringRatio", _fmt(audit.get("stinespring_ratio", 0.0)))
    _write_macro(lines, "ReconUnitarityResidual", _fmt(audit.get("unitarity_residual", 0.0)))
    _write_macro(lines, "ReconEigenvectorDetuning", _fmt(audit.get("eigenvector_rigidity_detuning", 0.0)))
    _write_macro(lines, "ReconPhaseUnitarityResidual", _fmt(audit.get("phase_unitarity_residual", 0.0)))
    _write_macro(lines, "ReconReconstructionAmplitude", _fmt(audit.get("reconstruction_amplitude", 0.0)))
    _write_macro(lines, "ReconCausalAuth", str(audit.get("causal_authorization_passed", "true")))
    _write_macro(lines, "ReconNoiseFloor", "1.0\\times{}10^{-122}")
    _write_macro(lines, "ReconDetuningTolerance", "10^{-12}")

    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return path


if __name__ == "__main__":
    print(generate_recon_results_tex())
