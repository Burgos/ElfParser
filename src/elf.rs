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

/// Magic number; byte 0
const EI_MAG0: usize = 0;
/// Magic number; byte 1
const EI_MAG1: usize = 1;
/// Magic number; byte 2
const EI_MAG2: usize = 2;
/// Magic number; byte 3
const EI_MAG3: usize = 3;
/// Class of machine
const EI_CLASS: usize = 4;
/// Data format
const EI_DATA: usize = 5;
/// ELF format version
const EI_VERSION: usize = 6;
/// Operating system / ABI ident
const EI_OSABI: usize = 7;
/// ABI version
const EI_ABIVERSION: usize = 8;
/// Start of architecture definition
const OLD_EI_BRAND: usize = 8;
/// Start of padding (per SV4 ABI)
const EI_PAD: usize = 9;
/// Size of e_ident array
const EI_NIDENT: usize = 16;


/// Values for the magic number bytes
const ELFMAG0: u8 = 0x7f;
const ELFMAG1: u8 = 'E' as u8;
const ELFMAG2: u8 = 'L' as u8;
const ELFMAG3: u8 = 'F' as u8;
const ELFMAG: &'static str = "\u{177}ELF";

/// Magic string size
const SELFMAG: usize = 4;

/// Values for  EI_DATA
enum EiData {
    /// Unknwon data format
    ELFDATANONE = 0,
    /// Little endian, 2's complement
    ELFDATA2LSB = 1,
    /// Big endian, 2's complement
    ELFDATA2MSB,
}

/// Values for EI_CLASS
enum EiClass {
    /// Unknown class
    ELFCLASSNONE = 0,
    /// 32 bit arhitecture
    ELFCLASS32 = 1,
    /// 64 bit architecture
    ELFCLASS64 = 2
}


/// Values for EI_OSABI
enum EiOsAbi {
	/// UNIX System V ABI 
	ELFOSABI_NONE          =  0,
	/// HP-UX operating system
	ELFOSABI_HPUX          =  1,
	/// NetBSD
	ELFOSABI_NETBSD        =  2,
	/// GNU/Linux
	ELFOSABI_LINUX         =  3,
	/// GNU/Hurd
	ELFOSABI_HURD          =  4,
	///  86Open common IA32 ABI
	ELFOSABI_86OPEN        =  5,
	/// Solaris
	ELFOSABI_SOLARIS       =  6,
	/// AIX
	ELFOSABI_AIX           =  7,
	/// IRIX
	ELFOSABI_IRIX          =  8,
	/// FreeBSD
	ELFOSABI_FREEBSD       =  9,
	/// TRU64 UNIX
	ELFOSABI_TRU64         =  10,
	/// Novell Modesto
	ELFOSABI_MODESTO       =  11,
	/// OpenBSD
	ELFOSABI_OPENBSD       =  12,
	/// Open VMS
	ELFOSABI_OPENVMS       =  13,
	/// HP Non-Stop Kernel
	ELFOSABI_NSK           =  14,
	/// Amiga Research OS
	ELFOSABI_AROS          =  15,
	/// FenixOS
	ELFOSABI_FENIXOS       =  16,
	/// Nuxi CloudABI
	ELFOSABI_CLOUDABI      =  17,
	/// ARM
	ELFOSABI_ARM           =  97,
	/// Standalone (embedded) application
	ELFOSABI_STANDALONE    =  255,
}

enum EiMachine {
    /// Unknown machine. 
    EM_NONE     = 0,
    /// AT&T W    E32100. 
    EM_M32      = 1,
    /// Sun SPARC. 
    EM_SPARC    = 2,
    /// Intel i386. 
    EM_386      = 3,
    /// Motorola= 68000. 
    EM_68K      = 4,
    /// Motorola= 88000. 
    EM_88K      = 5,
    /// Intel MCU. 
    EM_IAMCU    = 6,
    /// Intel i860. 
    EM_860      = 7,
    /// MIPS R3000 Big-    Endian only. 
    EM_MIPS     = 8,
    /// IBM System/370. 
    EM_S370     = 9,
    /// MIPS R3000 Little-    Endian.
    EM_MIPS_RS3_LE = 10,
    /// HP PA-RISC.
    EM_PARISC   = 15,
    /// Fujitsu VPP500.
    EM_VPP500   = 17,
    /// SPARC v8plus.
    EM_SPARC32PLUS = 18,
    /// Intel= 80960.
    EM_960      = 19,
    /// PowerPC= 32-bit.
    EM_PPC      = 20,
    /// PowerPC= 64-bit.
    EM_PPC64    = 21,
    /// IBM System/390.
    EM_S390     = 22,
    /// N    EC V800.
    EM_V800     = 36,
    /// Fujitsu FR20.
    EM_FR20     = 37,
    /// TRW RH-32.
    EM_RH32     = 38,
    /// Motorola RCE.
    EM_RCE      = 39,
    /// ARM.
    EM_ARM      = 40,
    /// Hitachi SH.
    EM_SH       = 42,
    /// SPARC v9= 64-bit.
    EM_SPARCV9   = 43,
    /// Siemens TriCore embedded processor.
    EM_TRICORE   = 44,
    /// Argonaut RISC Core.
    EM_ARC      = 45,
    /// Hitachi H8/300.
    EM_H8_300   = 46,
    /// Hitachi H8/300H.
    EM_H8_300H   = 47,
    /// Hitachi H8S.
    EM_H8S      = 48,
    /// Hitachi H8/500.
    EM_H8_500   = 49,
    /// Intel IA-64 Processor.
    EM_IA_64    = 50,
    /// Stanford MIPS-X.
    EM_MIPS_X   = 51,
    /// Motorola ColdFire.
    EM_COLDFIRE   = 52,
    /// Motorola M68HC12.
    EM_68HC12   = 53,
    /// Fujitsu MMA.
    EM_MMA      = 54,
    /// Siemens PCP.
    EM_PCP      = 55,
    /// Sony nCPU.
    EM_NCPU     = 56,
    /// Denso NDR1 microprocessor.
    EM_NDR1     = 57,
    /// Motorola Star*Core processor.
    EM_STARCORE   = 58,
    /// Toyota M    E16 processor.
    EM_ME16     = 59,
    /// STMicroelectronics ST100 processor.
    EM_ST100    = 60,
    /// Advanced Logic Corp. TinyJ processor.
    EM_TINYJ    = 61,
    /// Advanced Micro Devices x86-64,
    EM_X86_64   = 62,
    /// Sony DSP Processor.
    EM_PDSP     = 63,
    /// Siemens FX66 microcontroller.
    EM_FX66     = 66,
    EM_ST9PLUS   = 67,
    EM_ST7      = 68,
    /// Motorola MC68HC16 microcontroller.
    EM_68HC16   = 69,
    /// Motorola MC68HC11 microcontroller.
    EM_68HC11   = 70,
    /// Motorola MC68HC08 microcontroller.
    EM_68HC08   = 71,
    /// Motorola MC68HC05 microcontroller.
    EM_68HC05   = 72,
    /// Silicon Graphics SVx.
    EM_SVX      = 73,
    /// STMicroelectronics ST19= 8-bit mc.
    EM_ST19     = 74,
    /// Digital VAX.
    EM_VAX      = 75,
    EM_CRIS     = 76,
    EM_JAVELIN   = 77,
    ///    Element= 14 64-bit DSP Processor.
    EM_FIREPATH   = 78,
    /// LSI Logic= 16-bit DSP Processor.
    EM_ZSP      = 79,
    /// Donald Knuth's educational= 64-bit proc.
    EM_MMIX     = 80,
    EM_HUANY    = 81,
    /// SiTera Prism.
    EM_PRISM    = 82,
    /// Atmel AVR= 8-bit microcontroller.
    EM_AVR      = 83,
    /// Fujitsu FR30.
    EM_FR30     = 84,
    /// Mitsubishi D10V.
    EM_D10V     = 85,
    /// Mitsubishi D30V.
    EM_D30V     = 86,
    /// N    EC v850.
    EM_V850     = 87,
    /// Mitsubishi M32R.
    EM_M32R     = 88,
    /// Matsushita MN10300.
    EM_MN10300   = 89,
    /// Matsushita MN10200.
    EM_MN10200   = 90,
    /// picoJava.
    EM_PJ       = 91,
    /// OpenRISC= 32-bit embedded processor.
    EM_OPENRISC   = 92,
    /// ARC Cores Tangent-A5.
    EM_ARC_A5   = 93,
    /// Tensilica Xtensa Architecture.
    EM_XTENSA   = 94,
    /// Alphamosaic VideoCore processor.
    EM_VIDEOCORE   = 95,
	/// Thompson Multimedia General Purpose
    /// Processor.
    EM_TMM_GPP   = 96,
    /// National Semiconductor= 32000 series.
    EM_NS32K    = 97,
    /// Tenor Network TPC processor.
    EM_TPC      = 98,
    /// Trebia SNP= 1000 processor.
    EM_SNP1K    = 99,
    /// STMicroelectronics ST200 microcontroller.
    EM_ST200    = 100,
    /// Ubicom IP2xxx microcontroller family.
    EM_IP2K     = 101,
    /// MAX Processor.
    EM_MAX      = 102,
	/// National Semiconductor CompactRISC
    /// microprocessor.
    EM_CR       = 103,
    /// Fujitsu F2MC16.
    EM_F2MC16   = 104,
	/// Texas Instruments embedded microcontroller
    /// msp430. 
    EM_MSP430   = 105,
    /// Analog Devices Blackfin (DSP) processor.
    EM_BLACKFIN   = 106,
    /// S1C33 Family of Seiko     Epson processors.
    EM_SE_C33   = 107,
    /// Sharp embedded microprocessor.
    EM_SEP      = 108,
    /// Arca RISC Microprocessor.
    EM_ARCA     = 109,
	/// Microprocessor series from PKU-Unity Ltd.
    /// and MPRC of Peking University 
    EM_UNICORE   = 110,
    /// AArch64 (64-bit ARM)
    EM_AARCH64   = 183,
    /// RISC-V
    EM_RISCV    = 243,
}

enum EiVersion {
	// Invalid ELF version
	EV_NONE = 0,
	// Current version
	EV_CURRENT = 1,
}

enum EPhnum {
	// Extenden Numbering
	PN_XNUM = 0xFFFF,
}

enum PType {
	// Program header table entry
	PT_NULL = 0,
	// Loadable program segment
	PT_LOAD = 1,
	// Dynamic linking information
	PT_DYNAMIC = 2,
	// Program interpreter
	PT_INTERP = 3,
	// Auxiliary information
	PT_NOTE = 4,
	// Reserved, unspecified semantics
	PT_SHLIB = 5,
	// Entry for header table itself
	PT_PHDR = 6,
	// Thread local storage segment
	PT_TLS = 7,
	// OS-Specific
	PT_LOOS = 0x6000000,
	// OS-Specific
	PT_HIOS = 0x6FFFFFF,
	// Processor-specific
	PT_LOPROC = 0x7000000,
	// Processor-specific
	PT_HIPROC = 0x7FFFFFF, 
	
}


