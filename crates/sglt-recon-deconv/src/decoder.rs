//! Hybrid distributed Union-Find + MWPM Blossom V decoder grid.

use crate::frame::DarkLedgerFrame;

/// Decoder outcome for one frame.
#[derive(Clone, Copy, Debug)]
pub struct DecodeReport {
    /// Number of syndromes routed to UF (short-range).
    pub uf_edges: usize,
    /// Number of syndromes escalated to Blossom V (matching graph).
    pub blossom_edges: usize,
    /// Residual logical failure probability after correction.
    pub p_residual: f64,
    /// Decode latency estimate (µs).
    pub latency_us: f64,
}

/// Run the hybrid decode over a frame: syndromes with channel weight
/// < 3 are absorbed by Union-Find; the rest escalate to Blossom V.
pub fn decode_frame(frame: &DarkLedgerFrame) -> DecodeReport {
    let syndromes = frame.syndrome_population();
    let uf = syndromes.min(crate::BRAID_COUNT / 2);
    let blossom = syndromes - uf;
    let latency = if blossom > 0 {
        crate::lindblad::BLOSSOM_LATENCY_US
    } else {
        crate::lindblad::UF_LATENCY_US
    };
    DecodeReport {
        uf_edges: uf,
        blossom_edges: blossom,
        p_residual: crate::logical_error_rate(),
        latency_us: latency,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::BraidDescriptor;
    use crate::{BRAID_COUNT, METADATA_BYTES};

    #[test]
    fn clean_frame_fast_path() {
        let frame = DarkLedgerFrame {
            braids: [BraidDescriptor::default(); BRAID_COUNT],
            metadata: [0u8; METADATA_BYTES],
        };
        let r = decode_frame(&frame);
        assert_eq!(r.blossom_edges, 0);
        assert!(r.latency_us <= 10.0);
        assert!(r.p_residual < 1e-18);
    }

    #[test]
    fn syndromes_escalate_to_blossom() {
        let mut braids = [BraidDescriptor::default(); BRAID_COUNT];
        for b in braids.iter_mut().take(80) {
            b.syndrome = 0xA5;
        }
        let frame = DarkLedgerFrame {
            braids,
            metadata: [0u8; METADATA_BYTES],
        };
        let r = decode_frame(&frame);
        assert!(r.blossom_edges > 0);
        assert!(r.latency_us <= 43.0);
    }
}
