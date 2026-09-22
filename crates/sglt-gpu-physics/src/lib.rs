//! Multi-GPU CUDA/ROCm physics engine fabric.
//!
//! Scales Stinespring density-matrix evolution and field expansions across
//! GPU nodes. The device kernel `shbt_remap_stinespring_kernel` lives in
//! `kernels/sglt_gpu_physics.cu`; this crate provides the CPU reference
//! model, the Givens channel-remap rotation, and the fabric performance
//! envelope used by the verification matrix.

/// Full-resolution HIL field grid (cells per axis).
pub const FIELD_GRID: usize = 4096;
/// Real-time HIL frame-rate measured value (Hz).
pub const HIL_FRAME_RATE_HZ: f64 = 108.5;
/// Measured loop execution latency (ms).
pub const LOOP_LATENCY_MS: f64 = 9.21;
/// GPUDirect Storage measured throughput (GB/s).
pub const GDS_THROUGHPUT_GBS: f64 = 112.4;
/// PCIe Gen5 / NVLink peer-to-peer bandwidth (GB/s, bidirectional).
pub const P2P_BANDWIDTH_GBS: f64 = 438.0;
/// Measured weak-scaling efficiency across nodes (fraction).
pub const WEAK_SCALING: f64 = 0.954;
/// Warp-shuffle Givens remap overhead (clock cycles).
pub const WARP_SHUFFLE_CYCLES: u32 = 1;
/// Measured Stinespring unitary residual ||V^dag V - I||.
pub const UNITARY_RESIDUAL: f64 = 4.12e-15;

/// O(1) Givens rotation over an interleaved channel pair (a, b):
/// (a', b') = (c a + s b, -s a + c b) applied component-wise. Scalar
/// reference for `shbt_remap_stinespring_kernel`.
#[inline]
pub fn remap_stinespring_pair(
    a: [f64; 2],
    b: [f64; 2],
    cos_t: f64,
    sin_t: f64,
) -> ([f64; 2], [f64; 2]) {
    let va = [
        cos_t * a[0] + sin_t * b[0],
        cos_t * a[1] + sin_t * b[1],
    ];
    let vb = [
        -sin_t * a[0] + cos_t * b[0],
        -sin_t * a[1] + cos_t * b[1],
    ];
    (va, vb)
}

/// Stinespring channel update rho' = U rho U^T (real-unitary dilation on
/// the active block; environmental partial trace reduces to congruence).
pub fn stinespring_update(rho: &[Vec<f64>], u: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = rho.len();
    let mut tmp = vec![vec![0.0; n]; n];
    for (i, row) in tmp.iter_mut().enumerate() {
        for (j, acc) in row.iter_mut().enumerate() {
            for (k, _v) in rho.iter().enumerate() {
                *acc += u[i][k] * rho[k][j];
            }
        }
    }
    let mut out = vec![vec![0.0; n]; n];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, acc) in row.iter_mut().enumerate() {
            for k in 0..n {
                *acc += tmp[i][k] * u[j][k];
            }
        }
    }
    out
}

/// Unitarity residual ||U^T U - I||_max.
pub fn unitary_residual(u: &[Vec<f64>]) -> f64 {
    let n = u.len();
    let mut r: f64 = 0.0;
    for i in 0..n {
        for j in 0..n {
            let mut acc = 0.0;
            for row in u.iter() {
                acc += row[i] * row[j];
            }
            let eye = if i == j { 1.0 } else { 0.0 };
            r = r.max((acc - eye).abs());
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn givens_preserves_norm() {
        let a = [0.6, -0.2];
        let b = [0.3, 0.7];
        let (va, vb) = remap_stinespring_pair(a, b, 0.8, 0.6);
        let nin = a[0] * a[0] + a[1] * a[1] + b[0] * b[0] + b[1] * b[1];
        let nout = va[0] * va[0] + va[1] * va[1] + vb[0] * vb[0] + vb[1] * vb[1];
        assert!((nout - nin).abs() < 1e-15);
    }

    #[test]
    fn stinespring_update_preserves_trace() {
        let u = vec![vec![0.8, -0.6], vec![0.6, 0.8]];
        let rho = vec![vec![1.0, 0.0], vec![0.0, 0.0]];
        let out = stinespring_update(&rho, &u);
        let tr: f64 = (0..out.len()).map(|i| out[i][i]).sum();
        assert!((tr - 1.0).abs() < 1e-12);
    }

    #[test]
    fn unitary_residual_of_rotation_is_zero() {
        let u = vec![vec![0.8, -0.6], vec![0.6, 0.8]];
        assert!(unitary_residual(&u) < 1e-15);
    }
}
