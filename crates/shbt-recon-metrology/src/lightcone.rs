//! Hardware causal lightcone authorization — verifies target reconstruction
//! points satisfy x_tar ∈ J⁺(x_src) before the metrology mesh releases the
//! boundary relabeling map.

/// Spacetime point (t, x, y, z), signature −+++.
pub type Point4 = [f64; 4];

/// Interval η_μν Δx^μ Δx^ν = −Δt² + Δx⃗².
pub fn interval(src: Point4, tar: Point4) -> f64 {
    let dt = tar[0] - src[0];
    -(dt * dt) + (tar[1] - src[1]).powi(2) + (tar[2] - src[2]).powi(2) + (tar[3] - src[3]).powi(2)
}

/// Hardware causal authorization verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Authorization {
    /// x_tar ∈ J⁺(x_src): timelike/lightlike future-directed.
    Granted,
    /// Spacelike separation (outside the cone).
    DeniedSpacelike,
    /// Past-directed (inside cone but t_tar < t_src).
    DeniedPast,
}

/// Authorize a reconstruction hop src → tar.
pub fn authorize(src: Point4, tar: Point4) -> Authorization {
    if tar[0] < src[0] {
        return Authorization::DeniedPast;
    }
    if interval(src, tar) > 0.0 {
        return Authorization::DeniedSpacelike;
    }
    Authorization::Granted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grants_timelike_future() {
        assert_eq!(
            authorize([0.0; 4], [1.0, 0.9, 0.0, 0.0]),
            Authorization::Granted
        );
    }

    #[test]
    fn denies_spacelike_and_past() {
        assert_eq!(
            authorize([0.0; 4], [0.1, 5.0, 0.0, 0.0]),
            Authorization::DeniedSpacelike
        );
        assert_eq!(
            authorize([1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]),
            Authorization::DeniedPast
        );
    }
}
