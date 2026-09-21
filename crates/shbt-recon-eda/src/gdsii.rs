//! GDSII binary mask exporter — `GdsiiMaskExporter` transferred from
//! `sys1own/shbt-exotic` (`cad_export.rs`), PyO3 layer removed.
//!
//! 8×8 InP/InGaAs SHBT collector array, 50 µm pitch, 1.5 µm airbridge spans,
//! 300 nm Nb superconducting traces; GDSII v6.0 stream, 1 pm database unit.

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Pitch between adjacent emitters (µm).
pub const EMITTER_PITCH_UM: f64 = 50.0;
/// Substrate/InP layer square dimension (µm).
pub const SUBSTRATE_SIZE_UM: f64 = 350.0;
/// Airbridge span width/height (µm).
pub const AIRBRIDGE_WIDTH_UM: f64 = 1.5;
pub const AIRBRIDGE_HEIGHT_UM: f64 = 5.0;
/// Nb trace width (nm).
pub const TRACE_WIDTH_NM: f64 = 300.0;
/// Minimum resolvable e-beam feature (nm).
pub const MIN_EBEAM_RESOLUTION_NM: f64 = 50.0;
/// Nominal acoustic impedance of the matching layer (MRayl).
pub const NOMINAL_IMPEDANCE_MRAYL: f64 = 1.1512;
/// Array dimension (8×8).
pub const ARRAY_DIM: usize = 8;

fn um_to_pm(um: f64) -> i32 {
    (um * 1_000_000.0).round() as i32
}

/// GDSII 8-byte real (base-16 exponent, bias 64, 56-bit mantissa).
fn gds_real8(value: f64) -> [u8; 8] {
    if value == 0.0 {
        return [0; 8];
    }
    let sign_byte = if value.is_sign_negative() {
        0x80u8
    } else {
        0x00u8
    };
    let mut a = value.abs();
    let mut exp: i32 = 0;
    while a >= 1.0 {
        a /= 16.0;
        exp += 1;
    }
    while a < 1.0 / 16.0 {
        a *= 16.0;
        exp -= 1;
    }
    let mut mantissa = (a * 16.0_f64.powi(14)).round() as u64;
    if mantissa >= (1u64 << 56) {
        mantissa = (1u64 << 56) - 1;
    }
    let mut bytes = [0u8; 8];
    bytes[0] = sign_byte | ((exp + 64) as u8);
    for i in 0..7 {
        bytes[7 - i] = ((mantissa >> (8 * i)) & 0xff) as u8;
    }
    bytes
}

fn write_u16_be(w: &mut impl Write, v: u16) -> std::io::Result<()> {
    w.write_all(&v.to_be_bytes())
}
fn write_i16_be(w: &mut impl Write, v: i16) -> std::io::Result<()> {
    w.write_all(&v.to_be_bytes())
}
fn write_i32_be(w: &mut impl Write, v: i32) -> std::io::Result<()> {
    w.write_all(&v.to_be_bytes())
}

fn write_padded_string(w: &mut impl Write, s: &str) -> std::io::Result<()> {
    let mut buf = s.as_bytes().to_vec();
    if buf.len() % 2 == 0 {
        buf.push(0);
    } else {
        buf.push(0);
        buf.push(0);
    }
    w.write_all(&buf)
}

fn write_record_header(
    w: &mut impl Write,
    length: u16,
    record_type: u8,
    data_type: u8,
) -> std::io::Result<()> {
    write_u16_be(w, length)?;
    w.write_all(&[record_type, data_type])
}

fn write_no_data_record(w: &mut impl Write, record_type: u8) -> std::io::Result<()> {
    write_record_header(w, 4, record_type, 0x00)
}

fn write_i16_record(w: &mut impl Write, record_type: u8, value: i16) -> std::io::Result<()> {
    write_record_header(w, 6, record_type, 0x02)?;
    write_i16_be(w, value)
}

fn write_string_record(w: &mut impl Write, record_type: u8, s: &str) -> std::io::Result<()> {
    let bytes = s.as_bytes();
    let payload = if bytes.len() % 2 == 0 {
        bytes.len() + 2
    } else {
        bytes.len() + 1
    };
    write_record_header(w, (4 + payload) as u16, record_type, 0x06)?;
    write_padded_string(w, s)
}

fn write_real_pair_record(
    w: &mut impl Write,
    record_type: u8,
    a: f64,
    b: f64,
) -> std::io::Result<()> {
    write_record_header(w, 20, record_type, 0x05)?;
    w.write_all(&gds_real8(a))?;
    w.write_all(&gds_real8(b))
}

fn write_boundary(
    w: &mut impl Write,
    layer: i16,
    datatype: i16,
    lower_left: (i32, i32),
    upper_right: (i32, i32),
) -> std::io::Result<()> {
    let (x0, y0) = lower_left;
    let (x1, y1) = upper_right;
    let points: [(i32, i32); 5] = [(x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0)];
    let n = points.len() as u16;

    write_no_data_record(w, 0x08)?; // BOUNDARY
    write_i16_record(w, 0x0D, layer)?; // LAYER
    write_i16_record(w, 0x0E, datatype)?; // DATATYPE
    write_record_header(w, 4 + 4 * 2 * n, 0x10, 0x03)?; // XY
    for (x, y) in points {
        write_i32_be(w, x)?;
        write_i32_be(w, y)?;
    }
    write_no_data_record(w, 0x11)?; // ENDEL
    Ok(())
}

/// GDSII mask exporter (`GdsiiMaskExporter` transfer).
#[derive(Clone, Debug, Default)]
pub struct GdsiiMaskExporter;

impl GdsiiMaskExporter {
    pub fn new() -> Self {
        Self
    }

    /// Electron-beam DRC: all drawn features ≥ 50 nm.
    pub fn validate_drc(&self) -> (bool, Vec<String>) {
        let mut violations = Vec::new();
        let features: [(&str, f64, f64); 3] = [
            (
                "SUBSTRATE_INP",
                SUBSTRATE_SIZE_UM * 1000.0,
                SUBSTRATE_SIZE_UM * 1000.0,
            ),
            (
                "AIRBRIDGE_SPAN",
                AIRBRIDGE_WIDTH_UM * 1000.0,
                AIRBRIDGE_HEIGHT_UM * 1000.0,
            ),
            ("MET_NB_TRACE", TRACE_WIDTH_NM, AIRBRIDGE_HEIGHT_UM * 1000.0),
        ];
        for (name, w_nm, h_nm) in features {
            if w_nm < MIN_EBEAM_RESOLUTION_NM {
                violations.push(format!(
                    "DRC violation: {name} width {w_nm:.3} nm below {MIN_EBEAM_RESOLUTION_NM:.3} nm"
                ));
            }
            if h_nm < MIN_EBEAM_RESOLUTION_NM {
                violations.push(format!(
                    "DRC violation: {name} height {h_nm:.3} nm below {MIN_EBEAM_RESOLUTION_NM:.3} nm"
                ));
            }
        }
        (violations.is_empty(), violations)
    }

    /// Export the 8×8 InP/InGaAs SHBT array as a GDSII v6.0 stream.
    pub fn export_array<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut f = File::create(path)?;

        write_i16_record(&mut f, 0x00, 600)?; // HEADER v600
        write_record_header(&mut f, 28, 0x01, 0x02)?; // BGNLIB
        for _ in 0..12 {
            write_i16_be(&mut f, 0)?;
        }
        write_string_record(&mut f, 0x02, "shbt_mask.gds")?; // LIBNAME
        write_real_pair_record(&mut f, 0x03, 1.0, 1.0e-12)?; // UNITS (1 pm)
        write_record_header(&mut f, 28, 0x05, 0x02)?; // BGNSTR
        for _ in 0..12 {
            write_i16_be(&mut f, 0)?;
        }
        write_string_record(&mut f, 0x06, "shbt_array")?; // STRNAME

        // Layer 10: SUBSTRATE_INP (350 µm square)
        let sub_half = um_to_pm(SUBSTRATE_SIZE_UM);
        write_boundary(&mut f, 10, 0, (0, 0), (sub_half, sub_half))?;

        let pitch_pm = um_to_pm(EMITTER_PITCH_UM);
        let air_half_x = um_to_pm(AIRBRIDGE_WIDTH_UM / 2.0);
        let air_half_y = um_to_pm(AIRBRIDGE_HEIGHT_UM / 2.0);
        let trace_half_x = um_to_pm(TRACE_WIDTH_NM / 1000.0 / 2.0);
        let trace_half_y = air_half_y;

        for u in 0..ARRAY_DIM {
            for v in 0..ARRAY_DIM {
                let cx = u as i32 * pitch_pm + pitch_pm / 2;
                let cy = v as i32 * pitch_pm + pitch_pm / 2;
                // Layer 20: AIRBRIDGE_SPAN (1.5 × 5.0 µm)
                write_boundary(
                    &mut f,
                    20,
                    0,
                    (cx - air_half_x, cy - air_half_y),
                    (cx + air_half_x, cy + air_half_y),
                )?;
                // Layer 25: MET_NB_TRACE (300 nm × 5.0 µm)
                write_boundary(
                    &mut f,
                    25,
                    0,
                    (cx - trace_half_x, cy - trace_half_y),
                    (cx + trace_half_x, cy + trace_half_y),
                )?;
            }
        }

        write_no_data_record(&mut f, 0x07)?; // ENDSTR
        write_no_data_record(&mut f, 0x04)?; // ENDLIB
        f.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drc_passes_nominal_layout() {
        let (ok, v) = GdsiiMaskExporter::new().validate_drc();
        assert!(ok, "{v:?}");
    }

    #[test]
    fn gds_stream_has_valid_header() {
        let dir = std::env::temp_dir();
        let path = dir.join("shbt_recon_test_mask.gds");
        GdsiiMaskExporter::new().export_array(&path).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        // HEADER record: len 6, type 0x00, i16 version 600.
        assert_eq!(&bytes[..6], &[0x00, 0x06, 0x00, 0x02, 0x02, 0x58]);
        // ENDLIB trailer present.
        assert_eq!(&bytes[bytes.len() - 4..], &[0x00, 0x04, 0x04, 0x00]);
        std::fs::remove_file(&path).ok();
    }
}
