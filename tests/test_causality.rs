use rug::Float;
use shbt_recon::{verify_future_cone_fatal, CausalCoordinate, ReconError};

fn is_spacelike_with_prec(t: f64, x: f64) -> bool {
    let prec = 512;
    let t_f = Float::with_val(prec, t);
    let x_f = Float::with_val(prec, x);

    let mut x2 = x_f.clone();
    x2.square_mut();
    let mut t2 = t_f.clone();
    t2.square_mut();

    let delta = x2 - t2;
    let zero = Float::with_val(prec, 0.0);
    delta > zero
}

#[test]
fn null_limit_authorization_accepts_light_cone() {
    let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
    let tar = CausalCoordinate::new(1.0, 1.0, 0.0, 0.0);

    // Verify with 512-bit precision that the point is exactly on the light cone.
    assert!(!is_spacelike_with_prec(1.0, 1.0));

    // The on-cone target must be authorized.
    let result = verify_future_cone_fatal(&src, &tar);
    assert!(
        result.is_ok(),
        "A target on the future light cone should be authorized"
    );
}

#[test]
fn null_limit_authorization_rejects_just_outside_light_cone() {
    let src = CausalCoordinate::new(0.0, 0.0, 0.0, 0.0);
    let tar = CausalCoordinate::new(1.0, 1.000000000001, 0.0, 0.0);

    // Verify with 512-bit precision that the point is outside the light cone.
    assert!(is_spacelike_with_prec(1.0, 1.000000000001));

    // The barely spacelike target must raise a fatal AnomalyClosureError.
    let result = verify_future_cone_fatal(&src, &tar);
    assert!(
        matches!(result, Err(ReconError::AnomalyClosureError(_))),
        "A target just outside the future light cone must raise AnomalyClosureError, got {:?}",
        result
    );
}
