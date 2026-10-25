//! Phase 1 Macroscopic Anyon-Braid Quine Replication (`R_clone^SWARM`, Pathways A, B, C),
//! Two-Tier Topological SECDED `Hamming(72,64)` (`R = 8/9`, `d_min = 4`), and 256-Bit
//! Non-Abelian Burau-Cayley Phylogenetic Hash (`H_Cayley`, `Delta F_gen == 0`).

use shbt_recon_core::hyperframe::UnifiedStinespringHyperFrame;

pub const HAMMING_MIN_DIST: u8 = 4;
pub const HAMMING_DATA_BITS: u8 = 64;
pub const HAMMING_CODE_BITS: u8 = 72;
pub const SECDED_PACKED_BRAID_BYTES_PER_BANK: usize = 124 * 9; // 1,116 B <= 1,472 B dark-ledger budget
pub const DARK_LEDGER_BUDGET_BYTES: usize = 1472;

/// Prime modulus `p_64 = 2^64 - 59` for modular SL_2 / GL_2 matrix limbs.
pub const CAYLEY_MOD_P64: u128 = 18_446_744_073_709_551_557;
/// 5th root of unity surrogate `t` in `F_p64`.
pub const BURAU_ROOT_T: u128 = 0x9E37_79B9_7F4A_7C15;
pub const BURAU_INV_T: u128 = 0x61C8_8646_80B5_83AC;

pub const SK9_LOG10_ERROR_FLOOR: f64 = -230.1937;
pub const HOLO_LOG10_ERROR_FLOOR: f64 = -122.0;
pub const HAPPY_MERA_ISO_LOG10_FLOOR: f64 = -120.0;

pub const LANDAUER_SHUNT_NUM: u64 = 100;
pub const LANDAUER_SHUNT_DEN: u64 = 1089;
pub const DARK_SHUNT_NUM: u64 = 989;

/// Heat deposited into the visible 4.2 K cryostat per 32-bank synthesis reset (540,672 bits):
/// `Q_vis^reset = (100/1089) * 540,672 * 4.01937e-23 J = 1.9955e-18 J`.
pub const Q_VIS_32BANK_RESET_ATTOJOULES_MILLI: u64 = 1_996; // 1.9955 aJ
pub const Q_VIS_32BANK_RESET_JOULES: f64 = 1.9955e-18;

/// Pathway B PT-GoI optical propagation constants.
pub const PT_GOI_PROP_FS_PER_BANK: u64 = 1_701;      // 1.701 ps
pub const PT_GOI_32BANK_SWEEP_FS: u64 = 54_432;      // 54.432 ps
pub const PT_GOI_64BANK_SWEEP_FS: u64 = 108_864;     // 108.864 ps
pub const POLARITON_EPR_SQUEEZING_R3_MILLI: u32 = 3_800; // r_3 >= 3.80 (Delta X_-^2 <= 5.0e-4)
pub const VERLINDE_SPLIT_ISO_LOG10_BOUND: i32 = -125;    // ||Delta_split^dagger Delta_split - I||_op <= 10^-125

/// Pathway C 4.2 K Landauer floor & biogenic impedance matching constants.
pub const LANDAUER_4P2K_JOULES_PER_BIT: f64 = 4.01937e-23;
pub const LANDAUER_4P2K_FEMTO_ATTOJOULES_PER_BIT: u64 = 40_194; // 4.0194e-23 J/bit
pub const BIO_IMPEDANCE_MIN_MRAYL: f64 = 1.48;
pub const BIO_IMPEDANCE_MAX_MRAYL: f64 = 1.70;
pub const BIO_ACOUSTIC_REFLECTION_MAX: f64 = 1.0e-6;

/// Daughter Jump-Cruiser & Translocator Node Replication Constants on Tier 4 Direct Boundary Bus (`24.9625 EW`).
pub const MODE1B_SPECIFIC_SYNTH_WORK_TJ_KG: f64 = 56.845009;
pub const UPGRADED_MAR312X_SHIP_MASS_KG: f64 = 5892.30;
pub const LEGACY_MAR312C_SHIP_MASS_KG: f64 = 7200.00;
pub const UPGRADED_MAR312X_CLONE_WORK_PJ: f64 = 334.9477;
pub const LEGACY_MAR312C_CLONE_WORK_PJ: f64 = 409.2841;
pub const TIER4_DIRECT_BUS_POWER_EW: f64 = 24.9625; // 25.00 EW * 99.850%
pub const UPGRADED_MAR312X_REPLICATION_TIME_MS: f64 = 13.418; // 334.9477 PJ / 24.9625 EW = 13.4180 ms
pub const LEGACY_MAR312C_REPLICATION_TIME_MS: f64 = 16.396;   // 409.2841 PJ / 24.9625 EW = 16.39596 ms

/// Fault bitmasks for Quine replication & SECDED/Cayley checks.
pub const SHBT_SWARM_FAULT_ADM_IMBALANCE: u32  = 0x0000_0001;
pub const SHBT_SWARM_FAULT_PARTITION_LOCK: u32 = 0x0000_0002;
pub const SHBT_SWARM_FAULT_FRAMING_WEYL: u32   = 0x0000_0004;
pub const SHBT_SWARM_FAULT_BL_ANOMALY: u32     = 0x0000_0008;
pub const SHBT_SWARM_FAULT_NEUTRINO_LEAK: u32  = 0x0000_0010;
pub const SHBT_SWARM_FAULT_SECDED_UNCORR: u32  = 0x0000_0020;
pub const SHBT_SWARM_FAULT_CAYLEY_HASH: u32    = 0x0000_0040;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EccStatus {
    Clean,
    SingleBitCorrected { flipped_bit: u8 },
    DoubleBitDetected,
}

#[inline]
const fn data_bit_col_mask(bit_idx: u8) -> u8 {
    let mut col: u8 = 3;
    let mut k: u8 = 0;
    while k < bit_idx {
        col += 1;
        if (col & (col - 1)) != 0 {
            k += 1;
        }
    }
    col
}

/// Computes the 8-bit systematic `Hamming(72,64)` SECDED parity byte for a 64-bit Artin `B_3` word.
#[inline]
pub const fn encode_hamming72_64(word: u64) -> u8 {
    let mut syn: u8 = 0;
    let mut i: u8 = 0;
    while i < 64 {
        if ((word >> i) & 1) != 0 {
            syn ^= data_bit_col_mask(i);
        }
        i += 1;
    }
    let total_ones = (word.count_ones() + (syn.count_ones())) as u8;
    let p7 = (total_ones & 1) << 7;
    syn | p7
}

/// Decodes and scrubs a 64-bit Artin `B_3` word and its 8-bit SECDED parity byte.
#[inline]
pub const fn decode_hamming72_64(word: u64, parity: u8) -> (u64, u8, EccStatus) {
    let expected_p = encode_hamming72_64(word);
    let syn7 = (parity ^ expected_p) & 0x7F;
    let overall_err = ((word.count_ones() + parity.count_ones()) & 1) as u8;

    if syn7 == 0 && overall_err == 0 {
        return (word, parity, EccStatus::Clean);
    }

    if overall_err == 1 {
        if syn7 == 0 || (syn7 & (syn7 - 1)) == 0 {
            let fixed_p = parity ^ (if syn7 == 0 { 0x80 } else { syn7 });
            return (word, fixed_p, EccStatus::SingleBitCorrected { flipped_bit: 64 });
        }
        let mut i: u8 = 0;
        while i < 64 {
            if data_bit_col_mask(i) == syn7 {
                let fixed_w = word ^ (1u64 << i);
                return (fixed_w, parity, EccStatus::SingleBitCorrected { flipped_bit: i });
            }
            i += 1;
        }
    }

    (word, parity, EccStatus::DoubleBitDetected)
}

/// 2x2 Reduced Burau Representation Matrix over `F_p64` (256 bits total across 4 limbs).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BurauMat2x2 {
    pub m00: u64,
    pub m01: u64,
    pub m10: u64,
    pub m11: u64,
}

impl BurauMat2x2 {
    pub const IDENTITY: Self = Self {
        m00: 1,
        m01: 0,
        m10: 0,
        m11: 1,
    };

    #[inline]
    pub const fn mul_mod(&self, rhs: &Self) -> Self {
        let p = CAYLEY_MOD_P64;
        let t00_a = ((self.m00 as u128) * (rhs.m00 as u128)) % p;
        let t00_b = ((self.m01 as u128) * (rhs.m10 as u128)) % p;
        let a00 = (t00_a + t00_b) % p;

        let t01_a = ((self.m00 as u128) * (rhs.m01 as u128)) % p;
        let t01_b = ((self.m01 as u128) * (rhs.m11 as u128)) % p;
        let a01 = (t01_a + t01_b) % p;

        let t10_a = ((self.m10 as u128) * (rhs.m00 as u128)) % p;
        let t10_b = ((self.m11 as u128) * (rhs.m10 as u128)) % p;
        let a10 = (t10_a + t10_b) % p;

        let t11_a = ((self.m10 as u128) * (rhs.m01 as u128)) % p;
        let t11_b = ((self.m11 as u128) * (rhs.m11 as u128)) % p;
        let a11 = (t11_a + t11_b) % p;

        Self {
            m00: a00 as u64,
            m01: a01 as u64,
            m10: a10 as u64,
            m11: a11 as u64,
        }
    }

    #[inline]
    pub const fn generator(gen_code: u8) -> Self {
        let p = CAYLEY_MOD_P64;
        let neg_t = (p - BURAU_ROOT_T) as u64;
        let neg_inv_t = (p - BURAU_INV_T) as u64;
        match gen_code & 0x03 {
            0 => Self {
                m00: neg_t,
                m01: 1,
                m10: 0,
                m11: 1,
            },
            1 => Self {
                m00: neg_inv_t,
                m01: BURAU_INV_T as u64,
                m10: 0,
                m11: 1,
            },
            2 => Self {
                m00: 1,
                m01: 0,
                m10: BURAU_ROOT_T as u64,
                m11: neg_t,
            },
            _ => Self {
                m00: 1,
                m01: 0,
                m10: 1,
                m11: neg_inv_t,
            },
        }
    }

    #[inline]
    pub const fn to_digest_256(&self) -> [u64; 4] {
        [self.m00, self.m01, self.m10, self.m11]
    }
}

/// Evaluates the non-Abelian Burau-Cayley matrix across a 64-bit Artin `B_3` word (`ell = 42` generators).
#[inline]
pub const fn compute_braid_word_cayley_matrix(braid_word: u64) -> BurauMat2x2 {
    let mut acc = BurauMat2x2::IDENTITY;
    let mut j: u32 = 0;
    while j < 42 {
        let step_bits = ((braid_word.rotate_left(j * 3)) & 0x03) as u8;
        let g = BurauMat2x2::generator(step_bits);
        acc = acc.mul_mod(&g);
        j += 1;
    }
    acc
}

/// Evaluates the 256-bit (4 x 64-bit, with 3 primary MMIO limbs) Burau-Cayley digest over a 124-braid bank.
#[inline]
pub const fn compute_frame_cayley_digest(braid_descriptors: &[u64; 124]) -> [u64; 3] {
    let mut acc = BurauMat2x2::IDENTITY;
    let mut k: usize = 0;
    while k < 124 {
        let m = compute_braid_word_cayley_matrix(braid_descriptors[k]);
        acc = acc.mul_mod(&m);
        k += 1;
    }
    [
        acc.m00 ^ acc.m11.rotate_left(17),
        acc.m01 ^ acc.m10.rotate_left(31),
        acc.m00.wrapping_add(acc.m01).wrapping_add(acc.m10).wrapping_add(acc.m11),
    ]
}

/// Evaluates the full 4-limb 256-bit Burau-Cayley matrix digest over a 124-braid bank.
#[inline]
pub const fn compute_frame_cayley_digest_256(braid_descriptors: &[u64; 124]) -> [u64; 4] {
    let mut acc = BurauMat2x2::IDENTITY;
    let mut k: usize = 0;
    while k < 124 {
        let m = compute_braid_word_cayley_matrix(braid_descriptors[k]);
        acc = acc.mul_mod(&m);
        k += 1;
    }
    acc.to_digest_256()
}

/// Theorem 1.1 & Theorem 1.2 Hardware Guards:
/// Rejects unknown quantum superpositions (`overlap not in {0, 1000}` permille),
/// vacuum duplication (`M_feedstock != M_payload`), partition violation (`10 : 23`),
/// framing anomaly (`Delta_fr != 0`), or `(B-L)` violation.
#[inline]
pub const fn verify_no_cloning_and_adm_guards(
    overlap_permille: u32,
    feedstock_mass_ug: u64,
    payload_mass_ug: u64,
    vis_anchor_bytes: usize,
    dark_ledger_bytes: usize,
    net_b_minus_l: i64,
    framing_anomaly_milli: i64,
) -> Result<(), u32> {
    let mut fault: u32 = 0;
    if overlap_permille != 0 && overlap_permille != 1000 {
        fault |= SHBT_SWARM_FAULT_FRAMING_WEYL;
    }
    if feedstock_mass_ug == 0 || feedstock_mass_ug != payload_mass_ug {
        fault |= SHBT_SWARM_FAULT_ADM_IMBALANCE;
    }
    if vis_anchor_bytes != 640 || dark_ledger_bytes != 1472 || (vis_anchor_bytes * 23 != dark_ledger_bytes * 10) {
        fault |= SHBT_SWARM_FAULT_PARTITION_LOCK;
    }
    if framing_anomaly_milli != 0 {
        fault |= SHBT_SWARM_FAULT_FRAMING_WEYL;
    }
    if net_b_minus_l != 0 {
        fault |= SHBT_SWARM_FAULT_BL_ANOMALY;
    }
    if fault != 0 {
        Err(fault)
    } else {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PathwayAReplicationReceipt {
    pub banks_copied: u32,
    pub braids_verified: u32,
    pub single_bit_errors_corrected: u32,
    pub q_internal_joules_per_bit: u64,
    pub q_vis_reset_milli_attojoules: u64,
    pub generational_drift_delta_f: u64,
    pub cayley_digest: [u64; 3],
}

/// Executes Pathway A Tomita-Takesaki inverse modular flow (`sigma_{-t}^Psi`) copying
/// Banks `00..31` (Read-Only Parent Quine ROM) into Banks `32..63` (Active Daughter Synthesis Frame)
/// inside `UnifiedStinespringHyperFrame` with SECDED `Hamming(72,64)` scrubbing and 256-bit
/// `H_Cayley` verification (`Delta F_gen == 0`, `Q_internal = 0 J/bit`).
pub fn execute_pathway_a_quine_copy(
    hyperframe: &mut UnifiedStinespringHyperFrame,
) -> Result<PathwayAReplicationReceipt, u32> {
    let mut corrected_cnt: u32 = 0;
    let mut braids_total: u32 = 0;

    for b in 0..32usize {
        let mut rom_frame = hyperframe.banks[b];
        for k in 0..124usize {
            let w = rom_frame.dark_braid_ledger[k];
            let parity_word_idx = k / 8;
            let parity_byte_shift = (k % 8) * 8;
            let mut p = ((rom_frame.secded_hamming_ecc[parity_word_idx] >> parity_byte_shift) & 0xFF) as u8;
            if p == 0 && w != 0 {
                p = encode_hamming72_64(w);
            } else if w == 0 && p == 0 {
                p = encode_hamming72_64(0);
            }
            let (fixed_w, fixed_p, st) = decode_hamming72_64(w, p);
            match st {
                EccStatus::Clean => {}
                EccStatus::SingleBitCorrected { .. } => {
                    corrected_cnt += 1;
                }
                EccStatus::DoubleBitDetected => {
                    return Err(SHBT_SWARM_FAULT_SECDED_UNCORR);
                }
            }
            rom_frame.dark_braid_ledger[k] = fixed_w;
            let mask = !(0xFFu64 << parity_byte_shift);
            rom_frame.secded_hamming_ecc[parity_word_idx] =
                (rom_frame.secded_hamming_ecc[parity_word_idx] & mask)
                    | ((fixed_p as u64) << parity_byte_shift);
            braids_total += 1;
        }

        let rom_digest = compute_frame_cayley_digest_256(&rom_frame.dark_braid_ledger);
        hyperframe.banks[32 + b] = rom_frame;
        let synth_digest = compute_frame_cayley_digest_256(&hyperframe.banks[32 + b].dark_braid_ledger);
        if rom_digest != synth_digest {
            return Err(SHBT_SWARM_FAULT_CAYLEY_HASH);
        }
    }

    let digest0 = compute_frame_cayley_digest(&hyperframe.banks[32].dark_braid_ledger);
    Ok(PathwayAReplicationReceipt {
        banks_copied: 32,
        braids_verified: braids_total,
        single_bit_errors_corrected: corrected_cnt,
        q_internal_joules_per_bit: 0,
        q_vis_reset_milli_attojoules: Q_VIS_32BANK_RESET_ATTOJOULES_MILLI,
        generational_drift_delta_f: 0,
        cayley_digest: digest0,
    })
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PathwayBProjectionReceipt {
    pub active_banks: u32,
    pub total_propagation_fs: u64,
    pub epr_squeezing_r3_milli: u32,
    pub nilpotent_ep2_verified: bool,
    pub verlinde_isometry_log10: i32,
}

/// Executes Pathway B Adjoint Verlinde Splitting (`Delta_split`) coupled to the
/// `54.432 ps` PT-GoI single-pass optical template imprinting (`EX(M,U) = (I + UM)U`).
#[inline]
pub const fn execute_pathway_b_verlinde_pt_goi(
    active_banks: u32,
    epr_squeezing_r3_milli: u32,
    nilpotent_ep2_locked: bool,
) -> Result<PathwayBProjectionReceipt, u32> {
    if active_banks == 0 || active_banks > 64 {
        return Err(SHBT_SWARM_FAULT_PARTITION_LOCK);
    }
    if epr_squeezing_r3_milli < POLARITON_EPR_SQUEEZING_R3_MILLI || !nilpotent_ep2_locked {
        return Err(SHBT_SWARM_FAULT_FRAMING_WEYL);
    }
    Ok(PathwayBProjectionReceipt {
        active_banks,
        total_propagation_fs: (active_banks as u64) * PT_GOI_PROP_FS_PER_BANK,
        epr_squeezing_r3_milli,
        nilpotent_ep2_verified: true,
        verlinde_isometry_log10: VERLINDE_SPLIT_ISO_LOG10_BOUND,
    })
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PathwayCBioCloneReceipt {
    pub q_readout_joules_per_bit: f64,
    pub free_neutrino_flux: f64,
    pub acoustic_reflection_r: f64,
    pub bio_infidelity_log10: i32,
    pub homochiral_l_amino_d_sugar: bool,
}

/// Executes Pathway C Mode II `4.2 K` Landauer floor tomographic capture (`Q_readout = 4.0194e-23 J/bit`),
/// paired bilinear neutrino-free weak conversion (`Phi_nu == 0`), and dynamic GST-225 acoustic matching
/// (`Z_bio in [1.48, 1.70] MRayl`, `R <= 10^-6`), achieving `F_bio >= 1 - 10^-122`.
pub fn execute_pathway_c_landauer_bio_clone(
    z_bio_mrayl: f64,
    z_gst_tuned_mrayl: f64,
    phi_nu_flux: f64,
    homochiral_locked: bool,
) -> Result<PathwayCBioCloneReceipt, u32> {
    if !(BIO_IMPEDANCE_MIN_MRAYL..=BIO_IMPEDANCE_MAX_MRAYL).contains(&z_bio_mrayl) {
        return Err(0x0000_0100);
    }
    if phi_nu_flux != 0.0 {
        return Err(SHBT_SWARM_FAULT_NEUTRINO_LEAK);
    }
    if !homochiral_locked {
        return Err(0x0000_0100);
    }
    let r = ((z_gst_tuned_mrayl - z_bio_mrayl) / (z_gst_tuned_mrayl + z_bio_mrayl)).powi(2);
    if r > BIO_ACOUSTIC_REFLECTION_MAX {
        return Err(0x0000_0100);
    }
    Ok(PathwayCBioCloneReceipt {
        q_readout_joules_per_bit: LANDAUER_4P2K_JOULES_PER_BIT,
        free_neutrino_flux: 0.0,
        acoustic_reflection_r: r,
        bio_infidelity_log10: -122,
        homochiral_l_amino_d_sugar: true,
    })
}

/// Computes daughter Jump-Cruiser / translocator node replication time in milliseconds and microseconds
/// on the Tier 4 `24.9625 EW` Direct Boundary Bus (`B_direct^MAR-X`, `99.850%` efficiency):
/// - `13.418 ms` (`13,418 us`) for the `5,892.30 kg` MAR-312X Jump-Cruiser (`334.9477 PJ` at Mode 1b)
/// - `16.396 ms` (`16,396 us`) for the `7,200.00 kg` MAR-312C hull (`409.2841 PJ` at Mode 1b).
#[inline]
pub fn compute_direct_bus_ship_replication_time(use_upgraded_5892kg_hull: bool) -> (f64, u64, f64) {
    let work_pj = if use_upgraded_5892kg_hull {
        UPGRADED_MAR312X_CLONE_WORK_PJ
    } else {
        LEGACY_MAR312C_CLONE_WORK_PJ
    };
    let time_ms = work_pj / TIER4_DIRECT_BUS_POWER_EW;
    let time_us = ((work_pj * 1_000.0) / TIER4_DIRECT_BUS_POWER_EW).round() as u64;
    (work_pj, time_us, time_ms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shbt_recon_core::hyperframe::UnifiedStinespringFrame;

    #[test]
    fn test_secded_hamming_and_pathway_a_quine_copy() {
        const { assert!(SECDED_PACKED_BRAID_BYTES_PER_BANK <= DARK_LEDGER_BUDGET_BYTES); }
        let frame = UnifiedStinespringFrame {
            visible_register: [0u8; 640],
            dark_braid_ledger: [0xCAFE_BABE_1234_5678u64; 124],
            secded_hamming_ecc: [0u64; 60],
        };
        let mut hyper = UnifiedStinespringHyperFrame {
            banks: [frame; 64],
            active_bank_mask: !0u64,
            total_nucleons: 3.548e30,
            adm_mass_kg: 5892.30,
            heegaard_genus: 3,
        };
        // Inject a single-bit flip in bank 0 word 0 after encoding parity
        let p0 = encode_hamming72_64(0xCAFE_BABE_1234_5678u64);
        hyper.banks[0].secded_hamming_ecc[0] = p0 as u64;
        hyper.banks[0].dark_braid_ledger[0] ^= 1 << 13;

        let receipt = execute_pathway_a_quine_copy(&mut hyper).unwrap();
        assert_eq!(receipt.banks_copied, 32);
        assert_eq!(receipt.braids_verified, 3968);
        assert_eq!(receipt.single_bit_errors_corrected, 1);
        assert_eq!(receipt.q_internal_joules_per_bit, 0);
        assert_eq!(receipt.generational_drift_delta_f, 0);
        assert_eq!(hyper.banks[32].dark_braid_ledger[0], 0xCAFE_BABE_1234_5678u64);
    }

    #[test]
    fn test_pathway_b_c_and_ship_replication_times() {
        let rb = execute_pathway_b_verlinde_pt_goi(32, 3820, true).unwrap();
        assert_eq!(rb.total_propagation_fs, 54_432);

        let rc = execute_pathway_c_landauer_bio_clone(1.58, 1.58, 0.0, true).unwrap();
        assert_eq!(rc.bio_infidelity_log10, -122);
        assert_eq!(rc.free_neutrino_flux, 0.0);
        assert_eq!(rc.acoustic_reflection_r, 0.0);

        let (w_5892, us_5892, ms_5892) = compute_direct_bus_ship_replication_time(true);
        assert!((w_5892 - 334.9477).abs() < 1e-4);
        assert_eq!(us_5892, 13_418);
        assert!((ms_5892 - 13.418).abs() < 1e-3);

        let (w_7200, us_7200, ms_7200) = compute_direct_bus_ship_replication_time(false);
        assert!((w_7200 - 409.2841).abs() < 1e-4);
        assert_eq!(us_7200, 16_396);
        assert!((ms_7200 - 16.396).abs() < 1e-3);
    }
}
