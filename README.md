# SHBT Destination Reconstruction & State Decoupling Simulator

`shbt-recon` is the production-grade core engine for Static Holographic Boundary Theory (SHBT) destination reconstruction and artificial de-rendering / re-rendering. It implements the isometric Stinespring map, exact dark-ledger capacity partitioning, causal authorization, and phase-locked reconstruction operator from the SHBT technical specification, now wrapped in the high-precision **Modular State Translocator**.

## Features

- **Stinespring Dilation Map** (`DerenderingEngine`): maps visible boundary character blocks into the dark ledger while preserving passive stress-energy.
- **Exact Dark-Ledger Fractions** (`DarkLedger`): 512-bit rational tracking of residual `(10/33)` and completed `(23/33)` capacity partitions using the `rug` crate.
- **Causal Authorization** (`CausalCoordinate`): verifies `x_tar ∈ J^+(x_src)` before any re-rendering attempt; spacelike targets raise a fatal `AnomalyClosureError`.
- **Reconstruction Operator** (`ReconstructionOperator`): boundary relabeling plus phase-locked excitation `O^excitation(θ) = exp(-i θ Q_topological)`.
- **HIL Safety Monitor** (`HilSafetyMonitor`): triggers `EMERGENCY_ANOMALY_CLOSURE` when eigenvector rigidity detuning exceeds `10^-12`.
- **Metric Nullification Auditor** (`MetricNullificationAuditor`): verifies the Lorentzian determinant stays exactly `-1.0` while the active metric slice is nullified.
- **Hardware Synthesis Auditor** (`HardwareSynthesisAuditor`): audits phase jitter (`≤ 5.05×10^-5 rad`) and thermal-noise limits.
- **Thermodynamic Cost** (`ThermodynamicCost`): implements `C_get = k_B T ln 2 * (N_local / N_sat)` with `T = 15.4 mK`, `N_local ≈ 1.20×10^72`, and `N_sat ≈ 3.31×10^122`.
- **Modular State Translocator** (`ModularStateTranslocator`): production orchestrator that runs HIL, metric, hardware, and thermodynamic audits end-to-end.

## Build & Test

```bash
make
```

This creates a Python virtual environment, installs `maturin` and `pytest`, builds the Rust extension in release mode, runs the Python test suite, and builds `main.pdf`. To run only the Rust unit tests:

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

# Low-level engine API
engine = shbt_recon.DerenderingEngine()
engine.execute_stinespring_map(0, residual)
result = engine.reconstruct(src, tar, 0.421, 0, 1)
print(result)

# Production translocator API
trans = shbt_recon.ModularStateTranslocator()
result = trans.translocate(residual, src, tar, 0.421)
print(result)

# Full system audit
print(trans.audit())
```

Or via the CLI:

```bash
shbt-recon --tar-t 1.0 --theta 0.421
```
