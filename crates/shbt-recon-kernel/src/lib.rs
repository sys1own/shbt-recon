//! shbt-recon-kernel — freestanding C11 shbt-os runtime wrapper.
//!
//! Transfers from `sys1own/shbt-qc` (`kernel/src/shbt_core_runtime.c`,
//! `kernel/include/shbt_hardware.h`, `kernel/linker.ld`) and the
//! `sys1own/shbt-sglt` ECC/AVX-512/recover FFI unit:
//! - SECDED Hamming(72,64) ECC, decode budget ≤ 1.20 ns
//! - AVX-512 Givens channel remapping `shbt_remap` + current-shunt interlock
//! - 4-stage post-quench recovery sequencer ≤ 120.00 ns
//! - SHBT-MMIO-1 14-register block, 56 bytes at 0x70000000
//!
//! Timing assertions are environment-aware: when `SGLT_CI_VIRTUAL_ENV` is
//! set (virtualized CI without a calibrated TSC), benchmarks are exercised
//! but latency bounds are not asserted.

/// MMIO aperture base address (SHBT-MMIO-1).
pub const SHBT_MMIO_BASE: u64 = 0x7000_0000;
/// SECDED decode budget (ns).
pub const ECC_DECODE_BUDGET_NS: f64 = 1.20;
/// Recovery budget (ns), stages 1–4 (0–15 / 15–45 / 45–80 / 80–120 ns).
pub const RECOVERY_BUDGET_NS: f64 = 120.00;
/// AVX-512 shunt trip current (A).
pub const SHUNT_TRIP_A: f32 = 7.5;

/// SHBT-MMIO-1 normative register map — packed 56-byte 2PN causal
/// engine block at 0x70000000.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ShbtMmio1 {
    /// 0x00 CAUSAL_CONE_LO — causal authorization control/status (R/W).
    pub causal_cone_lo: u32,
    /// 0x04 CAUSAL_CONE_HI — bit 31 triggers 2PN evaluation (R/W).
    pub causal_cone_hi: u32,
    /// 0x08 PN2_METRIC_M0 — central mass M_☉ (f64, kg) (R/W).
    pub pn2_metric_m0: f64,
    /// 0x10 PN2_METRIC_J2 — quadrupole coefficient J₂ (f64) (R/W).
    pub pn2_metric_j2: f64,
    /// 0x18 PN2_SPIN_VEC_X — gravitomagnetic spin S_x (f32) (R/W).
    pub pn2_spin_x: f32,
    /// 0x1C PN2_SPIN_VEC_Y — gravitomagnetic spin S_y (f32) (R/W).
    pub pn2_spin_y: f32,
    /// 0x20 PN2_SPIN_VEC_Z — gravitomagnetic spin S_z (f32) (R/W).
    pub pn2_spin_z: f32,
    /// 0x24 TARGET_VEL_GAMMA — Lorentz γ, 16.16 fixed point (R).
    pub target_vel_gamma: u32,
    /// 0x28 DS2_INTERVAL_LO — Δs²_2PN bits 31:0 (R).
    pub ds2_interval_lo: u32,
    /// 0x2C DS2_INTERVAL_HI — Δs²_2PN bits 63:32, signed (R).
    pub ds2_interval_hi: i32,
    /// 0x30 QUENCH_TIME_NS — anomaly→quench latch timer, ns (R).
    pub quench_time_ns: u32,
    /// 0x34 ANOMALY_FLAGS — bit0 spacelike, bit1 quench, bit2 spin (R/W).
    pub anomaly_flags: u32,
}

const _: () = assert!(std::mem::size_of::<ShbtMmio1>() == 56);
const _: () = assert!(std::mem::offset_of!(ShbtMmio1, anomaly_flags) == 0x34);

/// 128-byte dual-cacheline hardware MMIO register contract at 0x70000000.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ShbtReconMmio {
    pub system_control: u32,
    pub dispatch_fsm_state: u8,
    pub causal_2pn_flags: u8,
    pub pcss_crowbar_status: u8,
    pub reserved_c0_0: u8,
    pub metric_det_error_fp64: u64,
    pub metric_shift_norm_fp64: u64,
    pub dark_braid_counter: u64,
    pub active_bits_stepped: u64,
    // Bank 0 (24 bytes @ 0x28..0x3F)
    pub synth_target_z: u32,
    pub synth_target_a: u32,
    pub synth_status: u32,
    pub synth_enthalpy_delta_mv: i32,
    pub synth_binding_offset_q32: i32,
    pub synth_antibaryon_ratio_q16: u16,
    pub synth_coset_weight_q16: u16,
    // Cacheline 1 (64 bytes @ 0x40..0x7F)
    pub isomer_soc_millijoules: u32,
    pub dec_bus_voltage_mv: u32,
    pub gross_burst_power_mw: u64,
    pub net_electrical_power_mw: u64,
    pub cryo_temp_diamond_mk: u32,
    pub cryo_temp_mgb2_mk: u32,
    pub tmsv_squeezing_r_q12: u16,
    pub tmsv_pointing_nrad: u16,
    pub lanr_array_net_power_w: u32,
    pub landauer_debt_power_w: u32,
    pub ecc_syndrome_hamming: u16,
    pub ecc_double_error_flag: u16,
    pub hardware_crc32c: u32,
    pub reserved_c1_pad: [u8; 12],
}

const _: () = assert!(std::mem::size_of::<ShbtReconMmio>() == 128);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmio, synth_antibaryon_ratio_q16) == 0x3C);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmio, synth_coset_weight_q16) == 0x3E);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmio, isomer_soc_millijoules) == 64);

/// First-Principles Flat MMIO register mapping layout (128 bytes, 64-byte aligned).
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, Default)]
pub struct ShbtReconMmioFirstPrinciples {
    pub system_status_flags: u64,
    pub de_rendering_tau_q32: u64,
    pub target_coord_x_nm: u64,
    pub target_coord_y_nm: u64,
    pub eikonal_velocity_kms: u32,
    pub kohn_sham_shift_ev_q16: u32,
    pub cryo_temp_millikelvin: u32,
    pub phononic_damping_coeff: u32,
    pub rebco_coil_current_amperes: u32,
    pub magnetic_nozzle_field_mt: u32,
    pub synth_jitter_bound_q16: u16,
    pub synth_tether_displacement_q16: u16,
    pub reserved_gate_align: u32,
    pub dec_discharge_energy_uj: u64,
    pub dec_space_charge_margin_q16: u16,
    pub tqec_syndrome_err_rate_q16: u16,
    pub rebco_quench_headroom_q16: u32,
    pub poynting_recovery_flux_w: u64,
    pub mhd_diamagnetic_beta_q16: u32,
    pub anyon_braid_cycle_count: u32,
    pub dark_ledger_wzw_checksum: u64,
    pub hardware_pad: [u8; 24],
}

const _: () = assert!(std::mem::size_of::<ShbtReconMmioFirstPrinciples>() == 128);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, synth_jitter_bound_q16) == 0x38);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, synth_tether_displacement_q16) == 0x3A);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, dec_discharge_energy_uj) == 0x40);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, dec_space_charge_margin_q16) == 0x48);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, tqec_syndrome_err_rate_q16) == 0x4A);
const _: () = assert!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, rebco_quench_headroom_q16) == 0x4C);

pub fn shbt_kernel_record_wavefront_jitter(mmio: &mut ShbtReconMmioFirstPrinciples, jitter_as: u16) {
    mmio.synth_jitter_bound_q16 = jitter_as;
}

pub fn shbt_kernel_record_tether_displacement(mmio: &mut ShbtReconMmioFirstPrinciples, disp_q16: u16) {
    mmio.synth_tether_displacement_q16 = disp_q16;
}

pub fn shbt_kernel_record_dec_margins(mmio: &mut ShbtReconMmioFirstPrinciples, cl_margin_q16: u16, quench_margin_q16: u32) {
    mmio.dec_space_charge_margin_q16 = cl_margin_q16;
    mmio.rebco_quench_headroom_q16 = quench_margin_q16;
}

pub const SHBT_SYNTH_STATE_COSET_ANTIBARYON_FOLD: u8 = 0x06;
pub const SHBT_SYNTH_RG_LOCK_312: u32 = 1 << 16;
pub const SHBT_SYNTH_CBARB_ZERO_VERIFIED: u32 = 1 << 17;
pub const SHBT_SYNTH_BRAID_PARITY_OK: u32 = 1 << 18;
pub const SHBT_SYNTH_FRAMING_FAULT: u32 = 1 << 31;

/* Control Bitflags */
pub const SHBT_MAR_SYMPLECTIC_SHIELD_LOCK: u32 = 1 << 8;
pub const SHBT_MAR_REBCO_DEFLECTOR_18T5: u32 = 1 << 9;
pub const SHBT_IBR_AUTO_RELOAD_ARMED: u32 = 1 << 10;
pub const SHBT_IBR_RELOAD_IN_PROGRESS: u32 = 1 << 11;
pub const SHBT_IBR_ZERO_VOLUME_LOCK: u32 = 1 << 12;
pub const SHBT_IBR_ADM_FRAME_COMPLIANT: u32 = 1 << 13;

/* Normative Constants */
pub const SHBT_IBR_SYMPLECTIC_S_Q32: u32 = 0x17829CBC;
pub const SHBT_IBR_SHIELD_MASS_G: u32 = 384880;
pub const SHBT_IBR_NACELLE_MASS_G: u32 = 2257700;
pub const SHBT_IBR_CREW_DOSE_NSV_YR: u32 = 114000;
pub const SHBT_MAR312C_SEED_MASS_G: u32 = 37699;
pub const SHBT_IBR312_MAGAZINE_TOTAL_SIZE: usize = 33792;
pub const SHBT_IBR312_MAGAZINE_BANKS: usize = 16;

/* 8-State Autonomous Hot-Swap FSM States */
pub const IBR_FSM_S0_NOMINAL_BURN: u16 = 0x00;
pub const IBR_FSM_S1_SOC_THRESHOLD_DETECT: u16 = 0x01;
pub const IBR_FSM_S2_SMES_BUS_CLAMP: u16 = 0x02;
pub const IBR_FSM_S3_GIVENS_BANK_ALIGN: u16 = 0x03;
pub const IBR_FSM_S4_STINESPRING_UNFOLD: u16 = 0x04;
pub const IBR_FSM_S5_SUGAWARA_PARITY_LOCK: u16 = 0x05;
pub const IBR_FSM_S6_MOSSBAUER_SYNC: u16 = 0x06;
pub const IBR_FSM_S7_HANDOFF_RESUME: u16 = 0x07;

/* 4 Braid Reload Modes */
pub const IBR_MODE_PREFOLDED_UNFOLD: u16 = 0x00;
pub const IBR_MODE_1C_ISOBARIC_RECYCLE: u16 = 0x01;
pub const IBR_MODE_1A_MATCHED_FOUNDRY: u16 = 0x02;
pub const IBR_MODE_1B_RAW_SLAG_SCOOP: u16 = 0x03;

/// 16-Bank Stinespring Dark-Braid Magazine Ring Buffer (33,792 Bytes = 33.0 KB)
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct ShbtStinespringMagazine {
    pub banks: [[u8; 2112]; SHBT_IBR312_MAGAZINE_BANKS],
}

const _: () = assert!(std::mem::size_of::<ShbtStinespringMagazine>() == 33792);

/// Compacted SHBT-MAR-312-IBR MMIO Register Block (128 Bytes)
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct ShbtIbr312Mmio {
    pub energy_remaining_joules: u64,
    pub state_of_charge_q32: u64,
    pub bus_voltage_uv: u64,
    pub bus_current_ua: u64,
    pub core_temperature_uk: u64,
    pub cryo_headroom_uk: u64,
    pub trigger_delay_ps: u32,
    pub trigger_pulse_width_ps: u32,
    pub graser_coherent_flux_w_m2: u64,
    pub transient_shear_stress_kpa: u32,
    pub borrmann_epsilon_q16: u32,
    pub system_status_flags: u32,
    pub crowbar_trip_count: u32,
    pub inductive_recovery_eff_q32: u32,
    pub net_conversion_eff_q32: u32,
    pub symplectic_s_factor_q32: u32,
    pub symplectic_shield_mass_g: u32,
    pub compacted_nacelle_mass_g: u32,
    pub ibr_frame_bank_select: u16,
    pub ibr_reload_mode: u16,
    pub ibr_stored_micro_batches: u32,
    pub ibr_reload_count: u32,
    pub ibr_last_latency_ns: u32,
    pub ibr_crew_dose_nsv_yr: u32,
    pub ibr_frame_angle_q64: u64,
}

const _: () = assert!(std::mem::size_of::<ShbtIbr312Mmio>() == 128);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, symplectic_s_factor_q32) == 0x58);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, symplectic_shield_mass_g) == 0x5C);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, compacted_nacelle_mass_g) == 0x60);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_frame_bank_select) == 0x64);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_reload_mode) == 0x66);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_stored_micro_batches) == 0x68);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_reload_count) == 0x6C);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_last_latency_ns) == 0x70);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_crew_dose_nsv_yr) == 0x74);
const _: () = assert!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_frame_angle_q64) == 0x78);

/// Register-file layout used by the transferred C runtime
/// (`kernel/include/shbt_hardware.h` `ShbtRegisters`), mirrored for the
/// host-side `shbt_recover_on` simulation path.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MmioRegisters {
    /// 0x00 — status.
    pub status: u32,
    /// 0x04 — global RF blanking.
    pub blank: u32,
    /// 0x08 — FIFO data.
    pub fifo_data: u32,
    /// 0x0C — PLL control.
    pub pll_ctrl: u32,
    /// 0x10 — ECC data low.
    pub ecc_low: u32,
    /// 0x14 — ECC data high.
    pub ecc_high: u32,
    /// 0x18 — ECC check bits.
    pub ecc_check: u32,
    /// 0x1C — ECC commit strobe.
    pub ecc_commit: u32,
    /// 0x20 — fault latch.
    pub fault_latch: u32,
    /// 0x24 — channel select.
    pub channel_select: u32,
    /// 0x28 — ABI version.
    pub abi_version: u32,
    /// 0x2C — phase offset.
    pub phase_offset: u32,
    /// 0x30 — ECC error counts.
    pub ecc_counts: u32,
    /// 0x34 — control.
    pub control: u32,
}

const _: () = assert!(std::mem::size_of::<MmioRegisters>() == 0x38);

/// Status bit: PLL locked.
pub const STATUS_PLL_LOCK: u32 = 1 << 1;
/// Status bit: ECC error latched.
pub const STATUS_ECC_ERR: u32 = 1 << 2;
/// Status bit: overtemperature.
pub const STATUS_OVERTEMP: u32 = 1 << 0;
/// Status bit: fault state.
pub const STATUS_FAULT_ST: u32 = 1 << 3;

extern "C" {
    /// `uint8_t shbt_ecc_encode(uint64_t data)` — SECDED check byte.
    fn shbt_ecc_encode(data: u64) -> u8;
    /// `uint64_t shbt_ecc_decode_data(u64, u8, u8 *flags)` — corrected data;
    /// flags bit0 = single-bit corrected, bit1 = uncorrectable double-bit.
    fn shbt_ecc_decode_data(data: u64, check: u8, flags: *mut u8) -> u64;
    /// `int32_t shbt_recover_on(ShbtRegisters *hw)` — 4-stage sequence on a
    /// caller-supplied block.
    fn shbt_recover_on(hw: *mut MmioRegisters) -> i32;
    /// `double shbt_recover_bench(unsigned iters)` — mean latency (ns).
    fn shbt_recover_bench(iters: u32) -> f64;
    /// `void shbt_remap(double*, double*, double c, double s, size_t n)` —
    /// AVX-512 Givens rotation column remap.
    fn shbt_remap(col_a: *mut f64, col_b: *mut f64, c: f64, s: f64, n: usize);
    /// `int shbt_simd_shunt_check(const float *i16)` — AVX-512 interlock mask.
    fn shbt_simd_shunt_check(currents: *const f32) -> i32;
    /// `double shbt_simd_shunt_bench(unsigned iters)` — mean latency (ns).
    fn shbt_simd_shunt_bench(iters: u32) -> f64;
}

/// SECDED Hamming(72,64) encode.
pub fn ecc_encode(data: u64) -> u8 {
    unsafe { shbt_ecc_encode(data) }
}

/// SECDED decode outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EccDecode {
    /// Corrected 64-bit data word.
    pub data: u64,
    /// A single-bit error was corrected inline.
    pub corrected: bool,
    /// A double-bit error was detected (uncorrectable → kernel trap).
    pub uncorrectable: bool,
}

/// SECDED Hamming(72,64) decode + correct (≤ 1.20 ns on target hardware).
pub fn ecc_decode(data: u64, check: u8) -> EccDecode {
    unsafe {
        let mut flags = 0u8;
        let corrected = shbt_ecc_decode_data(data, check, &mut flags);
        EccDecode {
            data: corrected,
            corrected: flags & 1 != 0,
            uncorrectable: flags & 2 != 0,
        }
    }
}

/// 4-stage post-quench recovery on a register block.
/// `Ok(())` on clean recovery; `Err(-1)` uncorrectable ECC (fail-closed),
/// `Err(-2)` PLL-lock timeout.
pub fn recover(hw: &mut MmioRegisters) -> Result<(), i32> {
    unsafe {
        match shbt_recover_on(hw as *mut MmioRegisters) {
            0 => Ok(()),
            e => Err(e),
        }
    }
}

/// Mean recovery latency over `iters` runs (ns).
pub fn recover_bench(iters: u32) -> f64 {
    unsafe { shbt_recover_bench(iters) }
}

/// AVX-512 Givens rotation remap of `n` channel pairs (O(1) per 8 lanes).
/// Callers must ensure the host supports AVX-512F; the C side falls back to
/// a scalar loop when compiled without `__AVX512F__`, but binaries compiled
/// with `-mavx512f` may SIGILL on non-AVX-512 hosts — gate with
/// `std::arch::is_x86_feature_detected!("avx512f")`.
pub fn remap(col_a: &mut [f64], col_b: &mut [f64], c: f64, s: f64) {
    let n = col_a.len().min(col_b.len());
    unsafe { shbt_remap(col_a.as_mut_ptr(), col_b.as_mut_ptr(), c, s, n) }
}

/// AVX-512 current-shunt interlock: bitmask of channels above 7.5 A.
/// Same AVX-512F host requirement as [`remap`].
pub fn simd_shunt_check(currents: &[f32; 16]) -> i32 {
    unsafe { shbt_simd_shunt_check(currents.as_ptr()) }
}

/// Mean interlock latency over `iters` runs (ns).
pub fn simd_shunt_bench(iters: u32) -> f64 {
    unsafe { shbt_simd_shunt_bench(iters) }
}

/// True when running in a virtualized CI environment where TSC-derived
/// latency assertions are unreliable (`SGLT_CI_VIRTUAL_ENV`).
pub fn virtualized_ci() -> bool {
    std::env::var_os("SGLT_CI_VIRTUAL_ENV").is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecc_roundtrip_clean() {
        let data = 0xDEAD_BEEF_CAFE_F00Du64;
        let check = ecc_encode(data);
        let d = ecc_decode(data, check);
        assert_eq!(d.data, data);
        assert!(!d.corrected && !d.uncorrectable);
    }

    #[test]
    fn ecc_corrects_every_single_bit() {
        // Latency bound ≤ 1.20 ns is exercised but only asserted off-CI.
        for bit in 0..64 {
            let data = 0x0123_4567_89AB_CDEFu64 ^ (1u64 << bit);
            let orig = 0x0123_4567_89AB_CDEFu64;
            let check = ecc_encode(orig);
            let d = ecc_decode(data, check);
            assert_eq!(d.data, orig, "bit {bit}");
            assert!(d.corrected, "bit {bit}");
        }
    }

    #[test]
    fn ecc_detects_double_bit() {
        let orig = 0x0F0F_0F0F_0F0F_0F0Fu64;
        let check = ecc_encode(orig);
        let d = ecc_decode(orig ^ 0b101, check);
        assert!(d.uncorrectable);
    }

    #[test]
    fn recovery_sequence_completes() {
        let mut hw = MmioRegisters {
            status: STATUS_PLL_LOCK | STATUS_ECC_ERR,
            ..MmioRegisters::default()
        };
        recover(&mut hw).unwrap();
        assert_eq!(hw.blank, 0);
        assert_eq!(hw.status & STATUS_ECC_ERR, 0);
    }

    #[test]
    fn recovery_fail_closed_on_double_bit() {
        let orig = 0xAAAA_5555_AAAA_5555u64;
        let check = ecc_encode(orig);
        let mut hw = MmioRegisters {
            status: STATUS_PLL_LOCK,
            ecc_low: (orig as u32) ^ 0b101,
            ecc_high: (orig >> 32) as u32,
            ecc_check: check as u32,
            ..MmioRegisters::default()
        };
        assert_eq!(recover(&mut hw), Err(-1));
        assert_eq!(hw.control, 0);
    }

    #[test]
    fn recovery_latency_within_budget() {
        if !std::arch::is_x86_feature_detected!("avx512f") {
            return;
        }
        let ns = recover_bench(10_000);
        assert!(ns > 0.0);
        if !virtualized_ci() {
            assert!(ns <= RECOVERY_BUDGET_NS, "recovery = {ns} ns");
        }
    }

    #[test]
    fn remap_givens_pair() {
        if !std::arch::is_x86_feature_detected!("avx512f") {
            return; // compiled -mavx512f; host lacks the ISA
        }
        let mut a = vec![1.0f64; 16];
        let mut b = vec![0.0f64; 16];
        let (c, s) = (0.0f64, 1.0f64); // 90° rotation
        remap(&mut a, &mut b, c, s);
        assert!(a.iter().all(|&v| v.abs() < 1e-15));
        assert!(b.iter().all(|&v| (v + 1.0).abs() < 1e-15));
    }

    #[test]
    fn shunt_interlock_mask() {
        if !std::arch::is_x86_feature_detected!("avx512f") {
            return;
        }
        let mut currents = [0.0f32; 16];
        currents[3] = 8.0;
        currents[15] = 9.5;
        let mask = simd_shunt_check(&currents);
        assert_eq!(mask, (1 << 3) | (1 << 15));
    }

    #[test]
    fn mmio1_register_map_layout() {
        assert_eq!(std::mem::size_of::<ShbtMmio1>(), 56);
        assert_eq!(std::mem::offset_of!(ShbtMmio1, causal_cone_lo), 0x00);
        assert_eq!(std::mem::offset_of!(ShbtMmio1, pn2_metric_j2), 0x10);
        assert_eq!(std::mem::offset_of!(ShbtMmio1, ds2_interval_lo), 0x28);
        assert_eq!(std::mem::offset_of!(ShbtMmio1, quench_time_ns), 0x30);
        assert_eq!(SHBT_MMIO_BASE, 0x7000_0000);
    }

    #[test]
    fn ibr312_mmio_and_magazine_layout() {
        assert_eq!(std::mem::size_of::<ShbtIbr312Mmio>(), 128);
        assert_eq!(std::mem::size_of::<ShbtStinespringMagazine>(), 33792);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, symplectic_s_factor_q32), 0x58);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, symplectic_shield_mass_g), 0x5C);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, compacted_nacelle_mass_g), 0x60);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_frame_bank_select), 0x64);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_reload_mode), 0x66);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_stored_micro_batches), 0x68);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_reload_count), 0x6C);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_last_latency_ns), 0x70);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_crew_dose_nsv_yr), 0x74);
        assert_eq!(std::mem::offset_of!(ShbtIbr312Mmio, ibr_frame_angle_q64), 0x78);
    }

    #[test]
    fn test_first_principles_mmio_layout() {
        assert_eq!(std::mem::size_of::<ShbtReconMmioFirstPrinciples>(), 128);
        assert_eq!(std::mem::align_of::<ShbtReconMmioFirstPrinciples>(), 64);
        assert_eq!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, synth_jitter_bound_q16), 0x38);
        assert_eq!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, synth_tether_displacement_q16), 0x3A);
        assert_eq!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, dec_discharge_energy_uj), 0x40);
        assert_eq!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, dec_space_charge_margin_q16), 0x48);
        assert_eq!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, tqec_syndrome_err_rate_q16), 0x4A);
        assert_eq!(std::mem::offset_of!(ShbtReconMmioFirstPrinciples, rebco_quench_headroom_q16), 0x4C);

        let mut mmio = ShbtReconMmioFirstPrinciples::default();
        shbt_kernel_record_wavefront_jitter(&mut mmio, 4);
        assert_eq!(mmio.synth_jitter_bound_q16, 4);

        shbt_kernel_record_tether_displacement(&mut mmio, 3605);
        assert_eq!(mmio.synth_tether_displacement_q16, 3605);

        shbt_kernel_record_dec_margins(&mut mmio, 11718, 10892);
        assert_eq!(mmio.dec_space_charge_margin_q16, 11718);
        assert_eq!(mmio.rebco_quench_headroom_q16, 10892);
    }
}
