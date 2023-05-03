use object::write::elf::{FileHeader, ProgramHeader, Writer};
use object::{elf, Architecture};

use crate::{ElfInfo, SegmentType};

/// Creates an ELF in memory from an [`ElfInfo`]
pub fn create_elf_file(info: &ElfInfo) -> Vec<u8> {
    let mut buffer = vec![];
    let mut writer = Writer::new(info.endian, info.is_64, &mut buffer);
    writer.reserve_file_header();

    writer.reserve_program_headers(info.segments.len() as u32);

    let mut segment_offsets = Vec::with_capacity(info.segments.len());
    for seg in &info.segments {
        let offset = if seg.load {
            writer.reserve(seg.data.len(), 64)
        } else {
            0
        };

        segment_offsets.push(offset);
    }

    // Write file header

    let e_type = match info.kind {
        object::ObjectKind::Executable => elf::ET_EXEC,
        _ => panic!("only EXEC files are supported for now"),
    };
    let e_machine = match info.architecture {
        Architecture::Aarch64 => elf::EM_AARCH64,
        Architecture::Arm => elf::EM_ARM,
        Architecture::Avr => elf::EM_AVR,
        Architecture::Bpf => elf::EM_BPF,
        Architecture::I386 => elf::EM_386,
        Architecture::X86_64 => elf::EM_X86_64,
        Architecture::X86_64_X32 => elf::EM_X86_64,
        Architecture::Hexagon => elf::EM_HEXAGON,
        Architecture::LoongArch64 => elf::EM_LOONGARCH,
        Architecture::Mips => elf::EM_MIPS,
        Architecture::Mips64 => elf::EM_MIPS,
        Architecture::Msp430 => elf::EM_MSP430,
        Architecture::PowerPc => elf::EM_PPC,
        Architecture::PowerPc64 => elf::EM_PPC64,
        Architecture::Riscv32 => elf::EM_RISCV,
        Architecture::Riscv64 => elf::EM_RISCV,
        Architecture::S390x => elf::EM_S390,
        Architecture::Sparc64 => elf::EM_SPARCV9,
        _ => {
            panic!()
        }
    };
    let e_flags = 0;

    writer
        .write_file_header(&FileHeader {
            os_abi: elf::ELFOSABI_NONE,
            abi_version: 0,
            e_type,
            e_machine,
            e_entry: info.entry,
            e_flags,
        })
        .unwrap();

    // Write segments

    for (i, seg) in info.segments.iter().enumerate() {
        let p_type = if seg.load { elf::PT_LOAD } else { elf::PT_NULL };
        let p_flags = match &seg.ty {
            SegmentType::Text => elf::PF_X | elf::PF_R,
            SegmentType::Data => elf::PF_R | elf::PF_W,
            SegmentType::RoData => elf::PF_R,
        };

        writer.write_program_header(&ProgramHeader {
            p_type,
            p_flags,
            p_offset: segment_offsets[i] as u64,
            p_vaddr: seg.addr,
            p_paddr: seg.addr,
            p_filesz: if seg.load { seg.data.len() as u64 } else { 0 },
            p_memsz: seg.data.len() as u64 + seg.zero_padding,
            p_align: 64,
        });
    }

    for seg in &info.segments {
        if seg.load && !seg.data.is_empty() {
            writer.write_align(64);
            writer.write(&seg.data);
        }
    }

    assert_eq!(writer.reserved_len(), writer.len());

    buffer
}
