"""Generate vector PDF figures for the SHBT reconstruction manuscript."""

import math
from pathlib import Path

import matplotlib
matplotlib.use("Agg")  # noqa: E402
import matplotlib.pyplot as plt
import numpy as np

import shbt_recon


def figure_path(name):
    path = Path("figures") / name
    path.parent.mkdir(parents=True, exist_ok=True)
    return path


def plot_stinespring_state():
    """Bar plot of the dark-ledger amplitudes after the Stinespring map."""
    engine = shbt_recon.DerenderingEngine()
    residual = [1.0 / math.sqrt(8.0)] * 8
    engine.execute_stinespring_map(0, residual)
    state = engine.get_state_vector()
    amplitudes = [abs(complex(re, im)) for re, im in state[0]]

    fig, ax = plt.subplots(figsize=(6, 3.5))
    ax.bar(range(8), amplitudes, color="steelblue", edgecolor="black")
    ax.set_xlabel("Dark ledger component $i$")
    ax.set_ylabel("Amplitude $|\\chi^{\\text{res}}_i|$")
    ax.set_title("Stinespring dark-state amplitude at $(26,8,312)$")
    ax.set_xticks(range(8))
    fig.tight_layout()
    path = figure_path("stinespring_state.pdf")
    fig.savefig(path, bbox_inches="tight")
    plt.close(fig)
    return path


def plot_causal_cone():
    """2D light-cone diagram for future causal authorization."""
    fig, ax = plt.subplots(figsize=(5, 5))
    t = np.linspace(0, 1.5, 200)
    ax.fill_betweenx(t, -t, t, color="lightyellow", alpha=0.7, label="$J^+(x_\\mathrm{src})$")
    ax.plot(t, t, "k--", lw=1)
    ax.plot(t, -t, "k--", lw=1)
    ax.plot(0, 0, "ko", label="$x_\\mathrm{src}$")
    ax.plot(1.0, 0.0, "rs", label="$x_\\mathrm{tar}$")
    ax.annotate("future cone", xy=(0.6, 0.4), fontsize=10)
    ax.set_xlabel("$x$ (spatial)")
    ax.set_ylabel("$t$ (temporal)")
    ax.set_xlim(-0.2, 1.6)
    ax.set_ylim(-0.2, 1.6)
    ax.set_aspect("equal")
    ax.legend(loc="upper right")
    fig.tight_layout()
    path = figure_path("causal_cone.pdf")
    fig.savefig(path, bbox_inches="tight")
    plt.close(fig)
    return path


def plot_phase_rotation():
    """Argand diagram of the phase-locked excitation operator."""
    theta = 0.421
    re = math.cos(theta)
    im = -math.sin(theta)

    fig, ax = plt.subplots(figsize=(4.5, 4.5))
    circle = plt.Circle((0, 0), 1, fill=False, color="gray", ls="--")
    ax.add_patch(circle)
    ax.annotate("", xy=(re, im), xytext=(0, 0),
                arrowprops=dict(arrowstyle="->", color="darkred", lw=2))
    ax.plot([re], [im], "o", color="darkred")
    ax.text(re + 0.08, im + 0.08, f"$e^{{-i{theta}}}$", fontsize=11)
    ax.set_xlim(-1.3, 1.3)
    ax.set_ylim(-1.3, 1.3)
    ax.set_aspect("equal")
    ax.axhline(0, color="black", lw=0.5)
    ax.axvline(0, color="black", lw=0.5)
    ax.set_xlabel("$\\mathrm{Re}$")
    ax.set_ylabel("$\\mathrm{Im}$")
    ax.set_title("Phase-locked excitation $O^{\\text{excitation}}(\\theta)$")
    fig.tight_layout()
    path = figure_path("phase_rotation.pdf")
    fig.savefig(path, bbox_inches="tight")
    plt.close(fig)
    return path


def generate_all():
    """Regenerate all manuscript figures."""
    return [
        plot_stinespring_state(),
        plot_causal_cone(),
        plot_phase_rotation(),
    ]


if __name__ == "__main__":
    for p in generate_all():
        print(p)
