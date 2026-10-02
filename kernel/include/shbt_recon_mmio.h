/*
 * @file shbt_recon_mmio.h
 * @brief 128-Byte Dual-Cacheline Hardware MMIO Register Contract
 * Physical Base Address: 0x70000000
 * ABI: C11 Freestanding (_Static_assert validated)
 *
 * Cacheline 0: control, dispatch FSM, 2PN causal, metric invariance.
 * Cacheline 1: isomer core, DEC bus, cryogenics, metrology, ECC, CRC.
 */

#ifndef SHBT_RECON_MMIO_H
#define SHBT_RECON_MMIO_H

#include <stdint.h>
#include <stddef.h>

#define SHBT_RECON_MMIO_BASE_ADDR (0x70000000U)

/* FSM Dispatch State Bitfields */
#define SHBT_STATE_STANDBY_STASIS     (0x01U)
#define SHBT_STATE_TRIGGER_ARMED      (0x02U)
#define SHBT_STATE_FOLDING_BURST      (0x04U)
#define SHBT_STATE_SYMPL_COOLDOWN     (0x08U)
#define SHBT_STATE_EMERGENCY_QUENCH   (0x10U)

/* 2PN Kinematic Flags */
#define SHBT_2PN_CAUSAL_AUTHORIZED   (1U << 0)
#define SHBT_2PN_LIGHTCONE_VIOLATION (1U << 1)
#define SHBT_2PN_JERK_PROFILE_VALID  (1U << 2)

/* PCSS Hardware Crowbar Status */
#define SHBT_PCSS_CROWBAR_ARMED      (1U << 0)
#define SHBT_PCSS_CROWBAR_TRIPPED    (1U << 1)
#define SHBT_PCSS_SMES_RECOVERY_ENG  (1U << 2)

#pragma pack(push, 1)

typedef struct {
    /* =========================================================================
     * CACHELINE 0: Control, Dispatch FSM, 2PN Causal, Metric Invariance (64 B)
     * Offset: 0x00 - 0x3F
     * ========================================================================= */
    volatile uint32_t system_control;         /* 0x00: System master control flags        */
    volatile uint8_t  dispatch_fsm_state;     /* 0x04: 5-Phase FSM state (0x01..0x10)     */
    volatile uint8_t  causal_2pn_flags;       /* 0x05: 2PN authorization & kinematic flags*/
    volatile uint8_t  pcss_crowbar_status;    /* 0x06: Fast optical crowbar bitfield      */
    volatile uint8_t  reserved_c0_0;          /* 0x07: Alignment padding                  */
    volatile uint64_t metric_det_error_fp64;  /* 0x08: |det(g) + 1| error residual (IEEE) */
    volatile uint64_t metric_shift_norm_fp64; /* 0x10: ||beta^i|| shift residual (m/s)    */
    volatile uint64_t dark_braid_counter;     /* 0x18: 124 Fibonacci braid step count     */
    volatile uint64_t active_bits_stepped;    /* 0x20: Cumulative boundary bits stepped   */
    volatile uint64_t target_nucleon_scale;   /* 0x28: Target N_local (10^23 .. 10^28)    */
    volatile uint32_t minimum_jerk_step_tau;  /* 0x30: s(tau) 5th-order jerk phase (Q32)  */
    volatile uint32_t wzw_framing_defect_raw; /* 0x34: Delta_fr residual (must be 0)      */
    volatile uint64_t reserved_c0_1;          /* 0x38: Reserved / Cacheline 0 pad         */

    /* =========================================================================
     * CACHELINE 1: Isomer Core, DEC Bus, Cryogenics, Metrology, ECC, CRC (64 B)
     * Offset: 0x40 - 0x7F
     * ========================================================================= */
    volatile uint32_t isomer_soc_millijoules; /* 0x40: Core SoC (mJ remaining / 500 TJ)   */
    volatile uint32_t dec_bus_voltage_mv;     /* 0x44: DEC output bus voltage (mV)        */
    volatile uint64_t gross_burst_power_mw;   /* 0x48: Instantaneous gross graser (mW)    */
    volatile uint64_t net_electrical_power_mw;/* 0x50: Net electrical output power (mW)   */
    volatile uint32_t cryo_temp_diamond_mk;   /* 0x58: CVD Diamond temp (mK, clamp 21130) */
    volatile uint32_t cryo_temp_mgb2_mk;      /* 0x5C: MgB2 rail temp (mK, max 32920)     */
    volatile uint16_t tmsv_squeezing_r_q12;   /* 0x60: TMSV squeezing r parameter (Q4.12) */
    volatile uint16_t tmsv_pointing_nrad;     /* 0x62: Wavefront error sigma_theta (nrad) */
    volatile uint32_t lanr_array_net_power_w; /* 0x64: LANR baseline net power (W)        */
    volatile uint32_t landauer_debt_power_w;  /* 0x68: Irreducible entropy debt (W)       */
    volatile uint16_t ecc_syndrome_hamming;   /* 0x6C: SECDED Hamming(72,64) syndrome     */
    volatile uint16_t ecc_double_error_flag;  /* 0x6E: SECDED Uncorrectable error count   */
    volatile uint32_t hardware_crc32c;        /* 0x70: Hardware CRC-32C across 0x00..0x6F */
    volatile uint8_t  reserved_c1_pad[12];    /* 0x74: Cacheline 1 terminating padding    */
} shbt_recon_mmio_t;

#pragma pack(pop)

/* Compile-time verification of C-ABI memory layout */
_Static_assert(sizeof(shbt_recon_mmio_t) == 128,
               "FATAL: shbt_recon_mmio_t layout must span exactly 128 bytes.");
_Static_assert(offsetof(shbt_recon_mmio_t, isomer_soc_millijoules) == 64,
               "FATAL: Cacheline 1 boundary misaligned; must start at offset 64.");
_Static_assert(offsetof(shbt_recon_mmio_t, hardware_crc32c) == 112,
               "FATAL: Hardware CRC32C field misaligned; must sit at offset 112.");

#endif /* SHBT_RECON_MMIO_H */
