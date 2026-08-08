//! Vectorised, branchless U(1) phase-locked excitation.
//!
//! The operator `O^excitation(θ) = exp(-iθ)` is applied to a fixed-size block
//! of eight complex amplitudes.  The implementation is selected at compile time
//! for x86_64 (AVX-512) or aarch64 (Neon), with a scalar fallback for other
//! targets or when the relevant SIMD feature is not detected at runtime.

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

const BLOCK_DIM: usize = 8;

/// Apply `exp(-iθ)` to the eight `(re, im)` pairs stored in two contiguous
/// arrays of length `BLOCK_DIM`.
///
/// The operation is branchless inside the hot path: a runtime feature check
/// selects the fastest implementation once per call, then all eight rotations
/// are performed without per-element conditionals.
pub fn rotate_block(theta: f64, re: &mut [f64; BLOCK_DIM], im: &mut [f64; BLOCK_DIM]) {
    rotate_block_precomputed(theta.cos(), theta.sin(), re, im);
}

pub fn rotate_block_precomputed(
    cos: f64,
    sin: f64,
    re: &mut [f64; BLOCK_DIM],
    im: &mut [f64; BLOCK_DIM],
) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            unsafe {
                return rotate_block_avx512(cos, sin, re, im);
            }
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            unsafe {
                return rotate_block_neon(cos, sin, re, im);
            }
        }
    }

    rotate_block_scalar(cos, sin, re, im);
}

fn rotate_block_scalar(cos: f64, sin: f64, re: &mut [f64; BLOCK_DIM], im: &mut [f64; BLOCK_DIM]) {
    for i in 0..BLOCK_DIM {
        let a = re[i];
        let b = im[i];
        // (a + i b) * (cos - i sin) = (a cos + b sin) + i (b cos - a sin)
        re[i] = a.mul_add(cos, b * sin);
        im[i] = b.mul_add(cos, -a * sin);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn rotate_block_avx512(
    cos: f64,
    sin: f64,
    re: &mut [f64; BLOCK_DIM],
    im: &mut [f64; BLOCK_DIM],
) {
    let a = _mm512_loadu_pd(re.as_ptr());
    let b = _mm512_loadu_pd(im.as_ptr());
    let c = _mm512_set1_pd(cos);
    let s = _mm512_set1_pd(sin);

    // new_re = a * cos + b * sin
    let new_re = _mm512_fmadd_pd(b, s, _mm512_mul_pd(a, c));
    // new_im = b * cos - a * sin = b * cos + (-a) * sin
    let neg_a = _mm512_sub_pd(_mm512_setzero_pd(), a);
    let new_im = _mm512_fmadd_pd(neg_a, s, _mm512_mul_pd(b, c));

    _mm512_storeu_pd(re.as_mut_ptr(), new_re);
    _mm512_storeu_pd(im.as_mut_ptr(), new_im);
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn rotate_block_neon(
    cos: f64,
    sin: f64,
    re: &mut [f64; BLOCK_DIM],
    im: &mut [f64; BLOCK_DIM],
) {
    let c = vdupq_n_f64(cos);
    let s = vdupq_n_f64(sin);

    for i in (0..BLOCK_DIM).step_by(2) {
        let a = vld1q_f64(re.as_ptr().add(i));
        let b = vld1q_f64(im.as_ptr().add(i));

        // new_re = a * c + b * s
        let ac = vmulq_f64(a, c);
        let new_re = vfmaq_f64(ac, b, s);

        // new_im = b * c - a * s
        let bs = vmulq_f64(a, s);
        let new_im = vsubq_f64(vmulq_f64(b, c), bs);

        vst1q_f64(re.as_mut_ptr().add(i), new_re);
        vst1q_f64(im.as_mut_ptr().add(i), new_im);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn quarter_turn_rotates_correctly() {
        let mut re: [f64; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let mut im: [f64; 8] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        rotate_block(PI / 2.0, &mut re, &mut im);

        for i in 0..8 {
            // (re + i im) * (cos - i sin) with cos=0, sin=1 => re' = im, im' = -re
            let old_re = i as f64 + 1.0;
            let old_im = i as f64;
            assert!((re[i] - old_im).abs() < 1e-15, "re[{}] = {}", i, re[i]);
            assert!((im[i] + old_re).abs() < 1e-15, "im[{}] = {}", i, im[i]);
        }
    }

    #[test]
    fn rotation_preserves_norm() {
        let theta = 0.421;
        let mut re = [0.3535533905932738; 8];
        let mut im = [0.3535533905932738; 8];
        rotate_block(theta, &mut re, &mut im);
        let norm_sq: f64 = re.iter().zip(im.iter()).map(|(r, i)| r * r + i * i).sum();
        assert!((norm_sq - 2.0).abs() < 1e-14, "norm_sq = {}", norm_sq);
    }

    #[test]
    fn phase_rotation_throughput() {
        use std::time::Instant;
        let theta: f64 = 0.421;
        let cos = theta.cos();
        let sin = theta.sin();
        let mut re = [0.3535533905932738; 8];
        let mut im = [0.3535533905932738; 8];
        let n = 10_000_000;
        for _ in 0..1_000 {
            rotate_block_precomputed(cos, sin, &mut re, &mut im);
        }
        let start = Instant::now();
        for _ in 0..n {
            rotate_block_precomputed(cos, sin, &mut re, &mut im);
        }
        let elapsed = start.elapsed();
        eprintln!(
            "phase_rotation throughput: {} rotations in {:?} ({:.2} ns/rotation)",
            n,
            elapsed,
            elapsed.as_nanos() as f64 / n as f64
        );
    }
}
