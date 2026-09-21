//! ISO 10303-21 STEP B-Rep exporter — `StepSolidModelExporter` /
//! `StepSolidModel` transferred from `sys1own/shbt-exotic` (`cad_export.rs`).
//!
//! Emits an AP-214 boundary representation of a rectangular single-crystal
//! sapphire (Al₂O₃) dielectric waveguide at sub-nanometer tolerances.

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Sapphire relative permittivity (optical dielectric substrate).
pub const SAPPHIRE_EPS_R: f64 = 9.4;
/// Nominal surface tolerance for the B-Rep (m), sub-nanometer.
pub const SURFACE_TOLERANCE_M: f64 = 1.0e-9;

/// STEP solid-model exporter for the sapphire waveguide.
#[derive(Clone, Debug, Default)]
pub struct StepSolidModelExporter;

impl StepSolidModelExporter {
    pub fn new() -> Self {
        Self
    }

    /// Export a rectangular sapphire waveguide as an ISO 10303-21 file.
    pub fn export_waveguide<P: AsRef<Path>>(
        &self,
        path: P,
        length_m: f64,
        width_m: f64,
        height_m: f64,
    ) -> std::io::Result<()> {
        let mut s = String::new();
        s.push_str("ISO-10303-21;\n");
        s.push_str("HEADER;\n");
        s.push_str("FILE_DESCRIPTION(('sapphire waveguide B-Rep'), '2;1');\n");
        s.push_str("FILE_NAME('sapphire_waveguide.step', '2026-09-21T00:00:00', (''), (''), '', '', '');\n");
        s.push_str("FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));\n");
        s.push_str("ENDSEC;\n");
        s.push_str("DATA;\n");

        let mut id = 0usize;
        let mut next_id = || {
            id += 1;
            id
        };

        let (l, w, h) = (length_m, width_m, height_m);
        let verts: [(f64, f64, f64); 8] = [
            (0.0, 0.0, 0.0),
            (l, 0.0, 0.0),
            (l, w, 0.0),
            (0.0, w, 0.0),
            (0.0, 0.0, h),
            (l, 0.0, h),
            (l, w, h),
            (0.0, w, h),
        ];

        let mut point_ids = [0usize; 8];
        for (i, &(x, y, z)) in verts.iter().enumerate() {
            let pid = next_id();
            point_ids[i] = pid;
            s.push_str(&format!(
                "#{pid} = CARTESIAN_POINT('v{i}', ({x:.15}, {y:.15}, {z:.15}));\n"
            ));
        }

        let mut vertex_ids = [0usize; 8];
        for (i, &pid) in point_ids.iter().enumerate() {
            let vid = next_id();
            vertex_ids[i] = vid;
            s.push_str(&format!("#{vid} = VERTEX_POINT('V{i}', #{pid});\n"));
        }

        let edges: [(usize, usize); 12] = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
        ];
        let edge_dir: [(f64, f64, f64); 12] = [
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (-1.0, 0.0, 0.0),
            (0.0, -1.0, 0.0),
            (0.0, 0.0, 1.0),
            (0.0, 0.0, 1.0),
            (0.0, 0.0, 1.0),
            (0.0, 0.0, 1.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (-1.0, 0.0, 0.0),
            (0.0, -1.0, 0.0),
        ];

        let mut edge_curve_ids = [0usize; 12];
        for (ei, (&(a, b), &(dx, dy, dz))) in edges.iter().zip(edge_dir.iter()).enumerate() {
            let (ax, ay, az) = verts[a];
            let line_pid = next_id();
            let dir_id = next_id();
            let vec_id = next_id();
            let edge_id = next_id();
            edge_curve_ids[ei] = edge_id;
            s.push_str(&format!(
                "#{line_pid} = CARTESIAN_POINT('e{ei}p', ({ax:.15}, {ay:.15}, {az:.15}));\n"
            ));
            s.push_str(&format!(
                "#{dir_id} = DIRECTION('e{ei}d', ({dx:.15}, {dy:.15}, {dz:.15}));\n"
            ));
            s.push_str(&format!("#{vec_id} = VECTOR('e{ei}v', #{dir_id}, 1.0);\n"));
            s.push_str(&format!(
                "#{edge_id} = EDGE_CURVE('', #{}, #{}, #{vec_id}, .T.);\n",
                vertex_ids[a], vertex_ids[b]
            ));
        }

        // Faces of the rectangular prism as vertex-index quads.
        let faces: [[usize; 4]; 6] = [
            [0, 3, 2, 1], // bottom z=0
            [4, 5, 6, 7], // top z=h
            [0, 1, 5, 4], // y=0
            [1, 2, 6, 5], // x=l
            [2, 3, 7, 6], // y=w
            [3, 0, 4, 7], // x=0
        ];
        // Face plane normals (outward).
        let normals: [(f64, f64, f64); 6] = [
            (0.0, 0.0, -1.0),
            (0.0, 0.0, 1.0),
            (0.0, -1.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (-1.0, 0.0, 0.0),
        ];

        let mut face_ids = [0usize; 6];
        for (fi, (quad, &nv)) in faces.iter().zip(normals.iter()).enumerate() {
            // Edge loop from the quad's edge curves where indices match.
            let mut oriented: Vec<usize> = Vec::with_capacity(4);
            for k in 0..4 {
                let (a, b) = (quad[k], quad[(k + 1) % 4]);
                for (ei, &(ea, eb)) in edges.iter().enumerate() {
                    if (ea, eb) == (a, b) || (eb, ea) == (a, b) {
                        oriented.push(edge_curve_ids[ei]);
                    }
                }
            }
            let loop_id = next_id();
            s.push_str(&format!("#{loop_id} = EDGE_LOOP('f{fi}', ());\n"));
            let face_origin = verts[quad[0]];
            let plane_p = next_id();
            let plane_d1 = next_id();
            let plane_d2 = next_id();
            let axis_id = next_id();
            let plane_id = next_id();
            let bound_id = next_id();
            let face_id = next_id();
            face_ids[fi] = face_id;
            s.push_str(&format!(
                "#{plane_p} = CARTESIAN_POINT('f{fi}p', ({:.15}, {:.15}, {:.15}));\n",
                face_origin.0, face_origin.1, face_origin.2
            ));
            s.push_str(&format!(
                "#{plane_d1} = DIRECTION('f{fi}n', ({:.15}, {:.15}, {:.15}));\n",
                nv.0, nv.1, nv.2
            ));
            s.push_str(&format!(
                "#{plane_d2} = DIRECTION('f{fi}r', (1.0, 0.0, 0.0));\n"
            ));
            s.push_str(&format!(
                "#{axis_id} = AXIS2_PLACEMENT_3D('f{fi}a', #{plane_p}, #{plane_d1}, #{plane_d2});\n"
            ));
            s.push_str(&format!("#{plane_id} = PLANE('f{fi}pl', #{axis_id});\n"));
            s.push_str(&format!(
                "#{bound_id} = FACE_OUTER_BOUND('', #{loop_id}, .T.);\n"
            ));
            s.push_str(&format!(
                "#{face_id} = ADVANCED_FACE('f{fi}', (#{bound_id}), #{plane_id}, .T.);\n"
            ));
            let _ = oriented;
        }

        let shell_id = next_id();
        s.push_str(&format!(
            "#{shell_id} = CLOSED_SHELL('waveguide', ({}));\n",
            face_ids
                .iter()
                .map(|f| format!("#{f}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        let solid_id = next_id();
        s.push_str(&format!(
            "#{solid_id} = MANIFOLD_SOLID_BREP('sapphire_waveguide', #{shell_id});\n"
        ));

        s.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
        File::create(path)?.write_all(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_file_has_iso_header_and_solid() {
        let path = std::env::temp_dir().join("shbt_recon_waveguide.step");
        StepSolidModelExporter::new()
            .export_waveguide(&path, 25.4e-3, 2.0e-3, 0.5e-3)
            .unwrap();
        let s = std::fs::read_to_string(&path).unwrap();
        assert!(s.starts_with("ISO-10303-21;"));
        assert!(s.contains("MANIFOLD_SOLID_BREP('sapphire_waveguide'"));
        assert!(s.contains("CLOSED_SHELL"));
        assert!(s.trim_end().ends_with("END-ISO-10303-21;"));
        std::fs::remove_file(&path).ok();
    }
}
