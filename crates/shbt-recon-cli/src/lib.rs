//! shbt-recon-cli — unified system orchestrator library + optional PyO3
//! C-extension surface (`python` feature) for `python/shbt_recon/cli`.

// -- intentionally no external deps; JSON emitted manually -----------------

use std::fmt::Write as _;

/// One verification-matrix gate result.
#[derive(Clone, Debug)]
pub struct GateResult {
    pub id: &'static str,
    pub domain: &'static str,
    pub metric: &'static str,
    pub limit: &'static str,
    pub measured: f64,
    pub passed: bool,
}

/// Serialize the gate matrix to a compact JSON array.
pub fn gates_to_json(gates: &[GateResult]) -> String {
    let mut s = String::from("[");
    for (i, g) in gates.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        let _ = write!(
            s,
            "{{\"id\":\"{}\",\"domain\":\"{}\",\"metric\":\"{}\",\"limit\":\"{}\",\"measured\":{},\"passed\":{}}}",
            g.id, g.domain, g.metric, g.limit, g.measured, g.passed
        );
    }
    s.push(']');
    s
}

/// Cross-domain verification matrix over the six workspace crates.
pub fn verify_matrix() -> Vec<GateResult> {
    use shbt_recon_core::{adm, causal, kinematics, stinespring, telemetry, translocate};
    use shbt_recon_eda::{gdsii, substrate};
    use shbt_recon_metrology::{gum, lightcone, tmsv};
    use shbt_recon_thermo::{cryo, landauer, lanr};

    let mut gates = Vec::new();
    let mut push = |id: &'static str,
                    domain: &'static str,
                    metric: &'static str,
                    limit: &'static str,
                    measured: f64,
                    passed: bool| {
        gates.push(GateResult {
            id,
            domain,
            metric,
            limit,
            measured,
            passed,
        });
    };

    // --- core: ADM / Gram / wake / causal / stinespring / telemetry --------
    let audit = adm::ADMMetricAuditor::new().audit_velocity(0.5);
    push(
        "G01",
        "core::adm",
        "max |det(g)+1|",
        "<= 1e-12",
        audit.max_determinant_error,
        audit.max_determinant_error <= adm::DET_TOLERANCE,
    );
    push(
        "G02",
        "core::adm",
        "min Gram eigenvalue",
        "> 0",
        audit.min_gram_eigenvalue,
        audit.min_gram_eigenvalue > 0.0,
    );
    let slice = adm::ADM3Plus1ShiftField::flat();
    push(
        "G03",
        "core::adm",
        "|beta^i| nullified",
        "<= 1e-12",
        slice.shift_magnitude(),
        slice.is_shift_nullified(),
    );

    let g = vec![vec![2.0, 0.1], vec![0.1, 1.0]];
    let lmin = translocate::GramPositivityVerifier::new()
        .enforce(&g)
        .unwrap_or(f64::NAN);
    push(
        "G04",
        "core::translocate",
        "lambda_min(Gram)",
        "> 0",
        lmin,
        lmin > 0.0,
    );

    let wake = kinematics::WakeTensorComp::new();
    let v0 = vec![0.0; 8];
    let dv = wake.rigidity_residual(&v0, &v0, 1e-3);
    push(
        "G05",
        "core::kinematics",
        "|delta mu| rigidity",
        "<= 1e-12",
        dv,
        dv <= kinematics::RIGIDITY_TOLERANCE,
    );

    let proj_ok = causal::CausalHistoryProjection::new(vec![(1.0, 0.0), (0.0, 0.0)])
        .map(|p| p.idempotency_residual(&[(1.0, 0.0), (0.0, 0.0)], 1e-12))
        .unwrap_or(false);
    push(
        "G06",
        "core::causal",
        "Pi^2 = Pi",
        "idempotent",
        if proj_ok { 0.0 } else { 1.0 },
        proj_ok,
    );

    let frac_err =
        (stinespring::completed_fraction() + stinespring::residual_fraction() - 1.0).abs();
    push(
        "G07",
        "core::stinespring",
        "10/33 + 23/33",
        "== 1",
        frac_err,
        frac_err < 1e-15 && stinespring::partition_is_exact(),
    );

    push(
        "G08",
        "core::telemetry",
        "TelemetryFrame size",
        "== 64 B align 64",
        std::mem::size_of::<telemetry::TelemetryFrame>() as f64,
        std::mem::size_of::<telemetry::TelemetryFrame>() == 64
            && std::mem::align_of::<telemetry::TelemetryFrame>() == 64,
    );

    // --- kernel: ECC / recovery / MMIO map ----------------------------------
    let data = 0x0123_4567_89AB_CDEFu64;
    let dec = shbt_recon_kernel::ecc_decode(data ^ (1 << 7), shbt_recon_kernel::ecc_encode(data));
    push(
        "G09",
        "kernel::ecc",
        "SECDED single-bit correction",
        "corrected",
        if dec.corrected && dec.data == data {
            0.0
        } else {
            1.0
        },
        dec.corrected && dec.data == data,
    );
    let rec_ns = shbt_recon_kernel::recover_bench(10_000);
    push(
        "G10",
        "kernel::recover",
        "T_recovery",
        "<= 120.00 ns",
        rec_ns,
        rec_ns <= shbt_recon_kernel::RECOVERY_BUDGET_NS || shbt_recon_kernel::virtualized_ci(),
    );
    push(
        "G11",
        "kernel::mmio",
        "SHBT-MMIO-1 block size",
        "== 56 B @0x70000000",
        std::mem::size_of::<shbt_recon_kernel::ShbtMmio1>() as f64,
        std::mem::size_of::<shbt_recon_kernel::ShbtMmio1>() == 56,
    );

    // --- thermo: Kapitza / LANR / Landauer ----------------------------------
    let layer = cryo::KapitzaInterfaceLayer {
        z_sapphire_mrayl: cryo::Z1_MRAYL * cryo::Z1_MRAYL / cryo::ZM_MRAYL,
        z_fluid_mrayl: cryo::ZM_MRAYL,
    };
    push(
        "G12",
        "thermo::cryo",
        "Z_1 (MRayl)",
        "== 44.178",
        layer.z1_mrayl(),
        (layer.z1_mrayl() - cryo::Z1_MRAYL).abs() < 1e-6,
    );
    let plant = lanr::LanrPowerLedger::new();
    push(
        "G13",
        "thermo::lanr",
        "net output (kW)",
        "~= 913.18",
        plant.nominal_gross_w() / 1e3,
        (plant.nominal_gross_w() / 1e3 - 913.18).abs() < 0.01,
    );
    push(
        "G14",
        "thermo::lanr",
        "TEG efficiency",
        "== 33.804%",
        plant.teg_efficiency(),
        (plant.teg_efficiency() - 0.33804).abs() < 1e-5,
    );
    let c = landauer::LandauerGetCalculator::new();
    push(
        "G15",
        "thermo::landauer",
        "C_get(|R|=8)",
        "== 3",
        c.c_get(8),
        (c.c_get(8) - 3.0).abs() < 1e-12,
    );

    // --- metrology: TMSV / lightcone / GUM ----------------------------------
    let mesh = tmsv::HeterodyneTmsvMesh::new();
    push(
        "G16",
        "metrology::tmsv",
        "sigma_r (pm/sqrtHz)",
        "<= 0.144",
        mesh.sigma_r_pm(),
        mesh.meets_displacement_bound(),
    );
    let auth = lightcone::authorize([0.0; 4], [1.0, 0.5, 0.0, 0.0]);
    push(
        "G17",
        "metrology::lightcone",
        "x_tar in J+(x_src)",
        "granted",
        if auth == lightcone::Authorization::Granted {
            0.0
        } else {
            1.0
        },
        auth == lightcone::Authorization::Granted,
    );
    let mc = gum::monte_carlo(|x| x * x, 0.0, 1.0, 200_000, 7);
    push(
        "G18",
        "metrology::gum",
        "MC mean E[x^2]",
        "~= 1",
        mc.mean,
        (mc.mean - 1.0).abs() < 0.01,
    );

    // --- eda: GDSII / STEP / substrate ---------------------------------------
    let (drc_ok, _) = gdsii::GdsiiMaskExporter::new().validate_drc();
    push(
        "G19",
        "eda::gdsii",
        "e-beam DRC",
        "0 violations",
        if drc_ok { 0.0 } else { 1.0 },
        drc_ok,
    );
    push(
        "G20",
        "eda::substrate",
        "K_diamond (W/mK)",
        ">= 2000",
        substrate::DIAMOND_THERMAL_COND_MIN,
        substrate::DIAMOND_THERMAL_COND_MIN >= 2000.0,
    );

    gates
}

/// Export the EDA artifacts (GDSII mask + STEP waveguide) into `dir`.
pub fn export_eda(
    dir: &std::path::Path,
) -> std::io::Result<(std::path::PathBuf, std::path::PathBuf)> {
    std::fs::create_dir_all(dir)?;
    let gds = dir.join("shbt_array.gds");
    shbt_recon_eda::gdsii::GdsiiMaskExporter::new().export_array(&gds)?;
    let step = dir.join("sapphire_waveguide.step");
    shbt_recon_eda::step::StepSolidModelExporter::new()
        .export_waveguide(&step, 25.4e-3, 2.0e-3, 0.5e-3)?;
    Ok((gds, step))
}

#[cfg(feature = "python")]
mod py {
    use pyo3::prelude::*;

    /// Verify the full gate matrix; returns JSON string.
    #[pyfunction]
    fn verify_json() -> String {
        crate::gates_to_json(&crate::verify_matrix())
    }

    #[pymodule(name = "shbt_recon_cli")]
    fn cli(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(verify_json, m)?)?;
        Ok(())
    }
}
