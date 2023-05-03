use object::elf;
use object::{Object, ObjectSegment, SegmentFlags};

use crate::{ElfInfo, Segment, SegmentType};

/// Reads in an ELF from bytes.
///
/// Any errors during reading will be returned.
///
/// This reading is *lossy*, only `LOAD` segments are kept as well as the file header.
pub fn read_elf_file(bytes: &[u8]) -> Result<ElfInfo, object::Error> {
    let file = object::File::parse(bytes)?;

    let mut info = ElfInfo {
        segments: vec![],
        entry: file.entry(),
        is_64: file.is_64(),
        endian: file.endianness(),
        architecture: file.architecture(),
        kind: file.kind(),
    };

    for seg in file.segments() {
        let SegmentFlags::Elf { p_flags } = seg.flags() else { continue };

        const TEXT_FLAGS: u32 = elf::PF_X | elf::PF_R;
        const DATA_FLAGS: u32 = elf::PF_R | elf::PF_W;
        const RODATA_FLAGS: u32 = elf::PF_R;
        let ty = match p_flags {
            TEXT_FLAGS => SegmentType::Text,
            DATA_FLAGS => SegmentType::Data,
            RODATA_FLAGS => SegmentType::RoData,
            _ => continue,
        };

        let memsize = seg.size();
        let data = seg.data()?;

        let file_size = data.len() as u64;
        let padding = memsize - file_size;

        info.segments.push(Segment {
            addr: seg.address(),
            ty,
            data: data.to_vec(),
            zero_padding: padding,
            load: true,
        });
    }

    Ok(info)
}
