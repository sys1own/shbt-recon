//! 1,472-byte dark-ledger SRAM TQEC frame: 992 B braid descriptor
//! payload (124 × 8 B) + 480 B SECDED/checkpoint metadata.

use crate::{BRAID_COUNT, BRAID_PAYLOAD_BYTES, FRAME_BYTES, METADATA_BYTES};

/// One Fibonacci braid descriptor (8 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BraidDescriptor {
    /// Braid worldline identifier.
    pub id: u16,
    /// Fusion channel flags (1 or τ).
    pub channel: u8,
    /// SECDED syndrome bits.
    pub syndrome: u8,
    /// Braid phase (radians, fixed-point ×2^-32).
    pub phase: u32,
}

/// Dark ledger frame as laid out in SRAM at 0x280..0x840.
#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct DarkLedgerFrame {
    /// 992 B braid descriptor array.
    pub braids: [BraidDescriptor; BRAID_COUNT],
    /// 480 B SECDED parity + checkpoint metadata.
    pub metadata: [u8; METADATA_BYTES],
}

const _: () = assert!(std::mem::size_of::<DarkLedgerFrame>() == FRAME_BYTES);
const _: () = assert!(std::mem::size_of::<BraidDescriptor>() * BRAID_COUNT == BRAID_PAYLOAD_BYTES);

impl DarkLedgerFrame {
    /// Parse a raw 1,472-byte buffer into a typed frame reference.
    /// Returns None on bad length or misalignment.
    pub fn parse(buf: &[u8]) -> Option<&Self> {
        if buf.len() != FRAME_BYTES || (buf.as_ptr() as usize) % std::mem::align_of::<Self>() != 0 {
            return None;
        }
        Some(unsafe { &*(buf.as_ptr() as *const Self) })
    }

    /// Count of descriptors carrying a nonzero syndrome.
    pub fn syndrome_population(&self) -> usize {
        self.braids.iter().filter(|b| b.syndrome != 0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_geometry() {
        assert_eq!(std::mem::size_of::<DarkLedgerFrame>(), 1472);
        assert_eq!(std::mem::size_of::<BraidDescriptor>(), 8);
        assert_eq!(BRAID_PAYLOAD_BYTES, 992);
    }

    #[test]
    fn parse_roundtrip() {
        let frame = DarkLedgerFrame {
            braids: [BraidDescriptor::default(); BRAID_COUNT],
            metadata: [0u8; METADATA_BYTES],
        };
        let bytes =
            unsafe { std::slice::from_raw_parts(&frame as *const _ as *const u8, FRAME_BYTES) };
        let parsed = DarkLedgerFrame::parse(bytes).unwrap();
        assert_eq!(parsed.syndrome_population(), 0);
        assert!(DarkLedgerFrame::parse(&bytes[..100]).is_none());
    }
}
