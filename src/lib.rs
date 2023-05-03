pub mod read;
pub mod write;

pub type Addr = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentType {
    Text,
    Data,
    RoData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    pub addr: Addr,
    pub ty: SegmentType,
    pub data: Vec<u8>,
    pub zero_padding: u64,
    pub load: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfInfo {
    pub segments: Vec<Segment>,
    pub entry: Addr,
    pub is_64: bool,
    pub endian: object::Endianness,
    pub architecture: object::Architecture,
    pub kind: object::ObjectKind,
}

/// Strip trailing zeroes from all segments.
///
/// The removes zeroes are kept track of in the `zero_padding` field.
pub fn strip_zeroes(info: &mut ElfInfo) -> usize {
    let mut total_stripped = 0;

    for seg in &mut info.segments {
        let trailing_zeros = seg.data.iter().rev().take_while(|x| **x == 0).count();

        if trailing_zeros > 0 {
            seg.data.truncate(seg.data.len() - trailing_zeros);

            seg.zero_padding += trailing_zeros as u64;
        }

        total_stripped += trailing_zeros;
    }

    total_stripped
}
