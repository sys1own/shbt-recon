/*
 * shbt_recon_abi.h - SHBT-RECON unified C-ABI surface.
 *
 * Exports the #[repr(C, align(64))] telemetry structures and FFI
 * declarations shared between the C11 shbt-os microkernel
 * (kernel/src/*.c) and the Rust workspace crates.
 *
 * Freestanding-compatible: depends only on compiler-provided headers.
 */
#ifndef SHBT_RECON_ABI_H
#define SHBT_RECON_ABI_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --------------------------------------------------------------------------
 * SHBT-MMIO-1 normative register map — packed 56-byte 2PN causal engine
 * block at base 0x70000000
 * -------------------------------------------------------------------------- */
#define SHBT_MMIO_BASE              0x70000000U
#define SHBT_MMIO_ABI_VERSION       2U

#define SHBT_REG_CAUSAL_CONE_LO     0x00U  /* causal auth ctrl/status low (R/W) */
#define SHBT_REG_CAUSAL_CONE_HI     0x04U  /* bit31: trigger 2PN eval (R/W) */
#define SHBT_REG_PN2_METRIC_M0      0x08U  /* central mass M_sun (f64) (R/W) */
#define SHBT_REG_PN2_METRIC_J2      0x10U  /* quadrupole J2 (f64) (R/W) */
#define SHBT_REG_PN2_SPIN_VEC_X     0x18U  /* spin S_x (f32) (R/W) */
#define SHBT_REG_PN2_SPIN_VEC_Y     0x1CU  /* spin S_y (f32) (R/W) */
#define SHBT_REG_PN2_SPIN_VEC_Z     0x20U  /* spin S_z (f32) (R/W) */
#define SHBT_REG_TARGET_VEL_GAMMA   0x24U  /* gamma, 16.16 fixed point (R) */
#define SHBT_REG_DS2_INTERVAL_LO    0x28U  /* ds^2_2PN bits 31:0 (R) */
#define SHBT_REG_DS2_INTERVAL_HI    0x2CU  /* ds^2_2PN bits 63:32, signed (R) */
#define SHBT_REG_QUENCH_TIME_NS     0x30U  /* anomaly->quench latch ns (R) */
#define SHBT_REG_ANOMALY_FLAGS      0x34U  /* bit0 spacelike bit1 quench bit2 spin (R/W) */

/* --------------------------------------------------------------------------
 * TMSV metrology controller aperture — 0x7F001000
 * -------------------------------------------------------------------------- */
#define MMIO_TMSV_BASE              0x7F001000U
#define SHBT_REG_TMSV_CTRL          0x1000U /* bit0 pump, bit1 lock, 2-5 squeeze (R/W) */
#define SHBT_REG_TMSV_NOISE         0x1008U /* Q16.16 quadrature noise floor (R) */
#define SHBT_REG_METRIC_G00         0x1010U /* f64 g00 metric term (R/W) */
#define SHBT_REG_METRIC_DS2         0x1018U /* f64 ds^2_2PN result (R/W) */
#define SHBT_REG_INTERLOCK_STAT     0x1020U /* bit0 causal, bit31 trip (R) */

/* --------------------------------------------------------------------------
 * GST metamaterial self-healing array aperture
 * -------------------------------------------------------------------------- */
#define SHBT_REG_GST_ARRAY_CFG      0x2000U /* bits0-15 channel sel, bit31 en (R/W) */
#define SHBT_REG_GST_PULSE_GEN      0x2004U /* bits0-11 pulse ns, 12-31 fluence (R/W) */
#define SHBT_REG_GST_SENSE_SIG      0x2008U /* Q8.24 conductivity sense (R) */
#define SHBT_REG_GST_HEAL_STAT      0x200CU /* bit0 active, bit1 locked, bit2 err (R) */

/* --------------------------------------------------------------------------
 * UnifiedStinespringFrame SRAM arena (kernel/linker.ld): 2,112 bytes
 *   0x70000000 - 0x7000027F   640 B  Active Operational Window
 *   0x70000280 - 0x7000083F  1472 B  Dark Ledger Log Space
 * -------------------------------------------------------------------------- */
#define SHBT_FRAME_BASE             SHBT_MMIO_BASE
#define SHBT_FRAME_BYTES            2112U
#define SHBT_ACTIVE_WINDOW_BYTES    640U
#define SHBT_DARK_LEDGER_BYTES      1472U
#define SHBT_DARK_LEDGER_OFFSET     0x280U

/* --------------------------------------------------------------------------
 * SPSC zero-copy telemetry frame — #[repr(C, align(64))] cache-line layout
 * -------------------------------------------------------------------------- */
typedef struct __attribute__((aligned(64))) {
    uint64_t sequence_id;        /* 0x00 */
    uint64_t timestamp_fs;       /* 0x08 */
    double   metric_det;         /* 0x10 */
    double   gram_lambda_min;    /* 0x18 */
    float    kapitza_temp_mk;    /* 0x20 */
    float    power_output_kw;    /* 0x24 */
    uint32_t status_flags;       /* 0x28 */
    uint8_t  reserved_padding[12]; /* 0x2C */
    uint32_t frame_crc32;        /* 0x38 */
} ShbtTelemetryFrame;            /* 64 B incl. tail alignment padding */

_Static_assert(sizeof(ShbtTelemetryFrame)  == 64U,   "frame 64 B");
_Static_assert(offsetof(ShbtTelemetryFrame, frame_crc32) == 0x38U, "crc off");

/* --------------------------------------------------------------------------
 * FFI entry points (kernel/src/shbt_core_runtime.c, shbt_ecc_avx512.c)
 * -------------------------------------------------------------------------- */
uint8_t  shbt_ecc_encode(uint64_t data);
uint64_t shbt_ecc_decode_data(uint64_t data, uint8_t check_code,
                              uint8_t *flags);
int32_t  shbt_recover_on(void *hw);   /* ShbtRegisters * */
double   shbt_recover_bench(unsigned iters);
void     shbt_remap(double *col_a, double *col_b, double c, double s,
                    size_t n);
int      shbt_simd_shunt_check(const float *currents_a);
double   shbt_simd_shunt_bench(unsigned iters);

#ifdef __cplusplus
}
#endif

#endif /* SHBT_RECON_ABI_H */
