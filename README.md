# black-hole-thesis

Simulating black hole formation in Rust. A redo of my undergraduate thesis.

Brifely and non-technically, we are simulating the evolution of a bubble of energy in a confined region of space. The idea is that no matter how weak the initial bubble of energy starts, it gets denser and denser under its own self-gravity as it bounces around inside the region, and eventually forms a black hole.

More info/documentation/blogging to come soon. This is still an early WIP.


---

## AI Summary:

Here’s the high-level picture first: we evolve a spherically symmetric massless scalar field in a finite cavity (r\in[0,1]) coupled to Einstein’s equations in polar-areal gauge. We use a first-order reduction of the wave equation; the constraints are solved on each time slice; and the cavity is perfectly reflecting. Regularity at the center and a Dirichlet wall at the outer boundary are enforced via clean parity/characteristic boundary conditions that keep the system first-order everywhere.

---

**Domain and fields.** (r\in[0,1]), (t\ge 0). Metric
[
ds^2=-\frac{A}{N^2},dt^2+\frac{dr^2}{A}+r^2 d\Omega .
]
First-order variables
[
\Phi:=\partial_r\phi,\qquad \Pi:=\frac{A}{N},\partial_t\phi .
]

**Evolution (first-order Klein–Gordon).**
[
\partial_t\Phi=\partial_r!\left(\tfrac{A}{N}\Pi\right),\qquad
\partial_t\Pi=\frac{1}{r^2},\partial_r!\left(r^2\tfrac{A}{N}\Phi\right).
]

**Constraints (solved on each time slice).** Let (m:=\tfrac12 r(1-A)). Then
[
\frac{N'}{N}=-,r\left(\Phi^2+\Pi^2\right),\qquad
m'=\tfrac12 r^2\left(\Phi^2+\Pi^2\right),\qquad
A=1-\frac{2m}{r}.
]
The identity
[
\partial_t m=\frac{A^2}{N},r^2,\Phi,\Pi
]
is monitored as a diagnostic (not used to evolve).

## Boundary conditions

### Left boundary (r=0) — regularity

Continuum conditions:
[
\Phi(0,t)=0,\qquad \partial_r\Pi(0,t)=0,\qquad m(0,t)=0,\qquad A(0,t)=1.
]
Numerical parity (ghost fills about (r=0)): (\phi,\Pi,A,N) even; (\Phi,m) odd.

### Right boundary (r=1) — perfectly reflecting wall

Dirichlet wall for the scalar:
[
\phi(1,t)=0.
]
In first-order variables this yields
[
\Pi(1,t)=0.
]
Closure for (\Phi) is provided by reflection parity (or, equivalently, characteristics):

* **Parity form (ghost-cell friendly):**
  [
  \phi(1+\delta,t)=-\phi(1-\delta,t),\quad
  \Pi(1+\delta,t)=-\Pi(1-\delta,t),\quad
  \Phi(1+\delta,t)=+\Phi(1-\delta,t).
  ]
  This makes (\Phi) even at the wall, so (\partial_r\Phi(1,t)=0) follows automatically from centered differences.

Gauge choice at the wall:
[
N(1,t)=1.
]

## Initial data

Example smooth data used in the confined problem:
[
\phi(r,0)=0,\qquad \Phi(r,0)=0,\qquad
\Pi(r,0)=\varepsilon,\exp!\left[-64,\tan^2!\Big(\frac{\pi r}{2}\Big)\right].
]
Given (\Phi(\cdot,0),\Pi(\cdot,0)), obtain (m(\cdot,0)) and (N(\cdot,0)) by integrating the constraints with
[
m(0,0)=0,\qquad N(1,0)=1.
]
That is,
[
m(r,0)=\frac12\int_{0}^{r}s^2!\left(\Phi^2+\Pi^2\right)!(s,0),ds,\qquad
N(r,0)=\exp!\left(-\int_{r}^{1}s,\left(\Phi^2+\Pi^2\right)!(s,0),ds\right).
]

## Diagnostics

Total mass inside the cavity:
[
m(1,t)=\frac12\int_{0}^{1}r^2\left(\Phi^2+\Pi^2\right),dr.
]
Energy flux at the wall vanishes under the reflecting BCs because (\Pi(1,t)=0), so the cavity is lossless and the mass (m(1,t)) is conserved up to numerical error.
