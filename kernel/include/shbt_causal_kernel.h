/* shbt_causal_kernel.h — SHBT-MMIO-1 2PN causal authorization engine.
 * 56-byte register block at physical base 0x70000000.
 */
#ifndef SHBT_CAUSAL_KERNEL_H
#define SHBT_CAUSAL_KERNEL_H

#include <stdint.h>
#include <stdbool.h>

#define SHBT_MMIO_BASE_ADDR    0x70000000U
#define CAUSAL_CONE_LO_OFF     0x00U
#define CAUSAL_CONE_HI_OFF     0x04U
#define PN2_METRIC_M0_OFF      0x08U
#define PN2_METRIC_J2_OFF      0x10U
#define PN2_SPIN_VEC_X_OFF     0x18U
#define PN2_SPIN_VEC_Y_OFF     0x1CU
#define PN2_SPIN_VEC_Z_OFF     0x20U
#define TARGET_VEL_GAMMA_OFF   0x24U
#define DS2_INTERVAL_LO_OFF    0x28U
#define DS2_INTERVAL_HI_OFF    0x2CU
#define QUENCH_TIME_NS_OFF     0x30U
#define ANOMALY_FLAGS_OFF      0x34U

#define G_CONST   6.67430e-11
#define C_LIGHT   299792458.0
#define M_SUN     1.98847e30
#define J2_SUN    2.20e-7
#define R_SUN     6.96342e8

typedef struct {
    volatile uint32_t causal_cone_lo;
    volatile uint32_t causal_cone_hi;
    volatile double   pn2_metric_m0;
    volatile double   pn2_metric_j2;
    volatile float    pn2_spin_vec[3];
    volatile uint32_t target_vel_gamma;
    volatile uint32_t ds2_interval_lo;
    volatile int32_t  ds2_interval_hi;
    volatile uint32_t quench_time_ns;
    volatile uint32_t anomaly_flags;
} __attribute__((packed, aligned(4))) shbt_mmio_block_t;

void shbt_causal_init(shbt_mmio_block_t *mmio);
bool shbt_evaluate_2pn_causal_interval(shbt_mmio_block_t *mmio,
                                       double dt, double dx, double dy,
                                       double dz, double vx, double vy,
                                       double vz);

#endif /* SHBT_CAUSAL_KERNEL_H */
