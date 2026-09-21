#include "shbt_causal_kernel.h"
#include <stdint.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>

static inline void mmio_write_barrier(void) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ __volatile__("mfence" ::: "memory");
#else
    __asm__ __volatile__("" ::: "memory");
#endif
}

void shbt_causal_init(shbt_mmio_block_t *mmio) {
    mmio->causal_cone_lo = 0x00000000U;
    mmio->causal_cone_hi = 0x00000000U;
    mmio->pn2_metric_m0 = M_SUN;
    mmio->pn2_metric_j2 = J2_SUN;
    mmio->pn2_spin_vec[0] = 0.0f;
    mmio->pn2_spin_vec[1] = 0.0f;
    mmio->pn2_spin_vec[2] = 1.92e33f;
    mmio->target_vel_gamma = 0x00010000U;
    mmio->ds2_interval_lo = 0U;
    mmio->ds2_interval_hi = 0;
    mmio->quench_time_ns = 0U;
    mmio->anomaly_flags = 0x00000000U;
    mmio_write_barrier();
}

bool shbt_evaluate_2pn_causal_interval(shbt_mmio_block_t *mmio,
                                       double dt, double dx, double dy,
                                       double dz, double vx, double vy,
                                       double vz) {
    struct timespec start_ts, end_ts;
    clock_gettime(CLOCK_MONOTONIC, &start_ts);

    double r = sqrt(dx * dx + dy * dy + dz * dz);
    double v2 = vx * vx + vy * vy + vz * vz;
    double beta2 = v2 / (C_LIGHT * C_LIGHT);

    if (beta2 >= 1.0) {
        mmio->anomaly_flags |= 0x00000001U;
        mmio_write_barrier();
        return false;
    }

    double gamma = 1.0 / sqrt(1.0 - beta2);
    mmio->target_vel_gamma = (uint32_t)(gamma * 65536.0);

    double gm_over_rc2 = (G_CONST * mmio->pn2_metric_m0) /
                         (C_LIGHT * C_LIGHT * r);
    double g00 = -(1.0 - 2.0 * gm_over_rc2
                 + 2.0 * gm_over_rc2 * gm_over_rc2
                 + (3.0 * G_CONST * mmio->pn2_metric_m0
                    * mmio->pn2_metric_j2 * R_SUN * R_SUN)
                       / (C_LIGHT * C_LIGHT * r * r * r));
    double g_ij = 1.0 + 2.0 * gm_over_rc2;

    double dr2 = dx * dx + dy * dy + dz * dz;
    double ds2_2pn = g00 * C_LIGHT * C_LIGHT * dt * dt + g_ij * dr2;

    int64_t ds2_fixed = (int64_t)(ds2_2pn * 1e-6);
    mmio->ds2_interval_lo = (uint32_t)(ds2_fixed & 0xFFFFFFFFLL);
    mmio->ds2_interval_hi = (int32_t)((ds2_fixed >> 32) & 0xFFFFFFFFLL);

    if (ds2_2pn > 0.0) {
        mmio->anomaly_flags |= 0x00000001U;
        mmio->causal_cone_hi |= 0x80000000U;
        mmio_write_barrier();

        clock_gettime(CLOCK_MONOTONIC, &end_ts);
        uint32_t elapsed_ns = (uint32_t)(
            (end_ts.tv_sec - start_ts.tv_sec) * 1000000000LL +
            (end_ts.tv_nsec - start_ts.tv_nsec));
        mmio->quench_time_ns = elapsed_ns;
        return false;
    }

    mmio_write_barrier();
    return true;
}
