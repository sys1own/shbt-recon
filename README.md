# SHBT Destination Reconstruction & State Decoupling Simulator

`shbt-recon` is the Phase 1 core engine for Static Holographic Boundary Theory (SHBT) destination reconstruction and artificial de-rendering / re-rendering. It implements the isometric Stinespring map, exact dark-ledger capacity partitioning, causal authorization, and phase-locked reconstruction operator from the SHBT Phase 1 technical specification.

## Features

- **Stinespring Dilation Map** (`DerenderingEngine`): maps visible boundary character blocks into the dark ledger while preserving passive stress-energy.
- **Exact Dark-Ledger Fractions** (`DarkLedger`): 512-bit rational tracking of residual `(10/33)` and completed `(23/33)` capacity partitions using the `rug` crate.
- **Causal Authorization** (`CausalCoordinate`): verifies `x_tar ∈ J^+(x_src)` before any re-rendering attempt.
- **Reconstruction Operator** (`ReconstructionOperator`): boundary relabeling plus phase-locked excitation `O^excitation(θ) = exp(-i θ Q_topological)`.
- **HIL Safety Monitor** (`HilSafetyMonitor`): triggers `EMERGENCY_ANOMALY_CLOSURE` when eigenvector rigidity detuning exceeds `10^-12`.

## Build & Test

```bash
make
```

This creates a Python virtual environment, installs `maturin` and `pytest`, builds the Rust extension in release mode, and runs the Python test suite. To run only the Rust unit tests:

```bash
make cargo-test
```

## Quick Start

```python
import math
import shbt_recon

residual = [1.0 / math.sqrt(8.0)] * 8
src = shbt_recon.CausalCoordinate(0.0, 0.0, 0.0, 0.0)
tar = shbt_recon.CausalCoordinate(1.0, 0.0, 0.0, 0.0)
result = shbt_recon.run_reconstruction(residual, src, tar, 0.421)
print(result)
```

Or via the CLI:

```bash
shbt-recon --tar-t 1.0 --theta 0.421
```
