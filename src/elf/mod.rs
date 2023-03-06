/*
⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀
⠸⣿⣿⣶⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣶⣿⣿⡏
⠀⣿⣿⣿⣿⣿⣿⣿⣶⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡤⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣦⠀⠀⠀⠀⠀⢠⣄⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣤⣶⣿⣿⣿⣿⣿⣿⣿⠁
⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣦⣄⣀⠀⢀⣴⡟⠀⠀⠀⠀⢠⣟⠛⠛⠛⠛⠛⠛⣣⡀⠀⠀⠀⠀⠹⣷⡀⠀⢀⣠⣴⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀
⠀⠀⠉⠛⠛⠛⠻⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⣰⣿⣿⣦⠀⠀⠀⠀⣰⣿⣿⣄⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⠿⠛⠛⠛⠉⠁⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠛⠛⠻⠿⣿⣿⡀⠀⠀⣴⣿⣿⣿⣿⣷⡀⠀⣼⣿⣿⣿⣿⣦⠀⠀⠀⣿⣿⡿⠿⠛⠛⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⣤⣤⣤⣤⣶⣿⣿⡇⠀⠼⠿⠿⠿⠿⠿⠿⠷⠾⠿⠿⠿⠿⠿⠿⠷⠀⢰⣿⣿⣷⣦⣤⣤⣤⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀
⠀⠀⠀⠀⠈⢿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠛⠉⢁⣹⣿⣿⣿⣶⣄⡀⠀⠀⠀⠀⢠⣆⠀⠀⠀⠀⢀⣠⣴⣿⣿⣿⣿⡀⠉⠙⠻⠿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠘⣿⣿⠿⠟⠛⠉⠁⠀⠀⣀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⢀⣾⣿⣆⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣦⣄⠀⠀⠀⠉⠛⠻⠿⣿⣿⠇⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⣿⡿⠋⣸⣿⣿⣿⣿⡿⠃⢀⣾⣿⣿⣿⡄⠀⠿⣿⣿⣿⣿⣯⠈⢿⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣴⣿⣿⣿⣿⣿⠋⠀⢠⣿⣿⣿⡏⠀⠀⢀⣼⣿⣿⣿⣿⣷⡀⠀⠀⠘⣿⣿⣿⡆⠀⠙⢿⣿⣿⣿⣿⣦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⠟⠁⠀⠀⣾⣿⣿⣿⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⡿⠂⠀⠀⢿⣿⣿⣿⡀⠀⠀⠙⣿⣿⣿⠿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠁⠀⠀⠀⣸⣿⣿⣿⡇⠀⢠⣤⡀⠉⢻⣿⣿⣿⠋⠀⣠⣄⠀⠘⣿⣿⣿⣧⠀⠀⠀⠈⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠙⠛⠀⢠⣿⣿⠃⠀⢸⣿⣿⣿⠀⠈⢿⣿⣇⠀⠛⠋⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⠃⠀⠀⢸⣿⣿⣿⠂⠀⠈⢻⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⣿⣿⡿⠿⠿⠿⣿⣿⡆⠀⠀⠈⢿⣿⠃⠀⠀⠰⣿⣿⠿⠿⠿⢿⣿⣿⠗⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⢧⡀⠀⠀⠘⠉⠀⠀⠀⠀⠈⠃⠀⠀⠀⠀⠈⠛⠀⠀⠀⣠⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
*/
use std::{io::{Read, Seek}, fs::File};

// ELF Header structure - Technical details for identification and execution.
#[derive(Debug)]
pub struct ELFHeader {
    // e_ident - ELF signature, this is constant and made of 4 fields.
    pub e_ident: EIdentFields,

    // e_type - ELF type (32-bit or 64-bit)
    pub e_type: u16,

    // e_machine - Target machine (x86, x86_64, ARM, etc.)
    pub e_machine: u16,

    // e_version - ELF version (should always be 1)
    pub e_version: u32,

    // e_entry - Entry point address (where the program starts)
    pub e_entry: u64,

    // e_phoff - Program header table offset
    pub e_phoff: u64,

    // e_shoff - Section header table offset
    pub e_shoff: u64,

    // e_ehsize - ELF header size
    pub e_ehsize: u16,

    // e_phentsize - Size of a single program header
    pub e_phentsize: u16,

    // e_phnum - Number of program headers
    pub e_phnum: u16,

    // e_shentsize - Size of a single section header
    pub e_shentsize: u16,

    // e_shnum - Number of section headers
    pub e_shnum: u16,

    // e_shstrndx - Index of the names section
    pub e_shstrndx: u16,
}

// Subfields for the e_ident field of the ELF header.
#[derive(Debug)]
pub struct EIdentFields {
    // EI_MAG - Magic number (0x7f, 'E', 'L', 'F') - 4 bytes. This is constant.
    pub ei_magic: [u8; 4], // 0x7f, 'E', 'L', 'F'

    //EI_CLASS - 32-bit or 64-bit - 1 byte.
    pub ei_class: u8, // 1 = 32-bit, 2 = 64-bit

    // EI_DATA - Little or big endian - 1 byte.
    pub ei_data: u8, // 1 = little endian, 2 = big endian

    // EI_VERSION - ELF version - 1 byte. Should always be 1.
    pub ei_version: u8, // Always 1
}

// PHT (Program Header Table) structure - Execution information.
#[derive(Debug)]
pub struct ProgramHeader {
    // p_type - Type of segment (loadable, dynamic, etc.)
    pub p_type: u32,

    // p_offset - Offset of where it should be read
    pub p_offset: u64,

    // p_vaddr - Virtual address of where it should be loaded
    pub p_vaddr: u64,

    // p_paddr - Physical address of where it should be loaded
    pub p_paddr: u64,

    // p_filesz - Size on file
    pub p_filesz: u64,

    // p_memsz - Size in memory
    pub p_memsz: u64,

    // p_flags - Flags (read, write, execute)
    pub p_flags: u32,
}

// parse_elf - Parses the ELF header of a file and outputs the ELFHeader struct.
pub fn parse_elf(file: &mut File) -> Result<ELFHeader, String> {
    let mut buffer = [0; 64];
    file.read(&mut buffer).unwrap();

    let mut e_ident = EIdentFields {
        ei_magic: [0; 4],
        ei_class: 0,
        ei_data: 0,
        ei_version: 0,
    };

    e_ident.ei_magic[0] = buffer[0];
    e_ident.ei_magic[1] = buffer[1];
    e_ident.ei_magic[2] = buffer[2];
    e_ident.ei_magic[3] = buffer[3];
    e_ident.ei_class = buffer[4];
    e_ident.ei_data = buffer[5];
    e_ident.ei_version = buffer[6];

    let e_type = u16::from_le_bytes([buffer[16], buffer[17]]);
    let e_machine = u16::from_le_bytes([buffer[18], buffer[19]]);
    let e_version = u32::from_le_bytes([buffer[20], buffer[21], buffer[22], buffer[23]]);
    let e_entry = u64::from_le_bytes([buffer[24], buffer[25], buffer[26], buffer[27], buffer[28], buffer[29], buffer[30], buffer[31]]);
    let e_phoff = u64::from_le_bytes([buffer[32], buffer[33], buffer[34], buffer[35], buffer[36], buffer[37], buffer[38], buffer[39]]);
    let e_shoff = u64::from_le_bytes([buffer[40], buffer[41], buffer[42], buffer[43], buffer[44], buffer[45], buffer[46], buffer[47]]);
    let e_ehsize = u16::from_le_bytes([buffer[48], buffer[49]]);
    let e_phentsize = u16::from_le_bytes([buffer[50], buffer[51]]);
    let e_phnum = u16::from_le_bytes([buffer[52], buffer[53]]);
    let e_shentsize = u16::from_le_bytes([buffer[54], buffer[55]]);
    let e_shnum = u16::from_le_bytes([buffer[56], buffer[57]]);
    let e_shstrndx = u16::from_le_bytes([buffer[58], buffer[59]]
    );

    let elf_header = ELFHeader {
        e_ident,
        e_type,
        e_machine,
        e_version,
        e_entry,
        e_phoff,
        e_shoff,
        e_ehsize,
        e_phentsize,
        e_phnum,
        e_shentsize,
        e_shnum,
        e_shstrndx,
    };

    Ok(elf_header)

}

// parse_pht - Parses the PHT of a file and outputs a vector of ProgramHeader structs.
pub fn parse_pht(file: &mut File, elf_header: &ELFHeader) -> Result<Vec<ProgramHeader>, String> {
    let mut pht = Vec::new();

    file.seek(std::io::SeekFrom::Start(elf_header.e_phoff as u64)).unwrap();

    for _ in 0..elf_header.e_phnum {
        let mut buffer = [0; 56];
        file.read(&mut buffer).unwrap();

        let p_type = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let p_offset = u64::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7], buffer[8], buffer[9], buffer[10], buffer[11]]);
        let p_vaddr = u64::from_le_bytes([buffer[12], buffer[13], buffer[14], buffer[15], buffer[16], buffer[17], buffer[18], buffer[19]]);
        let p_paddr = u64::from_le_bytes([buffer[20], buffer[21], buffer[22], buffer[23], buffer[24], buffer[25], buffer[26], buffer[27]]);
        let p_filesz = u64::from_le_bytes([buffer[28], buffer[29], buffer[30], buffer[31], buffer[32], buffer[33], buffer[34], buffer[35]]);
        let p_memsz = u64::from_le_bytes([buffer[36], buffer[37], buffer[38], buffer[39], buffer[40], buffer[41], buffer[42], buffer[43]]);
        let p_flags = u32::from_le_bytes([buffer[44], buffer[45], buffer[46], buffer[47]]);

        let program_header = ProgramHeader {
            p_type,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_flags,
        };

        pht.push(program_header);
    }

    Ok(pht)
}