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
#include "shbt_ibr312_mmio.h"

#define SHBT_RECON_MMIO_BASE_ADDR (0x70000000U)

/* FSM Dispatch State Bitfields */
#define SHBT_STATE_STANDBY_STASIS     (0x01U)
#define SHBT_STATE_TRIGGER_ARMED      (0x02U)
#define SHBT_STATE_FOLDING_BURST      (0x04U)
/* Dual-mode synthesis dispatch shares the 0x04 burst phase code. */
#define SHBT_STATE_TRANSMUTATION_BURST (0x04U)
#define SHBT_STATE_SYMPL_COOLDOWN     (0x08U)
/* SYMPLECTIC_CRYSTALLIZATION re-render phase (synthesis mode alias). */
#define SHBT_STATE_SYMPLECTIC_CRYSTALLIZATION (0x08U)
#define SHBT_STATE_EMERGENCY_QUENCH   (0x10U)

/* Bipartite Coset Spinor Folding Extension State (0x06) */
#define SHBT_SYNTH_STATE_COSET_ANTIBARYON_FOLD (0x06U)

/* Synthesis hardware status bits (synth bank @ 0x30) */
#define SYNTH_STATUS_IDLE             (0x00000000U)
#define SYNTH_STATUS_DYNKIN_LOCKED    (0x00000001U)
#define SYNTH_STATUS_FRAMING_CLEAN    (0x00000002U)
#define SYNTH_STATUS_DARK_SINK_ACTIVE (0x00000004U)
#define SYNTH_STATUS_DEC_SYNCHRONIZED (0x00000008U)
#define SYNTH_STATUS_CRYO_HEADROOM_OK (0x00000010U)
#define SHBT_SYNTH_RG_LOCK_312        (1U << 16)
#define SHBT_SYNTH_CBARB_ZERO_VERIFIED (1U << 17)
#define SHBT_SYNTH_BRAID_PARITY_OK    (1U << 18)
#define SHBT_SYNTH_FRAMING_FAULT      (1U << 31)
#define SYNTH_STATUS_ERROR_ANOMALY    (0x80000000U)

/* system_control bank-select bit: 0 = legacy translocator bank,
 * 1 = synthesis bank overlays Cacheline 0 offsets 0x28..0x3F. */
#define SHBT_CTRL_SYNTH_BANK_SEL      (1U << 5)

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
    /* Bank-switched register window 0x28-0x3F.  When
     * SHBT_CTRL_SYNTH_BANK_SEL is clear the window exposes the legacy
     * translocator registers; when set it exposes the synthesis target
     * specification written before a TRANSMUTATION_BURST (0x04) dispatch. */
    union {
        struct {
            volatile uint64_t target_nucleon_scale;   /* 0x28: Target N_local (10^23..10^28) */
            volatile uint32_t minimum_jerk_step_tau;  /* 0x30: s(tau) 5th-order jerk (Q32)   */
            volatile uint32_t wzw_framing_defect_raw; /* 0x34: Delta_fr residual (must be 0) */
            volatile uint64_t reserved_c0_1;          /* 0x38: Cacheline 0 pad               */
        } translocator;
        struct {
            volatile uint32_t synth_target_z;         /* 0x28: Target atomic number Z        */
            volatile uint32_t synth_target_a;         /* 0x2C: Target nucleon number A       */
            volatile uint32_t synth_status;           /* 0x30: SYNTH_STATUS_* telemetry      */
            volatile int32_t  synth_enthalpy_delta_mv;/* 0x34: formation dH (mJ/mol)         */
            volatile int32_t  synth_binding_offset_q32;/* 0x38: nuclear dB (MeV, Q32)        */
            volatile uint16_t synth_antibaryon_ratio_q16; /* 0x3C: anti-baryon fraction Q16 (0x8000 = 0.500) */
            volatile uint16_t synth_coset_weight_q16;     /* 0x3E: coset conformal weight Q16 (0xB400 = 45/64) */
        } synth;
    } bank0;

/* SHBT-MAR-312 Primary Power Plant & 5-Stage Relativistic DEC Constants */
#define SHBT_MAR312_TOTAL_ENERGY_EJ         (33.8822)
#define SHBT_MAR312_NET_ENERGY_EJ           (30.3310)
#define SHBT_MAR312_CORE_MASS_KG            (376.99)
#define SHBT_MAR312_DEC_EFFICIENCY_PPM      (895190U)  /* 89.519% */
#define SHBT_MAR312_DEC_BUS_MIN_MV          (15000000U) /* 15 kV */
#define SHBT_MAR312_DEC_BUS_MAX_MV          (2700000000U) /* 2.700 MV */
#define SHBT_MAR312_CRUISE_NET_MW           (895190000000000ULL)    /* 895.19 TW */
#define SHBT_MAR312_BURST_NET_MW            (6805000000000000ULL)   /* 6.805 PW */
#define SHBT_MAR312_IMPULSE_NET_MW          (895190000000000000ULL) /* 895.19 PW */

/* SHBT-MAR-312C Compacted Seed Wafer & Active Symplectic Nacelle Constants */
#define SHBT_MAR312C_COMPACTED_NACELLE_MASS_KG (2257.70)
#define SHBT_MAR312C_COMPACTED_NACELLE_VOL_M3  (0.8589)
#define SHBT_MAR312C_SEED_WAFER_MASS_KG        (37.699)
#define SHBT_MAR312C_SEED_WAFER_VOL_M3         (0.00359)
#define SHBT_MAR312C_SEED_GROSS_EJ             (3.38822)
#define SHBT_MAR312C_SEED_NET_EJ               (3.03310)
#define SHBT_MAR312C_SYMPLECTIC_SHIELD_MASS_KG (384.88)
#define SHBT_MAR312C_SYMPLECTIC_SHIELD_VOL_M3  (0.0850)
#define SHBT_MAR312C_SYMPLECTIC_S_FACTOR       (100.0 / 1089.0)
#define SHBT_MAR312C_REBCO_SHEATH_TESLA        (18.5)
#define SHBT_MAR312C_SINGLE_FRAME_LIMIT_KG     (16600.00)
#define SHBT_MAR312C_SINGLE_FRAME_MARGIN_KG    (14342.30)
#define SHBT_MAR312C_TIER1_MAX_CORES           (38U)

    /* =========================================================================
     * CACHELINE 1: SHBT-MAR-312 Reactor, 5-Stage DEC, Cryo, Metrology, ECC (64 B)
     * Offset: 0x40 - 0x7F
     * ========================================================================= */
    volatile uint32_t isomer_soc_millijoules; /* 0x40: MAR-312 Core SoC (mJ rem / 33.8822 EJ gross / 30.3310 EJ net) */
    volatile uint32_t dec_bus_voltage_mv;     /* 0x44: 5-Stage DEC bus voltage (mV, 15 kV - 2.700 MV DC)   */
    volatile uint64_t gross_burst_power_mw;   /* 0x48: Gross annihilation power (mW, 1.0 PW Cruise / 7.6 PW Burst / 1.0 EW Imp) */
    volatile uint64_t net_electrical_power_mw;/* 0x50: Net electrical output power (mW, 895.19 TW / 6.805 PW / 895.19 PW) */
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
_Static_assert(offsetof(shbt_recon_mmio_t, bank0) == 0x28,
               "FATAL: bank-switched window must begin at offset 0x28.");
_Static_assert(offsetof(shbt_recon_mmio_t, bank0.synth.synth_target_z) == 0x28,
               "FATAL: synth_target_z offset mismatch in Cacheline 0.");
_Static_assert(offsetof(shbt_recon_mmio_t, bank0.synth.synth_binding_offset_q32) == 0x38,
               "FATAL: synth_binding_offset_q32 offset mismatch in Cacheline 0.");
_Static_assert(offsetof(shbt_recon_mmio_t, bank0.synth.synth_antibaryon_ratio_q16) == 0x3C,
               "FATAL: synth_antibaryon_ratio_q16 offset mismatch in Cacheline 0.");
_Static_assert(offsetof(shbt_recon_mmio_t, bank0.synth.synth_coset_weight_q16) == 0x3E,
               "FATAL: synth_coset_weight_q16 offset mismatch in Cacheline 0.");

#endif /* SHBT_RECON_MMIO_H */
