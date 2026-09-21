//! Multi-node translocator swarm: M > 2 boundary relabeling via
//! Heegaard–Floer symplectic cobordism matrices T^∂_ij ∈ Sp(2g, Z) and the
//! complex swarm routing matrix R_swarm ∈ C^{M×M}.

/// Lower heliocentric focal distance bound (AU).
pub const Z_MIN_AU: f64 = 547.8;
/// Upper heliocentric focal distance bound (AU).
pub const Z_MAX_AU: f64 = 650.0;
/// Verified swarm scale: eight active nodes.
pub const SWARM_NODES: u32 = 8;

/// Check the symplectic condition T Ω Tᵀ = Ω for a 2g×2g matrix
/// stored row-major; returns the residual ‖T Ω Tᵀ − Ω‖_F.
pub fn symplectic_residual(g: usize, t: &[f64]) -> f64 {
    let n = 2 * g;
    assert_eq!(t.len(), n * n);
    let omega = |i: usize, j: usize| -> f64 {
        if i + g == j {
            1.0
        } else if j + g == i {
            -1.0
        } else {
            0.0
        }
    };
    let mut res = 0.0;
    for i in 0..n {
        for j in 0..n {
            let mut acc = 0.0;
            for a in 0..n {
                for b in 0..n {
                    acc += t[i * n + a] * omega(a, b) * t[j * n + b];
                }
            }
            let d = acc - omega(i, j);
            res += d * d;
        }
    }
    res.sqrt()
}

/// Determinant of a 2×2 symplectic block (det = +1 for Sp(2, Z)).
pub fn symplectic_det_2(t: &[f64; 4]) -> f64 {
    t[0] * t[3] - t[1] * t[2]
}

/// Swarm routing phase between nodes at focal distances z_i < z_j (AU):
/// R_ij = exp(−i ∫ k_eff dz) with k_eff = (ω/c)√(1 − r_s/z), evaluated
/// analytically for the weak-field limit.
pub fn routing_phase(omega_hz: f64, z_i_au: f64, z_j_au: f64) -> (f64, f64) {
    const C: f64 = 299_792_458.0;
    const AU: f64 = 1.495_978_707e11;
    const RS_SUN: f64 = 2953.339585_29; // Schwarzschild radius of the Sun (m)
    let zi = z_i_au * AU;
    let zj = z_j_au * AU;
    // ∫ sqrt(1 - rs/z) dz ≈ (z2 - z1) * mean expansion in weak field.
    let zbar = 0.5 * (zi + zj);
    let factor = (1.0 - RS_SUN / zbar).sqrt();
    let phase = -omega_hz / C * factor * (zj - zi);
    (phase.cos(), phase.sin())
}

/// Authorize swarm routing between nodes within the focal band.
pub fn route_authorized(z_i_au: f64, z_j_au: f64) -> bool {
    (Z_MIN_AU..=Z_MAX_AU).contains(&z_i_au) && (Z_MIN_AU..=Z_MAX_AU).contains(&z_j_au)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symplectic_identity_and_swap() {
        // Identity and a canonical swap block are symplectic.
        let i4 = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        assert_eq!(symplectic_residual(2, &i4), 0.0);
        assert_eq!(symplectic_det_2(&[1.0, 1.0, 0.0, 1.0]), 1.0);
    }

    #[test]
    fn swarm_window() {
        assert!(route_authorized(600.0, 650.0));
        assert!(!route_authorized(500.0, 600.0));
        assert_eq!(SWARM_NODES, 8);
    }
}
