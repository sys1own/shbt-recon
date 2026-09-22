/* shbt_tmsv_kernel.h — TMSV phase driver & 2PN causal interlock.
 * MMIO aperture 0x7F001000: TMSV_CTRL/NOISE, METRIC_G00/DS2, INTERLOCK_STAT.
 */
#ifndef SHBT_TMSV_KERNEL_H
#define SHBT_TMSV_KERNEL_H

#include <stdint.h>
#include <stdbool.h>

#define MMIO_TMSV_BASE        0x7F001000U
#define TMSV_CTRL_REG_OFF     0x00U   /* bit0 pump en, bit1 phase lock, 2-5 squeeze */
#define TMSV_NOISE_REG_OFF    0x08U   /* Q16.16 quadrature noise floor (R) */
#define METRIC_G00_REG_OFF    0x10U   /* f64 g00 metric term (R/W) */
#define METRIC_DS2_REG_OFF    0x18U   /* f64 ds^2_2PN result (R/W) */
#define INTERLOCK_STAT_OFF    0x20U   /* bit0 causal, bit31 trip (R) */

typedef struct __attribute__((aligned(64))) {
    double    squeezing_r;        /* 0x00: squeezing parameter r */
    double    attenuation_db;     /* 0x08: measured noise suppression dB */
    double    displacement_sd;    /* 0x10: S_r^{1/2} (pm/sqrt Hz) */
    double    r_3sigma_bound;     /* 0x18: displacement bound (nm) */
    double    metric_g00_g0i[4];  /* 0x20: g00, g01, g02, g03 */
    double    metric_gij_diag[4]; /* 0x40: g11, g22, g33, ds^2_2PN */
    uint32_t  interlock_status;   /* 0x60: 0x1 causal / 0x80000000 trip */
    uint8_t   reserved_pad[28];   /* 0x64: zero-pad to 128 B */
} tmsv_interlock_state_t;

_Static_assert(sizeof(tmsv_interlock_state_t) == 128, "tmsv state 128B");
_Static_assert(_Alignof(tmsv_interlock_state_t) == 64, "tmsv state 64B aligned");

bool verify_tmsv_causal_interlock(tmsv_interlock_state_t *state,
                                  double dt, double dx[3]);

#endif /* SHBT_TMSV_KERNEL_H */
