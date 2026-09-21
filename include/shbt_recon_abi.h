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
 * SHBT-MMIO-1 normative register map — 14 x uint32_t, 56 bytes at 0x70000000
 * -------------------------------------------------------------------------- */
#define SHBT_MMIO_BASE              0x70000000U
#define SHBT_MMIO_ABI_VERSION       1U

#define SHBT_REG_CTRL_REG           0x00U  /* master enable / soft reset (R/W) */
#define SHBT_REG_STATUS_REG         0x04U  /* lock / metric stability (R) */
#define SHBT_REG_METRIC_DET_L       0x08U  /* |det g| low word (R) */
#define SHBT_REG_METRIC_DET_H       0x0CU  /* |det g| high word (R) */
#define SHBT_REG_GRAM_LAMBDA_MIN    0x10U  /* min Gram eigenvalue (R) */
#define SHBT_REG_BETA_SHIFT_MAG     0x14U  /* |beta^i| nullification (R/W) */
#define SHBT_REG_KAPITZA_TEMP       0x18U  /* mixing-chamber temp (R) */
#define SHBT_REG_COOLING_PWR        0x1CU  /* cooling offset P_cool (R/W) */
#define SHBT_REG_ECC_ERR_CNT        0x20U  /* corrected ECC count (R) */
#define SHBT_REG_AVX_REMAP_ID       0x24U  /* Givens remap index (R/W) */
#define SHBT_REG_RECOVERY_STAGE     0x28U  /* quench recovery stage 0..4 (R) */
#define SHBT_REG_LANR_PWR_OUT       0x2CU  /* LANR net output (R) */
#define SHBT_REG_TEL_HEAD_PTR       0x30U  /* SPSC ring head (R/W) */
#define SHBT_REG_TEL_TAIL_PTR       0x34U  /* SPSC ring tail (R/W) */

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
