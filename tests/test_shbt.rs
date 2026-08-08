//! Property-based tests for the Causal Authorization Protocol (CAP).
//!
//! The CAP must be infallible: every spacelike or past interval must raise a
//! fatal `AnomalyClosureError`, and every future timelike or null interval must
//! be authorized.  The `STATUS_NOMINAL_PASS` flag from the HIL monitor is only
//! reachable when the target coordinate lies inside the future causal cone
//! `J^+(x_src)`.

use shbt_recon::{verify_future_cone_fatal, CausalCoordinate, ModularStateTranslocator, ReconError};

/// Deterministic 64-bit LCG seeded with a fixed value so the property test is
/// reproducible across runs and CI environments.
struct Lcg64(u64);

impl Lcg64 {
    const MULTIPLIER: u64 = 6_364_136_223_846_793_005;
    const INCREMENT: u64 = 1_442_695_040_888_963_407;

    fn new(seed: u64) -> Self {
        Lcg64(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(Self::MULTIPLIER).wrapping_add(Self::INCREMENT);
        self.0
    }

    /// Return a deterministic `f64` in `[min, max]`.
    fn next_f64(&mut self, min: f64, max: f64) -> f64 {
        let normalized = self.next_u64() as f64 / u64::MAX as f64;
        min + normalized * (max - min)
    }

    fn coordinate(&mut self, min: f64, max: f64) -> CausalCoordinate {
        CausalCoordinate::new(
            self.next_f64(min, max),
            self.next_f64(min, max),
            self.next_f64(min, max),
            self.next_f64(min, max),
        )
    }
}

fn unit_residual() -> Vec<f64> {
    vec![1.0 / (8.0f64).sqrt(); 8]
}

/// Interval classification used to decide the expected CAP outcome.
///
/// The future causal cone `J^+(x_src)` is defined by `dt >= 0` and
/// `spatial_interval <= dt^2`.  This corresponds to `Δs^2 >= 0` together with
/// a non-negative time separation, i.e. a future-pointing timelike or null
/// interval.
fn is_future_cone(src: &CausalCoordinate, tar: &CausalCoordinate) -> bool {
    let dt = tar.t - src.t;
    if dt < 0.0 {
        return false;
    }
    let dx = tar.x - src.x;
    let dy = tar.y - src.y;
    let dz = tar.z - src.z;
    let spatial = dx * dx + dy * dy + dz * dz;
    spatial <= dt * dt
}

/// Generate `n` deterministic source/target pairs and assert that the
/// low-level `verify_future_cone_fatal` function correctly separates
/// spacelike/past pairs from future causal pairs.
#[test]
fn cap_verify_future_cone_property_test() {
    let mut rng = Lcg64::new(0xCAFE_F00D_DEAD_BEEF);
    let r = 10.0;
    let n = 5_000;

    for _ in 0..n {
        let src = rng.coordinate(-r, r);
        // Use a larger displacement range so the pair covers both inside and
        // outside the future cone.
        let tar = rng.coordinate(-2.0 * r, 2.0 * r);
        let future = is_future_cone(&src, &tar);
        let result = verify_future_cone_fatal(&src, &tar);

        if future {
            assert!(
                result.is_ok(),
                "Future timelike/null target was rejected: {:?}",
                tar
            );
        } else {
            assert!(
                matches!(result, Err(ReconError::AnomalyClosureError(_))),
                "Spacelike or past target was not rejected: {:?}",
                tar
            );
        }
    }
}

/// End-to-end CAP test through `ModularStateTranslocator`.
///
/// The translocator runs the full HIL pre-check before the causal-authorization
/// gate.  `translocate` therefore only returns `Ok` (and the HIL monitor only
/// reaches `STATUS_NOMINAL_PASS`) when the target is inside `J^+(x_src)`.
#[test]
fn cap_translocate_status_nominal_only_in_future_cone() {
    let mut rng = Lcg64::new(0xBADC_0FFEE_0DDF00D);
    let r = 10.0;
    let n = 1_000;
    let residual = unit_residual();
    let mut trans = ModularStateTranslocator::new().unwrap();

    for _ in 0..n {
        let src = rng.coordinate(-r, r);
        let tar = rng.coordinate(-2.0 * r, 2.0 * r);
        let future = is_future_cone(&src, &tar);

        // active_velocity_c=0.0 keeps the metric audit trivially inside its
        // HIL bounds; theta=0.0 removes the phase-locked rotation, leaving the
        // causal-authorization gate as the dominant branch.
        let result = trans.translocate(
            residual.clone(),
            src,
            tar,
            0.0,
            0,
            1,
            0.0,
        );

        if future {
            assert!(
                result.is_ok(),
                "Translocate failed for a future-cone target: {:?}",
                tar
            );
            let payload = result.unwrap();
            assert_eq!(payload.len(), 8);
        } else {
            assert!(
                result.is_err(),
                "Translocate succeeded for a non-future target: {:?}",
                tar
            );
        }
    }
}

/// Explicit boundary cases for light-cone and spacelike limits.
#[test]
fn cap_explicit_boundary_cases() {
    let mut trans = ModularStateTranslocator::new().unwrap();
    let residual = unit_residual();
    let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);

    // Exactly on the future light cone.
    let tar_null = CausalCoordinate::new(1.0, 1.0, 0.0, 0.0);
    assert!(verify_future_cone_fatal(&src, &tar_null).is_ok());
    assert!(trans
        .translocate(residual.clone(), src, tar_null, 0.0, 0, 1, 0.0)
        .is_ok());

    // Just outside the future light cone.
    let tar_spacelike = CausalCoordinate::new(1.0, 1.000_000_000_001, 0.0, 0.0);
    assert!(matches!(
        verify_future_cone_fatal(&src, &tar_spacelike),
        Err(ReconError::AnomalyClosureError(_))
    ));
    assert!(trans
        .translocate(residual.clone(), src, tar_spacelike, 0.0, 0, 1, 0.0)
        .is_err());

    // Past timelike interval must also be rejected.
    let tar_past = CausalCoordinate::new(-1.0, 0.0, 0.0, 0.0);
    assert!(matches!(
        verify_future_cone_fatal(&src, &tar_past),
        Err(ReconError::AnomalyClosureError(_))
    ));
    assert!(trans
        .translocate(residual, src, tar_past, 0.0, 0, 1, 0.0)
        .is_err());
}
