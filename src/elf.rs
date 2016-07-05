/// Copyright (c) 2000, 2001, 2008, 2011, David E. O'Brien
/// Copyright (c) 1998 John D. Polstra.
/// Copyright (c) 2016 Nemanja Boric
/// All rights reserved
///
/// Redistribution and use in source and binary forms, with or without
/// modification, are permitted provided that the following conditions
/// are met:
/// 1. Redistributions of source code must retain the above copyright
///    notice, this list of conditions and the following disclaimer.
/// 2. Redistributions in binary form must reproduce the above copyright
///    notice, this list of conditions and the following disclaimer in the
///    documentation and/or other materials provided with the distribution.
///
/// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
/// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
/// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
/// ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
/// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
/// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
/// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
/// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
/// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
/// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
/// SUCH DAMAGE.
///
///	Source:
/// $FreeBSD: head/sys/sys/elf_common.h 294844 2016-01-26 18:20:25Z emaste $
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

/// Elf32 symbol table
#[repr(C)]
pub struct Elf32_Sym {
    st_name: Elf32_Word,
    st_value: Elf32_Addr,
    st_size: Elf32_Word,
    st_info: u8,
    st_other: u8,
    st_shndx: Elf32_Half,
}

/// Elf32 relocation structure that doesn't need an addend
#[repr(C)]
pub struct Elf32_Rel {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
}

/// Elf32 relocation structure that needs an addend
#[repr(C)]
pub struct Elf32_Rela {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
    r_addent: Elf32_Sword
}
