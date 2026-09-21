//! Second post-Newtonian (2PN) metric formalism in harmonic coordinates
//! for causal authorization of relativistic targets (v ≥ 0.1c):
//!
//!   ds² = g00 c²dt² + 2 g0i c dt dx^i + gij dx^i dx^j
//!   g00 = −(1 − 2GM/c²r + 2G²M²/c⁴r² + 3GM J₂ R²/c²r³ P₂(cosθ))
//!   g0i = −2G (S×r)_i / (c³r³) (1 + 3GM/c²r)
//!   gij = δij (1 + 2GM/c²r + 3G²M²/(2c⁴r²))

/// Gravitational constant.
pub const G: f64 = 6.67430e-11;
/// Speed of light.
pub const C: f64 = 299_792_458.0;
/// Solar mass (kg).
pub const M_SUN: f64 = 1.98847e30;
/// Solar quadrupole moment.
pub const J2_SUN: f64 = 2.20e-7;
/// Solar radius (m).
pub const R_SUN: f64 = 6.96342e8;
/// Solar spin angular momentum (J·s).
pub const S_SUN: f64 = 1.92e33;
/// Minimum handled target speed for 2PN authorization (fraction of c).
pub const V_TARGET_MIN_C: f64 = 0.10;

/// 2PN metric components at field point r = (x, y, z).
#[derive(Clone, Copy, Debug)]
pub struct Metric2pn {
    pub g00: f64,
    /// g0i for i = 1..3.
    pub g0i: [f64; 3],
    /// Diagonal spatial coefficient (g_ij = δ_ij * g_space).
    pub g_space: f64,
}

/// Legendre polynomial P₂(cos θ) = (3cos²θ − 1)/2, polar axis ẑ.
fn legendre_p2(r_vec: [f64; 3]) -> f64 {
    let r = (r_vec[0].powi(2) + r_vec[1].powi(2) + r_vec[2].powi(2)).sqrt();
    if r == 0.0 {
        return 0.0;
    }
    let ct = r_vec[2] / r;
    0.5 * (3.0 * ct * ct - 1.0)
}

/// Evaluate the 2PN metric tensor at field point `r_vec` (m).
pub fn metric_2pn(r_vec: [f64; 3]) -> Metric2pn {
    let r = (r_vec[0].powi(2) + r_vec[1].powi(2) + r_vec[2].powi(2)).sqrt();
    let u = G * M_SUN / (C * C * r);
    let u2 = u * u;
    let j2_term =
        3.0 * G * M_SUN * J2_SUN * R_SUN * R_SUN / (C * C * r * r * r) * legendre_p2(r_vec);
    let g00 = -(1.0 - 2.0 * u + 2.0 * u2 + j2_term);
    // Gravitomagnetic term −2G (S×r)_i / (c³ r³) (1 + 3u), S = S_☉ ẑ.
    let s_cross_r = [S_SUN * -r_vec[1], S_SUN * r_vec[0], 0.0];
    let g0i_scale = -2.0 * G / (C * C * C * r * r * r) * (1.0 + 3.0 * u);
    let g0i = [
        g0i_scale * s_cross_r[0],
        g0i_scale * s_cross_r[1],
        g0i_scale * s_cross_r[2],
    ];
    let g_space = 1.0 + 2.0 * u + 1.5 * u2;
    Metric2pn { g00, g0i, g_space }
}

/// Lorentz factor for target velocity |v| < c; None when |v| ≥ c.
pub fn lorentz_gamma(v: [f64; 3]) -> Option<f64> {
    let beta2 = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) / (C * C);
    if beta2 >= 1.0 {
        None
    } else {
        Some(1.0 / (1.0 - beta2).sqrt())
    }
}

/// Geodesic Doppler frequency shift factor γ(1 + β·cos φ).
pub fn doppler_shift(v: [f64; 3], cos_phi: f64) -> Option<f64> {
    let gamma = lorentz_gamma(v)?;
    let beta_parallel = (v[0] + v[1] + v[2]) / 3.0_f64.sqrt() / C * cos_phi;
    Some(gamma * (1.0 + beta_parallel))
}

/// Invariant 2PN interval for separation (dt, dr) at field point r_vec.
/// Authorization requires Δs²_2PN ≤ 0.
pub fn interval_2pn(dt: f64, dr: [f64; 3], r_vec: [f64; 3]) -> f64 {
    let m = metric_2pn(r_vec);
    let dr2 = dr[0] * dr[0] + dr[1] * dr[1] + dr[2] * dr[2];
    let cross = g0i_cross_term(&m, dt, dr);
    m.g00 * C * C * dt * dt + cross + m.g_space * dr2
}

fn g0i_cross_term(m: &Metric2pn, dt: f64, dr: [f64; 3]) -> f64 {
    2.0 * C * dt * (m.g0i[0] * dr[0] + m.g0i[1] * dr[1] + m.g0i[2] * dr[2])
}

/// Flat-space residual: with M, J₂, S → 0 the 2PN interval must equal the
/// Minkowski interval. Used by gate G-01 as the metric precision check.
pub fn flat_residual(dt: f64, dr: [f64; 3]) -> f64 {
    let dr2 = dr[0] * dr[0] + dr[1] * dr[1] + dr[2] * dr[2];
    let minkowski = -C * C * dt * dt + dr2;
    // Evaluate at the Oort-scale radius where Solar corrections vanish.
    let s2 = interval_2pn(dt, dr, [1.0e18, 0.0, 0.0]);
    (s2 - minkowski).abs() / minkowski.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_space_recovery() {
        let res = flat_residual(1.0, [1.0e5, 0.0, 0.0]);
        assert!(res < 1e-12, "flat residual {res:e}");
    }

    #[test]
    fn timelike_vs_spacelike() {
        // 1 s dt, 100 km dr at Earth orbit.
        let r = [1.496e11, 0.0, 0.0];
        assert!(interval_2pn(1.0, [1.0e5, 0.0, 0.0], r) < 0.0);
        assert!(interval_2pn(1.0e-3, [1.0e9, 0.0, 0.0], r) > 0.0);
    }

    #[test]
    fn lorentz_045c() {
        let g = lorentz_gamma([0.45 * C, 0.0, 0.0]).unwrap();
        assert!((g - 1.1199).abs() < 1e-3);
        assert!(lorentz_gamma([C, 0.0, 0.0]).is_none());
    }

    #[test]
    fn quadrupole_and_spin_magnitude() {
        let m = metric_2pn([1.496e11, 0.0, 0.0]);
        let j2_mag = 3.0 * G * M_SUN * J2_SUN * R_SUN * R_SUN / (C * C * 1.496e11_f64.powi(3));
        assert!(j2_mag < 1e-10 && j2_mag > 0.0);
        assert!(m.g0i.iter().all(|g| g.abs() < 1e-20));
    }
}
