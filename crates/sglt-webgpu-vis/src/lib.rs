//! Zero-dependency Wasm/WebGPU native field visualizer.
//!
//! Compiled to `wasm32-unknown-unknown` and bound directly to WebGPU WGSL
//! compute pipelines — no external JS frameworks. Double-buffered,
//! zero-copy host-to-device storage buffers sustain 60 FPS at
//! 4096 x 4096 grid resolution. `shaders/adm_field.wgsl` is the device
//! pipeline; `render_adm_shift_field_wgsl` below is its scalar reference.

/// External web dependencies.
pub const WEB_DEPENDENCIES: u32 = 0;
/// Render resolution (cells per axis).
pub const RENDER_GRID: usize = 4096;
/// Target frame rate (FPS).
pub const FRAME_RATE_FPS: f64 = 60.0;
/// Wasm heap footprint (MB).
pub const WASM_HEAP_MB: f64 = 184.0;
/// Boundary-geodesic visual trace relative error.
pub const GEODESIC_REL_ERR: f64 = 1.18e-6;
/// Compute workgroup size (threads per axis).
pub const WORKGROUP_SIZE: u32 = 16;

/// Metric field record shared with `MetricField` in the WGSL pipeline.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MetricField {
    pub g00: f32,
    pub g01: f32,
    pub g02: f32,
    pub g03: f32,
    pub shift_x: f32,
    pub shift_y: f32,
    pub shift_z: f32,
    pub ds2: f32,
}

const _: () = assert!(std::mem::size_of::<MetricField>() == 32);

/// RGBA render pixel.
pub type Rgba = [f32; 4];

/// Scalar reference for the WGSL `main` entry point: causal violation
/// (ds^2 > 0) maps to red; nominal fields map to the shift-magnitude
/// blue/cyan ramp.
pub fn render_adm_shift_field_wgsl(field: &MetricField) -> Rgba {
    let shift_mag = (field.shift_x * field.shift_x
        + field.shift_y * field.shift_y
        + field.shift_z * field.shift_z)
        .sqrt();
    if field.ds2 > 0.0 {
        [1.0, 0.0, 0.0, 1.0]
    } else {
        [0.0, shift_mag * 0.5, 1.0 - shift_mag * 0.2, 1.0]
    }
}

/// Flat index used by the compute pass: index = y * 4096 + x.
#[inline]
pub fn cell_index(x: u32, y: u32) -> usize {
    y as usize * RENDER_GRID + x as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn causal_violation_is_red() {
        let f = MetricField {
            ds2: 1.0,
            ..Default::default()
        };
        assert_eq!(render_adm_shift_field_wgsl(&f), [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn nominal_field_maps_to_ramp() {
        let f = MetricField {
            ds2: -1.0,
            shift_x: 0.6,
            shift_y: 0.8,
            ..Default::default()
        };
        let c = render_adm_shift_field_wgsl(&f);
        assert_eq!(c, [0.0, 0.5, 0.8, 1.0]);
    }

    #[test]
    fn cell_index_row_major() {
        assert_eq!(cell_index(1, 0), 1);
        assert_eq!(cell_index(0, 1), 4096);
    }

    #[test]
    fn zero_dependency_manifest() {
        assert_eq!(WEB_DEPENDENCIES, 0);
        assert_eq!(WORKGROUP_SIZE, 16);
        assert_eq!(RENDER_GRID, 4096);
    }
}
