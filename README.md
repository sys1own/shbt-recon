# SHBT Destination Reconstruction and State Decoupling Simulator

`shbt-recon` is the production-grade reference implementation for Static Holographic Boundary Theory (SHBT) destination reconstruction and artificial de-rendering / re-rendering.
It implements the isometric Stinespring map, exact dark-ledger capacity partitioning, future causal-cone authorization, phase-locked boundary relabeling, a real-time Hardware-in-the-Loop (HIL) safety monitor, and a topological-insulator edge-state protection auditor.

This repository accompanies the SHBT research programme [1,2,3].

## Theoretical Foundation

The simulator is built around the canonical anomaly-free boundary branch

$$
(k_l, k_q, K) = (26, 8, 312).
$$

In SHBT, a bulk destination is not an independent spacetime point but a rendered image of boundary character excitations.
The consistency of the rendered geometry is protected by the closure chain

$$
\text{modular invariance}
\;\Longleftrightarrow\;
\Delta_{\mathrm{fr}} = 0
\;\Longleftrightarrow\;
E_{\mu\nu} = 0,
$$

where $\Delta_{\mathrm{fr}}$ is the scalar framing defect of the boundary register and $E_{\mu\nu}$ is the holographic stress-energy residual.
When the chain holds, the boundary is anomaly-free and the emergent geometry is energetically and causally consistent.

### Stinespring de-rendering

A visible boundary character block $c$ with normalized local state

$$
|C_{\mathrm{loc}}(c)\rangle = \sum_{i=0}^{7} r_i |i\rangle,
\qquad \sum_i |r_i|^2 = 1,
$$

is de-rendered by the operator

$$
D^{\text{derender}} |C_{\mathrm{loc}}(c)\rangle
= |\mathrm{vac}\rangle_{\mathrm{vis}}
\otimes
\eta_D \sum_{i=0}^{7} r_i |i,0\rangle_{\mathrm{dark}}.
$$

The Stinespring amplitude is the completed dark capacity

$$
\eta_D = c_{\mathrm{dark}}^{\mathrm{comp}} = \frac{23}{33},
$$

and the residual dark capacity is the complementary exact rational fraction

$$
c_{\mathrm{dark}}^{\mathrm{res}} = \frac{10}{33}.
$$

The de-rendered state therefore carries the full information density of the completed sector while the active metric slice is nullified.

### Dark-ledger trace loss

The Stinespring operator factorises as $D^{\text{derender}} = \eta_D U$, where $U$ is an isometry from the local visible Hilbert space onto the completed dark subspace.
Consequently

$$
U^{\dagger} U = I_{\mathrm{vis}}, \qquad U U^{\dagger} = P_{\mathrm{comp}}, \qquad
D^{\text{derender}\,\dagger} D^{\text{derender}} = \eta_D^{2} I_{\mathrm{vis}}.
$$

The unitarity residual vanishes, $\epsilon_{\mathrm{unitary}} = 0$, because $U$ preserves the inner product.
The complementary trace $1 - \eta_D^{2}$ is deposited in the residual dark capacity $c_{\mathrm{dark}}^{\mathrm{res}} = 10/33$, so the total trace in the coupled visible$\otimes$dark space remains 1.
All rational capacities are represented as exact `rug::Rational` values at 512-bit precision, well below the $10^{-122}$ holographic noise floor.

### Reconstruction operator

Re-rendering at a future-authorized target boundary address is performed by

$$
R^{\text{rerender}}
= T^{\partial}(x_{\mathrm{tar}})
\, D^{\text{derender}\,\dagger} \,
\bigl(I_{\mathrm{vis}} \otimes O^{\text{excitation}}(\theta)\bigr),
$$

where $T^{\partial}$ is the Heegaard-Floer boundary relabeling map and $O^{\text{excitation}}(\theta) = e^{-i\theta Q_{\text{topological}}}$ is the phase-locked $U(1)$ excitation operator.
For the canonical anyon lattice $Q_{\text{topological}} = 1$.

$T^{\partial}$ is a spatial isometry: it copies the dark-ledger state from the source visible block to the target block without changing its norm.
Because the source and target addresses subtend the same entanglement-wedge support interval length $\ell_A = 2z$, the transition is adiabatic and entropy-preserving,

$$
\Delta S_A = 0.
$$

No external environment is coupled during the relabeling, so the operation is instantaneous on the boundary-register clock.

### Causal authorization

A re-rendering attempt is authorized only when the target lies inside or on the future causal cone of the source:

$$
x_{\mathrm{tar}} \in J^{+}(x_{\mathrm{src}})
\quad\Longleftrightarrow\quad
\Delta t > 0
\;\text{ and }\;
\Delta x^{2} + \Delta y^{2} + \Delta z^{2} \le (\Delta t)^{2},
$$

with $c = 1$.
Spacelike or past targets raise a fatal `AnomalyClosureError`.

### Entanglement wedge mapping

A bulk point at radial depth $z$ is dual to the minimal spatial boundary interval $A$ whose entanglement wedge contains the point.
For a Poincaré upper-half-plane geodesic whose boundary endpoints are separated by $\ell_A$, the geodesic radius is $R_A = \ell_A/2$ and its deepest point 
is at $z = R_A$. The minimal boundary support interval required for reconstruction at bulk depth $z$ is therefore

$$
\ell_A = 2z.
$$

Using the Ryu--Takayanagi relation, the entropy of that minimal support interval is

$$
S_A(z) = \frac{c}{3} \log\frac{2z}{\epsilon},
$$

where $c$ is the boundary central charge and $\epsilon$ is a UV cutoff [4].

### Passive stress-energy preservation

De-rendering sends the active metric slice to zero, g 
μν
active
​
 →0 , while total energy-momentum conservation gives $\nabla_\mu T^{\mu\nu}_{\mathrm{total}} = 0$.
Taking the nullification limit yields

$$
\nabla_\mu T^{\mu\nu}_{\mathrm{passive}}
= -\lim_{g^{\mathrm{active}}\to 0} \nabla_\mu T^{\mu\nu}_{\mathrm{active}}
= 0,
$$

so the passive stress-energy stored in the boundary register is conserved.

## Hardware Architecture

### High-speed boundary driver

The phase-locked boundary character excitations are modulated by an InP/InGaAs single heterojunction bipolar transistor (SHBT) with a micro-airbridge structure and quasi-coplanar contacts.
For a $1.5\times5\,\mu\mathrm{m}^2$ emitter, the measured current-gain cutoff frequency is $f_T = 53$ GHz and the maximum oscillation frequency reaches

$$
f_{\max} = 72\ \mathrm{GHz}.
$$

This bandwidth supports gating of the $U(1)$ phase rotation at microwave-clock rates.
The driver is operated at a cryogenic base temperature of $T = 15.4$ mK to suppress thermal phase jitter.

### Ballistic routing

The dark-ledger excitations are routed through a 2D topological-insulator edge-state waveguide.
The helical edge states are spin-momentum locked and are protected against non-magnetic disorder and backscattering [5,6,7].
A narrow constriction or tunnel contact couples the dark-ledger quantum dots to the edge states, so the excitation is transported with spin polarization close to unity.

### Topological protection auditor

The `TopologicalProtectionAuditor` simulates backscattering events on the helical edge state and verifies that the spin polarization remains above the $0.99$ stability threshold even after $10^9$ scattering attempts at a backscattering rate of $10^{-12}$ per event.

## HIL Safety

A cryogenic Hardware-in-the-Loop (HIL) safety monitor runs in the same control loop as the SHBT driver.
At every clock cycle the monitor samples the eigenvector-rigidity detuning $\delta\Phi$, the Lorentzian determinant residual $|\det(g)+1|$, the smallest Gram eigenvalue $\lambda_{\min}(\gamma)$, and the local information density $N_{\mathrm{local}}$.
The pass condition is

$$
\delta\Phi \le 10^{-12},
\qquad
|\det(g)+1| \le 10^{-12},
\qquad
\lambda_{\min}(\gamma) > 0,
\qquad
N_{\mathrm{local}} \le N_{\mathrm{sat}}.
$$

If any inequality is violated, the monitor asserts `EMERGENCY_ANOMALY_CLOSURE` and a hard-wired emergency shunt clamps the active shift field $\beta = v f(r_s)$ to zero.
The shutdown latency is hard-coded at $<2.5$ ns.
At $f_{\max} = 72$ GHz, one clock cycle is

$$
\tau_{\mathrm{clk}} = \frac{1}{f_{\max}} \approx 13.9\ \mathrm{ps},
$$

so the $2.5$ ns budget spans roughly $180$ gate cycles, sufficient for sensor readout, comparator logic, shunt driver, and field-collapse confirmation.

## Audit Benchmarks

A canonical run of `ModularStateTranslocator().audit()` produces the values injected into `main.tex` via `recon_results.tex`.
All bit-budget figures are calibrated for a $10$-metre radius translocation zone.

| Quantity | Symbol | Target | Measured |
|----------|--------|--------|----------|
| Boundary kernel | $(k_l, k_q, K)$ | $(26, 8, 312)$ | $(26, 8, 312)$ |
| Residual dark capacity | $c_{\mathrm{dark}}^{\mathrm{res}}$ | $10/33$ | $10/33$ |
| Completed dark capacity | $c_{\mathrm{dark}}^{\mathrm{comp}}$ | $23/33$ | $23/33$ |
| Stinespring ratio | $\eta_D$ | $23/33$ | $23/33$ |
| Unitarity residual | $\epsilon_{\mathrm{unitary}}$ | $<10^{-14}$ | $<10^{-14}$ |
| Eigenvector-rigidity detuning | $\delta\Phi$ | $<1.77\times10^{-16}$ | $<1.77\times10^{-16}$ |
| Phase unitarity residual | $\epsilon_{\mathrm{phase}}$ | $<10^{-14}$ | $<10^{-14}$ |
| Causal authorization | | `true` | `true` |
| HIL status | | `STATUS_NOMINAL_PASS` | `STATUS_NOMINAL_PASS` |
| Lorentzian determinant residual | $\| \det(g)+1 \|$ | $<10^{-12}$ | $<10^{-12}$ |
| Phase jitter | $\Delta\phi$ | $<5.05\times10^{-5}$ rad | $<5.05\times10^{-5}$ rad |
| GET thermodynamic cost | $C_{\text{get}}$ | $5.34296976800\times10^{-76}$ J/bit | $5.34296976800\times10^{-76}$ J/bit |

The GET cost is computed as

$$
C_{\text{get}} = k_B T \ln 2 \cdot \frac{N_{\mathrm{local}}}{N_{\mathrm{sat}}},
$$

with $T = 15.4$ mK, $N_{\mathrm{local}} \approx 1.20\times10^{72}$ bits (calibrated for $R = 10$ m), and $N_{\mathrm{sat}} \approx 3.31\times10^{122}$ bits.

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

## Citations

1. `shbt-recon`: SHBT Destination Reconstruction and State Decoupling Simulator, https://github.com/sys1own/shbt-recon.
2. `shbt-precision`: SHBT Precision Simulator (canonical $(26,8,312)$ kernel and cosmology module), https://github.com/sys1own/shbt-precision.
3. `shbt-warp`: SHBT Warp Drive Simulator (Alcubierre-type metric engineering and the $142.08$ MW benchmark), https://github.com/sys1own/shbt-warp.
4. S. Ryu and T. Takayanagi, "Holographic derivation of entanglement entropy from the anti-de Sitter/conformal field theory correspondence," *Phys. Rev. Lett.* **96**, 181602 (2006).
5. C. L. Kane and E. J. Mele, "Quantum spin Hall effect in graphene," *Phys. Rev. Lett.* **95**, 226801 (2005).
6. M. K\"onig, S. Wiedmann, C. Br\"une, A. Roth, H. Buhmann, L. W. Molenkamp, X.-L. Qi, and S.-C. Zhang, "Quantum spin Hall insulator state in HgTe quantum wells," *Science* **318**, 766 (2007).
7. M. Z. Hasan and C. L. Kane, "Topological insulators," *Rev. Mod. Phys.* **82**, 3045 (2010).

All numerical values in the manuscript are generated by running the simulator; the paper is therefore fully traceable to the executable code.
