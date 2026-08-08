"""Minimal CLI for the SHBT reconstruction simulator."""

import argparse
import math
import sys

import shbt_recon


def main():
    parser = argparse.ArgumentParser(description="SHBT Destination Reconstruction Simulator")
    parser.add_argument("--residual", nargs=8, type=float, default=None,
                        help="8 dark-ledger residual amplitudes")
    parser.add_argument("--src-t", type=float, default=0.0)
    parser.add_argument("--src-x", type=float, default=0.0)
    parser.add_argument("--tar-t", type=float, default=1.0)
    parser.add_argument("--tar-x", type=float, default=0.0)
    parser.add_argument("--theta", type=float, default=0.421)
    parser.add_argument("--source-index", type=int, default=0)
    parser.add_argument("--target-index", type=int, default=1)
    parser.add_argument("--active-velocity", type=float, default=1.071186,
                        help="Active metric shift velocity in units of c")
    parser.add_argument(
        "--edge-noise-variance",
        type=float,
        default=0.0,
        help="Topological edge-state phase-noise variance (rad^2) for HIL robustness test",
    )
    args = parser.parse_args()

    if args.residual is None:
        residual = [1.0 / math.sqrt(8.0)] * 8
    else:
        residual = args.residual
        norm = math.sqrt(sum(v * v for v in residual))
        residual = [v / norm for v in residual]

    src = shbt_recon.CausalCoordinate(args.src_t, args.src_x, 0.0, 0.0)
    tar = shbt_recon.CausalCoordinate(args.tar_t, args.tar_x, 0.0, 0.0)

    trans = shbt_recon.ModularStateTranslocator()
    trans.set_edge_noise_variance(args.edge_noise_variance)

    try:
        result = trans.translocate(
            residual,
            src,
            tar,
            args.theta,
            args.source_index,
            args.target_index,
            args.active_velocity,
        )
    except shbt_recon.AnomalyClosureError as exc:
        print(f"HIL safety abort: {exc}", file=sys.stderr)
        sys.exit(1)

    print("Reconstructed visible amplitude (re, im):")
    for i, (re, im) in enumerate(result):
        print(f"  [{i}] {re:+.12e} {im:+.12e}")

    # Emit the full engineering audit as well.
    audit = trans.audit()
    print("\nHIL status:", audit["hil_status"])
    print("Thermal status:", audit["thermal_status"])
    print("Effective phase jitter (rad):", audit["effective_phase_jitter_rad"])


if __name__ == "__main__":
    main()
