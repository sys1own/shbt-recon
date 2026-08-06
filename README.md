# SHBT Destination Reconstruction \& State Decoupling Simulator

`shbt-recon` is the production-grade reference implementation for Static Holographic Boundary Theory (SHBT) destination reconstruction and artificial de-rendering / re-rendering. It implements the isometric Stinespring map, exact dark-ledger capacity partitioning, future causal-cone authorization, phase-locked boundary relabeling, and a real-time Hardware-in-the-Loop (HIL) safety monitor.

## Theoretical Foundation

The simulator is built around the canonical anomaly-free boundary branch $(k_l, k_q, K) = (26, 8, 312)$. In SHBT, a bulk destination is not an independent spacetime point but a rendered image of boundary character excitations. The closure chain that protects the theory from unphysical geometries is

$$
\text{modular invariance} \;\Longleftrightarrow\; \Delta_{\mathrm{fr}} = 0 \;\Longleftrightarrow\; E_{\mu\nu} = 0,
$$

where $\Delta_{\mathrm{fr}}$ is the scalar framing defect of the boundary register and $E_{\mu\nu}$ is the holographic stress-energy residual.

### Stinespring de-rendering

A visible boundary character block $c$ with normalized local state

$$
|C_{\mathrm{loc}}(c)\rangle = \sum_{i=0}^{7} r_i |i\rangle,
\qquad \sum_i |r_i|^2 = 1,
$$

is de-rendered by the operator

$$
D^{\text{derender}} |C_{\mathrm{loc}}(c)\rangle
= |\mathrm{vac}\rangle_{\mathrm{vis}} \otimes \eta_D \sum_{i=0}^{7} r_i |i,0\rangle_{\mathrm{dark}},
$$

with Stinespring amplitude

$$
\eta_D = c_{\mathrm{dark}}^{\mathrm{comp}} = \frac{23}{33}.
$$

The residual dark capacity is the complementary exact rational fraction

$$
c_{\mathrm{dark}}^{\mathrm{res}} = \frac{10}{33}.
$$

The de-rendered state therefore carries the full information density of the completed sector while the active metric slice is nullified.

### Reconstruction operator

Re-rendering at a future-authorized target boundary address is performed by

$$
R^{\text{rerender}}
= T^{\partial}(x_{\mathrm{tar}}) \, D^{\text{derender}\,\dagger} \,
\bigl(I_{\mathrm{vis}} \otimes O^{\text{excitation}}(\theta)\bigr),
$$

where $T^{\partial}$ is the Heegaard--Floer boundary relabeling map and $O^{\text{excitation}}(\theta) = e^{-i\theta Q_{\text{topological}}}$ is the phase-locked $U(1)$ excitation operator. The topological charge is $Q_{\text{topological}} = 1$ for the canonical anyon lattice.

### Causal authorization

A re-rendering attempt is authorized only when the target lies inside or on the future causal cone of the source:

$$
x_{\mathrm{tar}} \in J^{+}(x_{\mathrm{src}})
\quad\Longleftrightarrow\quad
\Delta t > 0
\;\text{ and }\;
\Delta x^{2} + \Delta y^{2} + \Delta z^{2} \le (\Delta t)^{2},
$$

with $c = 1$. Spacelike or past targets raise a fatal `AnomalyClosureError`.

### Passive stress-energy preservation

De-rendering sends the active metric slice to zero, $g^{\mathrm{active}}_{\mu\nu} \to 0$, while total energy-momentum conservation gives

$$
\nabla_\mu T^{\mu\nu}_{\mathrm{total}} = 0.
$$

Taking the nullification limit yields

$$
\nabla_\mu T^{\mu\nu}_{\mathrm{passive}}
= -\lim_{g^{\mathrm{active}}\to 0} \nabla_\mu T^{\mu\nu}_{\mathrm{active}}
= 0,
$$

so the passive stress-energy stored in the boundary register is conserved.

## Hardware Implementation

The boundary excitations are driven by an InP/InGaAs single heterojunction bipolar transistor (SHBT) with a micro-airbridge structure and quasi-coplanar contacts. For a $1.5\times5\,\mu\mathrm{m}^{2}$ emitter the measured current-gain cutoff frequency is $f_T = 53$~GHz and the maximum oscillation frequency reaches

$$
f_{\max} = 72\ \mathrm{GHz}.
$$

This bandwidth supports gating of the $U(1)$ phase rotation at microwave-clock rates.

The dark-ledger excitations are routed through a 2D topological-insulator edge-state waveguide. The helical edge states are spin-momentum locked and robust against non-magnetic disorder and backscattering. The `TopologicalProtectionAuditor` simulates backscattering events and verifies that the spin polarization remains above the $0.99$ stability threshold even after $10^{9}$ scattering attempts at a backscattering rate of $10^{-12}$ per event.

A cryogenic HIL safety monitor runs continuously during the translocation cycle. If the eigenvector-rigidity detuning exceeds $10^{-12}$, the monitor asserts `EMERGENCY_ANOMALY_CLOSURE` and the translocation field is collapsed in less than $2.5$~ns. The monitor also checks the Lorentzian determinant residual against $10^{-12}$, the Gram eigenvalue lower bound, and the local information-density budget.

## Quick Start

### Build and test

```bash
make
```

This creates a Python virtual environment, installs `maturin` and `pytest`, builds the Rust extension in release mode, runs the Python test suite, regenerates the figures and `recon_results.tex`, and compiles `main.pdf`.

To run only the Rust unit tests:

```bash
make cargo-test
```

### Python API

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

### CLI

```bash
shbt-recon --tar-t 1.0 --theta 0.421
```

## Audit Results

A canonical run of `ModularStateTranslocator().audit()` produces the values injected into `main.tex` via `recon_results.tex`:

- Stinespring ratio: $\eta_D = 23/33 \approx 0.697$
- Unitarity residual: below $10^{-14}$
- Eigenvector-rigidity detuning: below $1.77\times10^{-16}$
- Phase-locked excitation residual: below $10^{-14}$
- Future causal-cone authorization: `true`
- HIL status: `STATUS_NOMINAL_PASS`
- Lorentzian determinant residual: $|\det(g)+1| < 10^{-12}$
- Phase jitter: below $5.05\times10^{-5}$~rad
- Thermodynamic GET cost: $C_{\text{get}} = k_B T \ln 2 \cdot (N_{\mathrm{local}} / N_{\mathrm{sat}})$, with $T = 15.4$~mK, $N_{\mathrm{local}} \approx 1.20\times10^{72}$, $N_{\mathrm{sat}} \approx 3.31\times10^{122}$.

All printed numbers in the manuscript are generated by running the simulator; the paper is therefore fully traceable to the executable code.

## Code Availability

- `shbt-recon`: https://github.com/sys1own/shbt-recon
- `shbt-precision`: https://github.com/sys1own/shbt-precision
- `shbt-warp`: https://github.com/sys1own/shbt-warp
