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

/// Elf32 Program header
#[repr(C)]
pub struct Elf32_Phdr {
    p_type: Elf32_Word,
    p_offset: Elf32_Off,
    p_vaddr: Elf32_Addr,
    p_paddr: Elf32_Addr,
    p_filesz: Elf32_Word,
    p_memsz: Elf32_Word,
    p_flags: Elf32_Word,
    p_align: Elf32_Word,
}

/// Elf32 section header
#[repr(C)]
pub struct Elf32_Shrdr {
    sh_name: Elf32_Word,
    sh_type: Elf32_Word,
    sh_flags: Elf32_Word,
    sh_addr: Elf32_Addr,
    sh_offset: Elf32_Off,
    sh_size: Elf32_Word,
    sh_link: Elf32_Word,
    sh_info: Elf32_Word,
    sh_addalign: Elf32_Word,
    sh_entsize: Elf32_Word,
}
