/* shbt_tmsv_kernel.c — TMSV phase & 2PN causal interlock evaluation.
 * Bare-metal builds write the interlock result to the MMIO aperture;
 * hosted builds latch it into the state block only.
 */
#include "shbt_tmsv_kernel.h"
#include <math.h>

#ifdef SHBT_BARE_METAL
#define REG_INTERLOCK  (*(volatile uint32_t *)(MMIO_TMSV_BASE + INTERLOCK_STAT_OFF))
#else
static uint32_t host_interlock_reg;
#define REG_INTERLOCK  (host_interlock_reg)
#endif

bool verify_tmsv_causal_interlock(tmsv_interlock_state_t * const state,
                                  double dt, double dx[3]) {
    state->attenuation_db = 20.0 * log10(exp(state->squeezing_r));

    const double c = 299792458.0;
    double ds2 = state->metric_g00_g0i[0] * (c * dt) * (c * dt);
    for (int i = 0; i < 3; i++) {
        ds2 += 2.0 * state->metric_g00_g0i[i + 1] * (c * dt) * dx[i];
        ds2 += state->metric_gij_diag[i] * dx[i] * dx[i];
    }
    state->metric_gij_diag[3] = ds2;

    if (ds2 > 0.0) {
        state->interlock_status = 0x80000000U;
        REG_INTERLOCK = state->interlock_status;
        return false;
    }
    state->interlock_status = 0x00000001U;
    REG_INTERLOCK = state->interlock_status;
    return true;
}
