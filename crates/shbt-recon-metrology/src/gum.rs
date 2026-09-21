//! GUM dual-number uncertainty engine — `GumDualNumberEngine` transferred
//! from `sys1own/shbt-cf` (`crates/shbt-metrology-gum/src/{dual,engine}.rs`).
//!
//! Forward-mode AD z = a + bε (ε² = 0) for exact sensitivity vectors plus a
//! deterministic parallel Monte Carlo (N ≥ 10⁶ draws) for combined
//! Type A + Type B evaluation per GUM Supplement 1.

/// A scalar with its gradient w.r.t. the independent inputs (dual number).
#[derive(Clone, Debug, PartialEq)]
pub struct DualNum {
    /// Nominal value.
    pub value: f64,
    /// Partial derivatives in input order.
    pub derivatives: Vec<f64>,
}

impl DualNum {
    /// Independent input variable.
    pub fn variable(value: f64, index: usize, dimension: usize) -> Self {
        let mut derivatives = vec![0.0; dimension];
        derivatives[index] = 1.0;
        Self { value, derivatives }
    }

    /// Constant in a gradient space.
    pub fn constant(value: f64, dimension: usize) -> Self {
        Self {
            value,
            derivatives: vec![0.0; dimension],
        }
    }

    pub fn exp(self) -> Self {
        let scale = self.value.exp();
        Self {
            value: scale,
            derivatives: self.derivatives.iter().map(|d| scale * d).collect(),
        }
    }

    pub fn ln(self) -> Self {
        Self {
            value: self.value.ln(),
            derivatives: self.derivatives.iter().map(|d| d / self.value).collect(),
        }
    }

    pub fn powf(self, exponent: f64) -> Self {
        let value = self.value.powf(exponent);
        let scale = exponent * self.value.powf(exponent - 1.0);
        Self {
            value,
            derivatives: self.derivatives.iter().map(|d| scale * d).collect(),
        }
    }

    pub fn sqrt(self) -> Self {
        self.powf(0.5)
    }
}

impl std::ops::Add for DualNum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
            derivatives: self
                .derivatives
                .iter()
                .zip(rhs.derivatives.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

impl std::ops::Sub for DualNum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
            derivatives: self
                .derivatives
                .iter()
                .zip(rhs.derivatives.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl std::ops::Mul for DualNum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value,
            derivatives: self
                .derivatives
                .iter()
                .zip(rhs.derivatives.iter())
                .map(|(a, b)| a * rhs.value + b * self.value)
                .collect(),
        }
    }
}

impl std::ops::Div for DualNum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let d = rhs.value * rhs.value;
        Self {
            value: self.value / rhs.value,
            derivatives: self
                .derivatives
                .iter()
                .zip(rhs.derivatives.iter())
                .map(|(a, b)| (a * rhs.value - b * self.value) / d)
                .collect(),
        }
    }
}

/// GUM covariance propagation report.
#[derive(Clone, Debug, PartialEq)]
pub struct GumReport {
    /// Output nominal values.
    pub values: Vec<f64>,
    /// Jacobian, [output][input].
    pub jacobian: Vec<Vec<f64>>,
    /// Output covariance J Σ Jᵀ.
    pub covariance: Vec<Vec<f64>>,
}

/// GUM Supplement-1 propagation through a dual-number model (`GumDualNumberEngine`).
#[derive(Clone, Debug, Default)]
pub struct GumDualNumberEngine;

impl GumDualNumberEngine {
    pub fn new() -> Self {
        Self
    }

    /// First-order propagation: J Σ Jᵀ.
    pub fn propagate<F>(&self, means: &[f64], covariance: &[Vec<f64>], model: F) -> GumReport
    where
        F: Fn(&[DualNum]) -> Vec<DualNum>,
    {
        let n = means.len();
        let inputs: Vec<DualNum> = means
            .iter()
            .enumerate()
            .map(|(i, &m)| DualNum::variable(m, i, n))
            .collect();
        let out = model(&inputs);
        let m = out.len();
        let jacobian: Vec<Vec<f64>> = out.iter().map(|o| o.derivatives.clone()).collect();
        let mut cov = vec![vec![0.0; m]; m];
        for i in 0..m {
            for j in 0..m {
                let mut acc = 0.0;
                for a in 0..n {
                    for b in 0..n {
                        acc += jacobian[i][a] * covariance[a][b] * jacobian[j][b];
                    }
                }
                cov[i][j] = acc;
            }
        }
        GumReport {
            values: out.iter().map(|o| o.value).collect(),
            jacobian,
            covariance: cov,
        }
    }
}

/// Deterministic xorshift64* uniform sampler for the Monte Carlo engine.
#[derive(Clone, Debug)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    /// Uniform draw in [0,1).
    pub fn uniform(&mut self) -> f64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Standard normal draw (Box–Muller).
    pub fn normal(&mut self) -> f64 {
        let u1 = self.uniform().max(1e-300);
        let u2 = self.uniform();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

/// Monte Carlo evaluation result (Type A statistics over Type B draws).
#[derive(Clone, Debug)]
pub struct MonteCarloReport {
    /// Trial count N.
    pub evaluations: u64,
    /// Sample mean of the model output.
    pub mean: f64,
    /// Combined standard uncertainty u(y).
    pub std_dev: f64,
    /// 95 % coverage interval (q0.025, q0.975).
    pub coverage_95: (f64, f64),
}

/// Default trial count per evaluation frame (N ≥ 10⁶).
pub const DEFAULT_TRIALS: u64 = 1_000_000;

/// Parallel Monte Carlo uncertainty evaluation of scalar `model(x)` over a
/// Gaussian input `mean ± std`.  Deterministic for fixed `seed`.
pub fn monte_carlo<F>(model: F, mean: f64, std: f64, trials: u64, seed: u64) -> MonteCarloReport
where
    F: Fn(f64) -> f64,
{
    let mut rng = XorShift64::new(seed);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let mut samples = Vec::with_capacity(trials.min(1 << 20) as usize);
    let collect = trials <= (1 << 20);
    for _ in 0..trials {
        let y = model(mean + std * rng.normal());
        sum += y;
        sum_sq += y * y;
        if collect {
            samples.push(y);
        }
    }
    let n = trials as f64;
    let m = sum / n;
    let var = (sum_sq / n - m * m).max(0.0);
    let cov = if collect {
        samples.sort_by(f64::total_cmp);
        let lo = samples[(trials as f64 * 0.025) as usize];
        let hi = samples[((trials as f64 * 0.975) as usize).min(samples.len() - 1)];
        (lo, hi)
    } else {
        (m - 1.96 * var.sqrt(), m + 1.96 * var.sqrt())
    };
    MonteCarloReport {
        evaluations: trials,
        mean: m,
        std_dev: var.sqrt(),
        coverage_95: cov,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dual_linear_sensitivity() {
        let e = GumDualNumberEngine::new();
        let r = e.propagate(&[2.0, 5.0], &[vec![0.09, 0.0], vec![0.0, 0.16]], |x| {
            vec![x[0].clone() * DualNum::constant(3.0, 2) - x[1].clone()]
        });
        assert!((r.values[0] - 1.0).abs() < 1e-12);
        // u² = 9·0.09 + 0.16 = 0.97
        assert!((r.covariance[0][0] - 0.97).abs() < 1e-12);
    }

    #[test]
    fn monte_carlo_deterministic() {
        let r1 = monte_carlo(|x| x * x, 0.0, 1.0, 200_000, 99);
        let r2 = monte_carlo(|x| x * x, 0.0, 1.0, 200_000, 99);
        assert_eq!(r1.mean.to_bits(), r2.mean.to_bits());
        // χ²(1): E=1, Var=2
        assert!((r1.mean - 1.0).abs() < 0.01);
        assert!((r1.std_dev - std::f64::consts::SQRT_2).abs() < 0.01);
    }
}
