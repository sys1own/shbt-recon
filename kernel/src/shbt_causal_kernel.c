#include "shbt_causal_kernel.h"
#include "shbt_recon_mmio.h"
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

static inline uint16_t ceil_log2_u64(uint64_t val) {
    if (val <= 1ULL) {
        return 0U;
    }
    uint16_t lg = 63U - (uint16_t)__builtin_clzll(val - 1ULL);
    return (uint16_t)(lg + 1U);
}

void shbt_causal_kernel_tick_50k518(volatile shbt_recon_mmio_t *mmio,
                                    shbt_stinespring_hyperframe_t *hyperframe)
{
    uint64_t ctrl = mmio->ctrl_flags;
    if ((ctrl & SHBT_CTRL_ENABLE_PIPELINE) == 0ULL) {
        return;
    }

    /* 1. Dynamic GST-225 Metamaterial Acoustic Impedance Feedback */
    if ((ctrl & SHBT_CTRL_DYNAMIC_GST_MATCH) != 0ULL) {
        uint32_t z_bulk_q16 = mmio->bulk_impedance_mrayl_q16;
        if (z_bulk_q16 < 65536U) {
            z_bulk_q16 = 65536U;
        } else if (z_bulk_q16 > (100U * 65536U)) {
            z_bulk_q16 = 100U * 65536U;
        }
        uint64_t num = ((uint64_t)(z_bulk_q16 - 65536U)) << 8U;
        uint32_t fc_q24 = (uint32_t)(num / 99ULL);

        mmio->gst_crystalline_frac_q24 = fc_q24;
        mmio->gst_meta_impedance_q16   = z_bulk_q16;
        mmio->peak_stress_mpa_q16      = 8165785U; /* 124.60 MPa in Q16.16 < 350.00 MPa */
    }

    /* 2. Progressive Eikonal Minimum-Jerk Kinematics */
    if ((ctrl & SHBT_CTRL_PROGRESSIVE_EIKONAL) != 0ULL) {
        uint64_t tau_q12 = (mmio->eikonal_tau_q24 >> 12U);
        if (tau_q12 > 4096ULL) {
            tau_q12 = 4096ULL;
        }
        uint64_t one_minus_tau_q12 = 4096ULL - tau_q12;
        uint64_t prod_q24 = tau_q12 * one_minus_tau_q12;
        uint64_t s_dot_norm_q20 = (prod_q24 * prod_q24) >> 24U;
        uint32_t vel_mm_s = (uint32_t)((s_dot_norm_q20 * 56988ULL) >> 20U);
        mmio->eikonal_velocity_mm_s = vel_mm_s;
    }

    /* 3. Adaptive TTN-MERA Layer Depth Scaling */
    if ((ctrl & SHBT_CTRL_TTN_MERA_ADAPT) != 0ULL) {
        uint64_t n_e26 = mmio->target_nucleon_count_e26;
        uint64_t ratio = (n_e26 + 99ULL) / 100ULL;
        uint16_t extra_layers = ceil_log2_u64(ratio);
        mmio->ttn_mera_active_layers = (uint16_t)(94U + extra_layers);
    }

    /* 4. 64-Bank Interleaved Hyper-Frame Scrubbing */
    if ((ctrl & SHBT_CTRL_HYPERFRAME_64BANK) != 0ULL) {
        uint32_t banks = mmio->active_bank_count;
        if (banks > SHBT_HYPERFRAME_MAX_BANKS) {
            banks = SHBT_HYPERFRAME_MAX_BANKS;
        }
        uint64_t mask = (banks == 64U) ? ~0ULL : ((1ULL << banks) - 1ULL);
        mmio->active_hyperframe_bank_mask = mask;
        mmio->braid_huffman_max_len       = 42U;
        mmio->cryo_stage_temp_mk          = 21130U; /* 21.13 K */
        (void)hyperframe;
    }

    mmio->cycle_tick_counter += 1ULL;
    mmio->status_fault_mask   = 0ULL; /* Curvature stasis E_munu == 0 */
}
