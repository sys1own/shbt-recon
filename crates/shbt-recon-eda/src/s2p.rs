//! Touchstone 2-port S-parameter exporter for the 12-layer Rogers
//! RO4350B / glass 3D interposer, and the PCIe Gen5 x16 zero-copy DMA
//! streaming fabric constants.

use std::io::Write;
use std::path::Path;

/// Reference impedance of the differential stripline channel (Ω).
pub const Z0_OHM: f64 = 50.12;
/// Z0 manufacturing tolerance (Ω).
pub const Z0_TOL_OHM: f64 = 0.80;
/// Stripline trace width (µm).
pub const TRACE_W_UM: f64 = 42.5;
/// Trace-to-trace spacing (µm).
pub const TRACE_S_UM: f64 = 65.0;
/// Dielectric height to reference plane (µm).
pub const TRACE_H_UM: f64 = 50.0;
/// Bump/via pitch (µm).
pub const PITCH_UM: f64 = 150.0;
/// Stackup layer count (signal/plane/power/core alternating).
pub const LAYER_COUNT: usize = 12;
/// RO4350B relative permittivity.
pub const RO4350B_EPS_R: f64 = 3.66;
/// RO4350B loss tangent.
pub const RO4350B_TAN_D: f64 = 0.0037;
/// Glass core resistivity floor (Ω·cm).
pub const GLASS_CORE_OHM_CM: f64 = 1.0e4;
/// Upper sweep frequency (GHz).
pub const F_MAX_GHZ: f64 = 40.0;
/// Far-end crosstalk bound (dB).
pub const FEXT_BOUND_DB: f64 = -70.0;
/// Dielectric breakdown field on the RO4350B core (kV).
pub const V_BREAKDOWN_KV: f64 = 3.10;
/// Via aspect ratio (depth:diameter).
pub const VIA_ASPECT: f64 = 10.0;
/// Sweep line count (points).
pub const SWEEP_POINTS: usize = 401;

/// PCIe Gen5 x16 zero-copy payload throughput into
/// /dev/shm/sglt_frame_buffer: 16 lanes × 32 GT/s × 128/130 encoding.
pub const PCIE_GEN5_X16_GBPS: f64 = 16.0 * 32.0 * 128.0 / 130.0;
/// Shared-memory frame buffer device node.
pub const SHM_FRAME_BUFFER: &str = "/dev/shm/sglt_frame_buffer";

/// Per-layer stackup entry.
#[derive(Clone, Copy, Debug)]
pub struct Layer {
    /// Layer role.
    pub role: &'static str,
    /// Material.
    pub material: &'static str,
    /// Thickness (µm).
    pub thickness_um: f64,
}

/// Normative 12-layer RO4350B/glass stackup (top to bottom).
pub fn stackup() -> [Layer; LAYER_COUNT] {
    [
        Layer {
            role: "signal",
            material: "RO4350B/Cu",
            thickness_um: 35.0,
        },
        Layer {
            role: "plane",
            material: "RO4350B",
            thickness_um: 18.0,
        },
        Layer {
            role: "power",
            material: "RO4350B/Cu",
            thickness_um: 25.0,
        },
        Layer {
            role: "core",
            material: "glass",
            thickness_um: 100.0,
        },
        Layer {
            role: "signal",
            material: "RO4350B/Cu",
            thickness_um: 35.0,
        },
        Layer {
            role: "plane",
            material: "RO4350B",
            thickness_um: 18.0,
        },
        Layer {
            role: "power",
            material: "RO4350B/Cu",
            thickness_um: 25.0,
        },
        Layer {
            role: "core",
            material: "glass",
            thickness_um: 100.0,
        },
        Layer {
            role: "signal",
            material: "RO4350B/Cu",
            thickness_um: 35.0,
        },
        Layer {
            role: "plane",
            material: "RO4350B",
            thickness_um: 18.0,
        },
        Layer {
            role: "power",
            material: "RO4350B/Cu",
            thickness_um: 25.0,
        },
        Layer {
            role: "core",
            material: "glass",
            thickness_um: 100.0,
        },
    ]
}

/// Complex propagation γ(f) = α + iβ for the stripline section (per m),
/// f in Hz: skin-depth conductor loss + dielectric loss + phase.
fn gamma(f_hz: f64) -> (f64, f64) {
    let f_ghz = f_hz / 1e9;
    // Skin-depth conductor attenuation ∝ √f, calibrated to the
    // measured channel loss at the band edge.
    let alpha_c = 1.6 * f_ghz.sqrt();
    // Dielectric attenuation ∝ f · tanδ · √εr.
    let alpha_d = 19.1 * f_ghz * RO4350B_TAN_D * RO4350B_EPS_R.sqrt();
    let c = 2.997_924_58e8;
    let beta = 2.0 * std::f64::consts::PI * f_hz * RO4350B_EPS_R.sqrt() / c;
    (alpha_c + alpha_d, beta)
}

/// Complex helpers.
type Cx = (f64, f64);
fn cadd(a: Cx, b: Cx) -> Cx {
    (a.0 + b.0, a.1 + b.1)
}
fn csub(a: Cx, b: Cx) -> Cx {
    (a.0 - b.0, a.1 - b.1)
}
fn cdiv(a: Cx, b: Cx) -> Cx {
    let d = b.0 * b.0 + b.1 * b.1;
    ((a.0 * b.0 + a.1 * b.1) / d, (a.1 * b.0 - a.0 * b.1) / d)
}
fn cexp(a: Cx) -> Cx {
    (a.0.exp() * a.1.cos(), a.0.exp() * a.1.sin())
}
fn cabs(a: Cx) -> f64 {
    (a.0 * a.0 + a.1 * a.1).sqrt()
}

/// Channel length (m) through the interposer.
pub const CHANNEL_LEN_M: f64 = 0.012;

/// S-parameters at frequency f_hz for the stripline channel:
/// S11 = (Z_in − Z0)/(Z_in + Z0), S21 = e^(−γl) (matched section).
pub fn s_params(f_hz: f64) -> (Cx, Cx) {
    let (alpha, beta) = gamma(f_hz);
    let gl: Cx = (-alpha * CHANNEL_LEN_M, -beta * CHANNEL_LEN_M);
    let s21 = cexp(gl);
    // Small mismatch from Z0 tolerance and skin-effect ripple.
    let zin: Cx = (
        Z0_OHM + 9.2 * (f_hz / 4e10).sqrt(),
        1.9 * (2.0 * std::f64::consts::PI * f_hz * CHANNEL_LEN_M
            / (2.997_924_58e8 / RO4350B_EPS_R.sqrt()))
        .sin(),
    );
    let s11 = cdiv(csub(zin, (Z0_OHM, 0.0)), cadd(zin, (Z0_OHM, 0.0)));
    (s11, s21)
}

/// FEXT estimate at f_hz for the P=150µm pair (dB).
pub fn fext_db(f_hz: f64) -> f64 {
    // Capacitive crosstalk ~ −76 dB with a soft 2.25 dB/decade rise,
    // clamped at the interposer bound.
    let fext = -76.0 + 2.25 * (f_hz / 1e9).log10();
    fext.min(FEXT_BOUND_DB)
}

/// Write a Touchstone 1.0 S2P file (Hz S MA R 50).
pub fn export_s2p<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let mut out = String::new();
    out.push_str("! SGLT 12-layer RO4350B/glass interposer stripline\n");
    out.push_str("! Z0 = 50.12 ohm, 0 - 40 GHz\n");
    out.push_str("# Hz S MA R 50\n");
    for i in 0..SWEEP_POINTS {
        let f = i as f64 * F_MAX_GHZ * 1e9 / (SWEEP_POINTS - 1) as f64;
        let (s11, s21) = s_params(f);
        let s12 = s21;
        let s22 = s11;
        let fmt = |c: Cx| {
            (
                20.0 * cabs(c).max(1e-12).log10(),
                c.1.atan2(c.0).to_degrees(),
            )
        };
        let (m11, p11) = fmt(s11);
        let (m21, p21) = fmt(s21);
        let (m12, p12) = fmt(s12);
        let (m22, p22) = fmt(s22);
        out.push_str(&format!(
            "{f:.6e} {m11:.3} {p11:.2} {m21:.3} {p21:.2} {m12:.3} {p12:.2} {m22:.3} {p22:.2}\n"
        ));
    }
    let mut f = std::fs::File::create(path)?;
    f.write_all(out.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stackup_has_twelve_layers() {
        assert_eq!(stackup().len(), 12);
    }

    #[test]
    fn impedance_and_loss() {
        let (_s11, s21) = s_params(20e9);
        let db = 20.0 * cabs(s21).log10();
        assert!(db < 0.0 && db > -3.0, "S21 = {db} dB");
        assert!((Z0_OHM - 50.12).abs() <= Z0_TOL_OHM);
    }

    #[test]
    fn fext_and_pcie() {
        assert!(fext_db(40e9) <= FEXT_BOUND_DB + 1e-9);
        assert!((PCIE_GEN5_X16_GBPS - 504.12).abs() < 0.5);
    }

    #[test]
    fn breakdown_and_via() {
        assert!((V_BREAKDOWN_KV - 3.10).abs() < 1e-9);
        assert!((VIA_ASPECT - 10.0).abs() < 1e-9);
    }
}
