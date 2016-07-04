/// Elf reader module
///

const EI_NIDENT: usize = 4;

// ELF32 types
#[allow(non_camel_case_types)]
type Elf32_Addr = u32;
#[allow(non_camel_case_types)]
type Elf32_Half = u16;
#[allow(non_camel_case_types)]
type Elf32_Lworld = u64;
#[allow(non_camel_case_types)]
type Elf32_Off = u32;
#[allow(non_camel_case_types)]
type Elf32_Sword = i32;
#[allow(non_camel_case_types)]
type Elf32_Word = u32;

/// Elf32 header.
/// Source: http://www.freebsd.org/cgi/man.cgi?elf(5)
#[repr(C)]
pub struct Elf32_Ehdr {
    e_ident: [u8; EI_NIDENT],
    e_type: Elf32_Half,
    e_machine: Elf32_Half,
    e_version: Elf32_Word,
    e_entry: Elf32_Addr,
    e_phoff: Elf32_Off,
    e_shoff: Elf32_Off,
    e_flags: Elf32_Word,
    e_ehsize: Elf32_Half,
    e_phentsize: Elf32_Half,
    e_phnum: Elf32_Half,
    e_shentsize: Elf32_Half,
    e_shnum: Elf32_Half,
    e_shstrndx: Elf32_Half,
}
