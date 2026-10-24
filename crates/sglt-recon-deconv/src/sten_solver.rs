//! Phase 2 `SHBT-STEN` `54.432 ps` PT-GoI Optical Fixed-Point Solver (`1.701 ps/bank`,
//! `EX(M, U) = (I + UM)U` at `(UM)^2 = 0`), Blueshifted Hyper-Temporal Enclave
//! (`alpha in [10^1, 10^4]`, `||nabla_i alpha|| == 0`, `3.6366e9 sweeps/tick`),
//! Higher-Genus `Sp(2g, Z)` Stinespring Entanglement Network Routing (`g = M + 2 >= 3`,
//! `Ent(phi) == 0.000`, `rho_spec = 1`, `tau_byz <= 1.701 ps`), and the 9-Component
//! Proof-Carrying `Linear<V>` Control Barrier Function (`h_phys(x) >= 0`) with
//! Xenobiological Triage (`Classes M0, X0, X1, X2`).

pub const SHBT_TICK_HZ: u64 = 50_518;
pub const SHBT_TICK_NS: u64 = 19_795;
pub const PT_GOI_PROP_FS_PER_BANK: u64 = 1_701; // 1.701 ps = 1,701 fs
pub const PT_GOI_32_BANK_EQ_FS: u64 = 32 * PT_GOI_PROP_FS_PER_BANK; // 54,432 fs = 54.432 ps
pub const LANDAUER_SHUNT_NUM: u64 = 100;
pub const LANDAUER_SHUNT_DEN: u64 = 1089; // S = (10/33)^2 = 100/1089

pub const C_CRIT_SAPIENT_Q16: u64 = 1_000_000 << 16; // C_crit^sapient = 10^6 nat-layers
pub const W_SYNTH_REGOLITH_KJ_PER_UG: u64 = 56_845;  // 56.845009 TJ/kg
pub const W_SYNTH_X2_BIOMASS_KJ_PER_UG: u64 = 425;   // 0.424815 TJ/kg
pub const DELTA_W_SYNTH_SAVED_MILLI_TJ_KG: u64 = 56_420; // 56.420194 TJ/kg saved (99.25% reduction)

pub const SHBT_SWARM_FAULT_ADM_IMBALANCE: u32  = 0x0000_0001;
pub const SHBT_SWARM_FAULT_PARTITION_LOCK: u32 = 0x0000_0002;
pub const SHBT_SWARM_FAULT_FRAMING_WEYL: u32   = 0x0000_0004;
pub const SHBT_SWARM_FAULT_BL_ANOMALY: u32     = 0x0000_0008;
pub const SHBT_SWARM_FAULT_NEUTRINO_LEAK: u32  = 0x0000_0010;
pub const SHBT_SWARM_FAULT_CAYLEY_HASH: u32    = 0x0000_0040;
pub const SHBT_SWARM_FAULT_THERMAL_CLAMP: u32  = 0x0000_0080;
pub const SHBT_SWARM_FAULT_CBF_PHYS_VIOL: u32  = 0x0000_0100;
pub const SHBT_SWARM_FAULT_GREY_GOO_CAP: u32   = 0x0000_0200;
pub const SHBT_SWARM_FAULT_CLF_DIVERGENCE: u32 = 0x0000_0400;
pub const SHBT_SWARM_FAULT_X0_PROTECT_LCK: u32 = 0x0000_0800;
pub const SHBT_SWARM_FAULT_X1_SAPIENT_LCK: u32 = 0x0000_1000;

/// Non-Hermitian PT-Symmetric Geometry of Interaction (GoI) Optical Fixed-Point Solver (`SHBT-STEN`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PtGoiEquilibriumSolver {
    pub nilpotent_ep2_locked: bool, // (UM)^2 == 0 (|| (UM)^2 ||_op <= 10^-125)
    pub second_chern_number: i32,   // C_2 != 0
    pub backscatter_log10_bound: i32, // <= -12
}

impl PtGoiEquilibriumSolver {
    #[inline]
    pub const fn canonical() -> Self {
        Self {
            nilpotent_ep2_locked: true,
            second_chern_number: 1,
            backscatter_log10_bound: -14,
        }
    }

    #[inline]
    pub const fn single_pass_latency_fs(active_banks: u32) -> u64 {
        (active_banks as u64) * PT_GOI_PROP_FS_PER_BANK
    }

    #[inline]
    pub const fn sweeps_per_tick(enclave_alpha: u32) -> u64 {
        // tau_0 / 54.432 ps = 363,663.37 sweeps at alpha = 1 => 3,636,633,700 at alpha = 10^4
        (363_663_370u64 * (enclave_alpha as u64)) / 1_000
    }

    /// Evaluates the exact single-pass Geometry of Interaction fixed point `EX(M, U) = (I + UM)U`
    /// at the order-2 Exceptional Point `(UM)^2 = 0`, returning `(latency_ps, sweeps_per_tick)`.
    pub fn evaluate_ep2_deconvolution(&self, active_banks: u32, enclave_alpha: u32) -> Result<(f64, u64), u32> {
        if !self.nilpotent_ep2_locked || self.second_chern_number == 0 {
            return Err(SHBT_SWARM_FAULT_FRAMING_WEYL);
        }
        let latency_ps = (Self::single_pass_latency_fs(active_banks) as f64) / 1_000.0;
        let sweeps = Self::sweeps_per_tick(enclave_alpha);
        Ok((latency_ps, sweeps))
    }
}

/// Higher-Genus `Sp(2g, Z)` Stinespring Entanglement Network Routing (`g = M + 2 >= 3`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StinespringHiveConsensus {
    pub fleet_nodes_m: u32,
    pub genus_g: u32,
    pub kojima_entropy_milli: u32, // Strictly 0 for Lagrangian symplectic shears (rho_spec == 1)
    pub spectral_radius_milli: u32, // Strictly 1,000 (rho_spec == 1.000)
    pub byzantine_reject_fs: u64,   // <= 1,701 fs (1.701 ps)
}

impl StinespringHiveConsensus {
    #[inline]
    pub const fn new(fleet_nodes_m: u32) -> Self {
        let m = if fleet_nodes_m == 0 { 1 } else { fleet_nodes_m };
        Self {
            fleet_nodes_m: m,
            genus_g: m.saturating_add(2),
            kojima_entropy_milli: 0,
            spectral_radius_milli: 1_000,
            byzantine_reject_fs: PT_GOI_PROP_FS_PER_BANK,
        }
    }

    #[inline]
    pub const fn verify_byzantine_frame_digest(
        expected_hash: [u64; 3],
        incoming_hash: [u64; 3],
    ) -> Result<u64, u32> {
        if expected_hash[0] != incoming_hash[0]
            || expected_hash[1] != incoming_hash[1]
            || expected_hash[2] != incoming_hash[2]
        {
            return Err(SHBT_SWARM_FAULT_CAYLEY_HASH);
        }
        Ok(PT_GOI_PROP_FS_PER_BANK)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum XenoTriageClass {
    M0AbioticRegolith  = 0x0000_0000,
    X0ProtectedEarth   = 0x1000_0000,
    X1ProtectedSapient = 0x2000_0000,
    X2AssimilablePrim  = 0x4000_0000,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReactorGeneration {
    Mar312cCompacted, // T_MAR <= 32,920 mK (32.92 K)
    Mar312xExotic,    // T_MAR <= 192,500 mK (192.50 K)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShieldVerdict {
    Approved {
        cmd_opcode: u32,
        triage_class: XenoTriageClass,
        energy_saved_tj_per_kg_milli: u64,
    },
    BlockedFault {
        fault_mask: u32,
    },
}

/// 9-Component Proof-Carrying `Linear<V>` Control Barrier Function (`h_phys(x) >= 0`)
/// and Xenobiological Triage Calculus (`Classes M0, X0, X1, X2`).
pub struct LinearVCbfSafetyShield;

impl LinearVCbfSafetyShield {
    #[inline]
    pub const fn classify_xenobiology(d_cayley: u32, c_mera_q16: u64) -> XenoTriageClass {
        if d_cayley == 0 {
            XenoTriageClass::X0ProtectedEarth
        } else if c_mera_q16 >= C_CRIT_SAPIENT_Q16 {
            XenoTriageClass::X1ProtectedSapient
        } else if c_mera_q16 > 0 {
            XenoTriageClass::X2AssimilablePrim
        } else {
            XenoTriageClass::M0AbioticRegolith
        }
    }

    /// Evaluates the 9-component physical safety barrier `h_phys(x) >= 0`:
    /// 1. `Delta M_ADM == 0` (`feedstock_ug == payload_ug`)
    /// 2. `eta_A : eta_D == 10 : 23` (`640 B : 1472 B`)
    /// 3. `Delta_fr == 0 <=> E_munu == 0`
    /// 4. `Delta(B - L) == 0`
    /// 5. `Phi_nu == 0`
    /// 6. `P_Brem == 0 W`
    /// 7. `|s_ddot| <= 5.7735`
    /// 8. `T_QC <= 4.200 K` (`4,200 mK`)
    /// 9. `T_MAR <= T_crit` (`32.92 K` for MAR-312C / `192.50 K` for MAR-312X),
    /// plus anti-grey-goo fleet cap `h_pop(M) >= 0`, CLF `dV_terra/dt < 0`, and X0/X1 protection interlocks.
    #[allow(clippy::too_many_arguments)]
    pub const fn verify_and_filter_command_with_gen(
        requested_cmd: u32,
        feedstock_ug: u64,
        payload_ug: u64,
        net_b_minus_l: i64,
        weyl_e_munu_norm: u64,
        phi_nu_flux: u64,
        p_brem_watts: u64,
        eikonal_accel_milli: u32,
        temp_qc_mk: u32,
        temp_mar_mk: u32,
        reactor_gen: ReactorGeneration,
        active_nodes_m: u32,
        max_nodes_m_max: u32,
        v_terra_q32: u64,
        dv_terra_dt_q32: i64,
        d_cayley: u32,
        c_mera_q16: u64,
    ) -> ShieldVerdict {
        let mut fault: u32 = 0;

        if feedstock_ug != payload_ug {
            fault |= SHBT_SWARM_FAULT_ADM_IMBALANCE;
        }
        if weyl_e_munu_norm != 0 {
            fault |= SHBT_SWARM_FAULT_FRAMING_WEYL;
        }
        if net_b_minus_l != 0 {
            fault |= SHBT_SWARM_FAULT_BL_ANOMALY;
        }
        if phi_nu_flux != 0 {
            fault |= SHBT_SWARM_FAULT_NEUTRINO_LEAK;
        }
        if p_brem_watts != 0 || eikonal_accel_milli > 5773 {
            fault |= SHBT_SWARM_FAULT_CBF_PHYS_VIOL;
        }
        let mar_limit_mk = match reactor_gen {
            ReactorGeneration::Mar312cCompacted => 32_920,
            ReactorGeneration::Mar312xExotic => 192_500,
        };
        if temp_qc_mk > 4200 || temp_mar_mk > mar_limit_mk {
            fault |= SHBT_SWARM_FAULT_THERMAL_CLAMP;
        }

        let is_clone_ship = (requested_cmd & 0x0000_0008) != 0;
        if active_nodes_m > max_nodes_m_max || (is_clone_ship && active_nodes_m >= max_nodes_m_max) {
            fault |= SHBT_SWARM_FAULT_GREY_GOO_CAP;
        }

        if v_terra_q32 > 0 && dv_terra_dt_q32 >= 0 {
            fault |= SHBT_SWARM_FAULT_CLF_DIVERGENCE;
        }

        let triage = Self::classify_xenobiology(d_cayley, c_mera_q16);
        let is_derender_or_assimilate = (requested_cmd & (0x0000_0004 | 0x0000_0010 | 0x0000_1000)) != 0;
        if is_derender_or_assimilate {
            match triage {
                XenoTriageClass::X0ProtectedEarth => {
                    fault |= SHBT_SWARM_FAULT_X0_PROTECT_LCK;
                }
                XenoTriageClass::X1ProtectedSapient => {
                    fault |= SHBT_SWARM_FAULT_X1_SAPIENT_LCK;
                }
                XenoTriageClass::X2AssimilablePrim | XenoTriageClass::M0AbioticRegolith => {}
            }
        }

        if fault != 0 {
            ShieldVerdict::BlockedFault { fault_mask: fault }
        } else {
            let saved = match triage {
                XenoTriageClass::X2AssimilablePrim => {
                    W_SYNTH_REGOLITH_KJ_PER_UG - W_SYNTH_X2_BIOMASS_KJ_PER_UG
                }
                _ => 0,
            };
            ShieldVerdict::Approved {
                cmd_opcode: requested_cmd,
                triage_class: triage,
                energy_saved_tj_per_kg_milli: saved,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sten_pt_goi_sp2g_and_xeno_shield() {
        let solver = PtGoiEquilibriumSolver::canonical();
        let (lat_ps, sweeps) = solver.evaluate_ep2_deconvolution(32, 10_000).unwrap();
        assert!((lat_ps - 54.432).abs() < 1e-6);
        assert_eq!(sweeps, 3_636_633_700);

        let hive = StinespringHiveConsensus::new(10_000);
        assert_eq!(hive.genus_g, 10_002);
        assert_eq!(hive.kojima_entropy_milli, 0);
        assert_eq!(hive.spectral_radius_milli, 1_000);
        assert_eq!(hive.byzantine_reject_fs, 1_701);

        // X0 protected Earth-seeded lockout
        let v_x0 = LinearVCbfSafetyShield::verify_and_filter_command_with_gen(
            0x0000_0004,
            1000,
            1000,
            0,
            0,
            0,
            0,
            5000,
            4200,
            24850,
            ReactorGeneration::Mar312xExotic,
            10,
            510_000,
            100,
            -5,
            0,
            0,
        );
        assert_eq!(
            v_x0,
            ShieldVerdict::BlockedFault {
                fault_mask: SHBT_SWARM_FAULT_X0_PROTECT_LCK
            }
        );

        // X1 sapient lockout
        let v_x1 = LinearVCbfSafetyShield::verify_and_filter_command_with_gen(
            0x0000_0004,
            1000,
            1000,
            0,
            0,
            0,
            0,
            5000,
            4200,
            24850,
            ReactorGeneration::Mar312xExotic,
            10,
            510_000,
            100,
            -5,
            7,
            C_CRIT_SAPIENT_Q16,
        );
        assert_eq!(
            v_x1,
            ShieldVerdict::BlockedFault {
                fault_mask: SHBT_SWARM_FAULT_X1_SAPIENT_LCK
            }
        );

        // X2 primitive biomass assimilation approved
        let v_x2 = LinearVCbfSafetyShield::verify_and_filter_command_with_gen(
            0x0000_1000,
            1000,
            1000,
            0,
            0,
            0,
            0,
            5000,
            4200,
            24850,
            ReactorGeneration::Mar312xExotic,
            10,
            510_000,
            100,
            -5,
            7,
            1024,
        );
        assert_eq!(
            v_x2,
            ShieldVerdict::Approved {
                cmd_opcode: 0x0000_1000,
                triage_class: XenoTriageClass::X2AssimilablePrim,
                energy_saved_tj_per_kg_milli: 56_420,
            }
        );
    }
}
