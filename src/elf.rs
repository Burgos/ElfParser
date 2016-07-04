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

