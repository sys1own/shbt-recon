// CUDA/ROCm unified microkernel: O(1) Givens rotation & Stinespring
// channel remap. Executes the G(i, j, theta) rotation across interleaved
// channel pairs with warp-level register access — the CPU reference model
// for the device kernel lives in sglt-gpu-physics (remap_stinespring_pair).
#include <cuda_runtime.h>
#include <device_launch_parameters.h>

__global__ void shbt_remap_stinespring_kernel(
    const double2* __restrict__ d_psi_in,
    double2* __restrict__ d_psi_out,
    const double cos_theta,
    const double sin_theta,
    const int dim)
{
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    int stride = gridDim.x * blockDim.x;

    for (int i = idx; i < dim / 2; i += stride) {
        int idx_a = 2 * i;
        int idx_b = 2 * i + 1;

        double2 u_a = d_psi_in[idx_a];
        double2 u_b = d_psi_in[idx_b];

        double2 v_a;
        v_a.x = cos_theta * u_a.x + sin_theta * u_b.x;
        v_a.y = cos_theta * u_a.y + sin_theta * u_b.y;

        double2 v_b;
        v_b.x = -sin_theta * u_a.x + cos_theta * u_b.x;
        v_b.y = -sin_theta * u_a.y + cos_theta * u_b.y;

        d_psi_out[idx_a] = v_a;
        d_psi_out[idx_b] = v_b;
    }
}
