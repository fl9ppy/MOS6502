use crate::bus::Bus;

/// Addressing modes used by the 6502
#[derive(Clone, Copy, Debug)]
pub enum AddrMode {
    Imp,
    Acc,
    Imm,
    Zp,
    ZpX,
    ZpY,
    Abs,
    AbsX,
    AbsY,
    Ind,
    XInd,
    IndY,
    Rel,
}

/// A compact description of each opcode
#[derive(Clone, Copy)]
pub struct OpcodeInfo {
    pub mnemonic: &'static str,
    pub mode: AddrMode,
    pub size: u8,
}

//
// FULL TABLE
// 256 entries, including undocumented opcodes
//
pub const OPCODES: [OpcodeInfo; 256] = [
    // 0x00
    OpcodeInfo {
        mnemonic: "BRK",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ORA",
        mode: AddrMode::XInd,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    }, // illegal
    OpcodeInfo {
        mnemonic: "SLO",
        mode: AddrMode::XInd,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Zp,
        size: 2,
    }, // undocumented ZP NOPs
    OpcodeInfo {
        mnemonic: "ORA",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ASL",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SLO",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "PHP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ORA",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ASL",
        mode: AddrMode::Acc,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ANC",
        mode: AddrMode::Imm,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ORA",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ASL",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "SLO",
        mode: AddrMode::Abs,
        size: 3,
    },
    // 0x10
    OpcodeInfo {
        mnemonic: "BPL",
        mode: AddrMode::Rel,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ORA",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "SLO",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ORA",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ASL",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SLO",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "CLC",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ORA",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "SLO",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ORA",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ASL",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "SLO",
        mode: AddrMode::AbsX,
        size: 3,
    },
    // 0x20
    OpcodeInfo {
        mnemonic: "JSR",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "AND",
        mode: AddrMode::XInd,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "RLA",
        mode: AddrMode::XInd,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "BIT",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "AND",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ROL",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "RLA",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "PLP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "AND",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ROL",
        mode: AddrMode::Acc,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ANC",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "BIT",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "AND",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ROL",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "RLA",
        mode: AddrMode::Abs,
        size: 3,
    },
    // 0x30
    OpcodeInfo {
        mnemonic: "BMI",
        mode: AddrMode::Rel,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "AND",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "RLA",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "AND",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ROL",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "RLA",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SEC",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "AND",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "RLA",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "AND",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ROL",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "RLA",
        mode: AddrMode::AbsX,
        size: 3,
    },
    // 0x40
    OpcodeInfo {
        mnemonic: "RTI",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "EOR",
        mode: AddrMode::XInd,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "SRE",
        mode: AddrMode::XInd,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "EOR",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LSR",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SRE",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "PHA",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "EOR",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LSR",
        mode: AddrMode::Acc,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ALR",
        mode: AddrMode::Imm,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "JMP",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "EOR",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "LSR",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "SRE",
        mode: AddrMode::Abs,
        size: 3,
    },
    // 0x50
    OpcodeInfo {
        mnemonic: "BVC",
        mode: AddrMode::Rel,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "EOR",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "SRE",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "EOR",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LSR",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SRE",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "CLI",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "EOR",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "SRE",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "EOR",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "LSR",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "SRE",
        mode: AddrMode::AbsX,
        size: 3,
    },
    // 0x60
    OpcodeInfo {
        mnemonic: "RTS",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ADC",
        mode: AddrMode::XInd,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "RRA",
        mode: AddrMode::XInd,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ADC",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ROR",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "RRA",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "PLA",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ADC",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ROR",
        mode: AddrMode::Acc,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ARR",
        mode: AddrMode::Imm,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "JMP",
        mode: AddrMode::Ind,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ADC",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ROR",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "RRA",
        mode: AddrMode::Abs,
        size: 3,
    },
    // 0x70
    OpcodeInfo {
        mnemonic: "BVS",
        mode: AddrMode::Rel,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ADC",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "RRA",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ADC",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ROR",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "RRA",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SEI",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ADC",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "RRA",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ADC",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ROR",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "RRA",
        mode: AddrMode::AbsX,
        size: 3,
    },
    // 0x80
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "STA",
        mode: AddrMode::XInd,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SAX",
        mode: AddrMode::XInd,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "STY",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "STA",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "STX",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SAX",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "DEY",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "TXA",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "XAA",
        mode: AddrMode::Imm,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "STY",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "STA",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "STX",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "SAX",
        mode: AddrMode::Abs,
        size: 3,
    },
    // 0x90
    OpcodeInfo {
        mnemonic: "BCC",
        mode: AddrMode::Rel,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "STA",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "AHX",
        mode: AddrMode::IndY,
        size: 2,
    }, // undocumented
    OpcodeInfo {
        mnemonic: "STY",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "STA",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "STX",
        mode: AddrMode::ZpY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SAX",
        mode: AddrMode::ZpY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "TYA",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "STA",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "TXS",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "TAS",
        mode: AddrMode::AbsY,
        size: 3,
    }, // undocumented
    OpcodeInfo {
        mnemonic: "SHY",
        mode: AddrMode::AbsX,
        size: 3,
    }, // undocumented
    OpcodeInfo {
        mnemonic: "STA",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "SHX",
        mode: AddrMode::AbsY,
        size: 3,
    }, // undocumented
    OpcodeInfo {
        mnemonic: "AHX",
        mode: AddrMode::AbsY,
        size: 3,
    }, // undocumented
    // 0xA0
    OpcodeInfo {
        mnemonic: "LDY",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LDA",
        mode: AddrMode::XInd,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LDX",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LAX",
        mode: AddrMode::XInd,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "LDY",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LDA",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LDX",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LAX",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "TAY",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "LDA",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "TAX",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "LAX",
        mode: AddrMode::Imm,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "LDY",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "LDA",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "LDX",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "LAX",
        mode: AddrMode::Abs,
        size: 3,
    },
    // 0xB0
    OpcodeInfo {
        mnemonic: "BCS",
        mode: AddrMode::Rel,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LDA",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "LAX",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LDY",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LDA",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LDX",
        mode: AddrMode::ZpY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "LAX",
        mode: AddrMode::ZpY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "CLV",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "LDA",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "TSX",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "LAS",
        mode: AddrMode::AbsY,
        size: 3,
    }, // undocumented
    OpcodeInfo {
        mnemonic: "LDY",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "LDA",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "LDX",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "LAX",
        mode: AddrMode::AbsY,
        size: 3,
    },
    // 0xC0
    OpcodeInfo {
        mnemonic: "CPY",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "CMP",
        mode: AddrMode::XInd,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "DCP",
        mode: AddrMode::XInd,
        size: 2,
    }, // illegal
    OpcodeInfo {
        mnemonic: "CPY",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "CMP",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "DEC",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "DCP",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "INY",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "CMP",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "DEX",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "AAX",
        mode: AddrMode::Imm,
        size: 2,
    }, // undocumented
    OpcodeInfo {
        mnemonic: "CPY",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "CMP",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "DEC",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "DCP",
        mode: AddrMode::Abs,
        size: 3,
    },
    // 0xD0
    OpcodeInfo {
        mnemonic: "BNE",
        mode: AddrMode::Rel,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "CMP",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "DCP",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "CMP",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "DEC",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "DCP",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "CLD",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "CMP",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "DCP",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "CMP",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "DEC",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "DCP",
        mode: AddrMode::AbsX,
        size: 3,
    },
    // 0xE0
    OpcodeInfo {
        mnemonic: "CPX",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::XInd,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ISC",
        mode: AddrMode::XInd,
        size: 2,
    }, // illegal (also called ISC)
    OpcodeInfo {
        mnemonic: "CPX",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "INC",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ISC",
        mode: AddrMode::Zp,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "INX",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::Imm,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::Imm,
        size: 2,
    }, // duplicate immediate-style illegal variants
    OpcodeInfo {
        mnemonic: "CPX",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "INC",
        mode: AddrMode::Abs,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ISC",
        mode: AddrMode::Abs,
        size: 3,
    },
    // 0xF0
    OpcodeInfo {
        mnemonic: "BEQ",
        mode: AddrMode::Rel,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "KIL",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ISC",
        mode: AddrMode::IndY,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "INC",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "ISC",
        mode: AddrMode::ZpX,
        size: 2,
    },
    OpcodeInfo {
        mnemonic: "SED",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::Imp,
        size: 1,
    },
    OpcodeInfo {
        mnemonic: "ISC",
        mode: AddrMode::AbsY,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "NOP",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "SBC",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "INC",
        mode: AddrMode::AbsX,
        size: 3,
    },
    OpcodeInfo {
        mnemonic: "ISC",
        mode: AddrMode::AbsX,
        size: 3,
    },
    // ---------------------------
    // 0x80–0x8F
    // ---------------------------
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // 80
    OpcodeEntry {
        mnemonic: "STA",
        mode: AddressingMode::IndirectX,
        size: 2,
        illegal: false,
    }, // 81
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // 82
    OpcodeEntry {
        mnemonic: "SAX",
        mode: AddressingMode::IndirectX,
        size: 2,
        illegal: true,
    }, // 83
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: true,
    }, // 84
    OpcodeEntry {
        mnemonic: "STA",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // 85
    OpcodeEntry {
        mnemonic: "STX",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // 86
    OpcodeEntry {
        mnemonic: "SAX",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: true,
    }, // 87
    OpcodeEntry {
        mnemonic: "DEY",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // 88
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // 89
    OpcodeEntry {
        mnemonic: "TXA",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // 8A
    OpcodeEntry {
        mnemonic: "ANE",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // 8B
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: true,
    }, // 8C
    OpcodeEntry {
        mnemonic: "STA",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // 8D
    OpcodeEntry {
        mnemonic: "STX",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // 8E
    OpcodeEntry {
        mnemonic: "SAX",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: true,
    }, // 8F
    // ---------------------------
    // 0x90–0x9F
    // ---------------------------
    OpcodeEntry {
        mnemonic: "BCC",
        mode: AddressingMode::Relative,
        size: 2,
        illegal: false,
    }, // 90
    OpcodeEntry {
        mnemonic: "STA",
        mode: AddressingMode::IndirectY,
        size: 2,
        illegal: false,
    }, // 91
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // 92
    OpcodeEntry {
        mnemonic: "SHA",
        mode: AddressingMode::IndirectY,
        size: 2,
        illegal: true,
    }, // 93
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: true,
    }, // 94
    OpcodeEntry {
        mnemonic: "STA",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: false,
    }, // 95
    OpcodeEntry {
        mnemonic: "STX",
        mode: AddressingMode::ZeroPageY,
        size: 2,
        illegal: false,
    }, // 96
    OpcodeEntry {
        mnemonic: "SAX",
        mode: AddressingMode::ZeroPageY,
        size: 2,
        illegal: true,
    }, // 97
    OpcodeEntry {
        mnemonic: "TYA",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // 98
    OpcodeEntry {
        mnemonic: "STA",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: false,
    }, // 99
    OpcodeEntry {
        mnemonic: "TXS",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // 9A
    OpcodeEntry {
        mnemonic: "TAS",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: true,
    }, // 9B
    OpcodeEntry {
        mnemonic: "SHY",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: true,
    }, // 9C
    OpcodeEntry {
        mnemonic: "STA",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: false,
    }, // 9D
    OpcodeEntry {
        mnemonic: "SHX",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: true,
    }, // 9E
    OpcodeEntry {
        mnemonic: "SHA",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: true,
    }, // 9F
    // ---------------------------
    // 0xA0–0xAF
    // ---------------------------
    OpcodeEntry {
        mnemonic: "LDY",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: false,
    }, // A0
    OpcodeEntry {
        mnemonic: "LDA",
        mode: AddressingMode::IndirectX,
        size: 2,
        illegal: false,
    }, // A1
    OpcodeEntry {
        mnemonic: "LDX",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: false,
    }, // A2
    OpcodeEntry {
        mnemonic: "LAX",
        mode: AddressingMode::IndirectX,
        size: 2,
        illegal: true,
    }, // A3
    OpcodeEntry {
        mnemonic: "LDY",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // A4
    OpcodeEntry {
        mnemonic: "LDA",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // A5
    OpcodeEntry {
        mnemonic: "LDX",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // A6
    OpcodeEntry {
        mnemonic: "LAX",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: true,
    }, // A7
    OpcodeEntry {
        mnemonic: "TAY",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // A8
    OpcodeEntry {
        mnemonic: "LDA",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: false,
    }, // A9
    OpcodeEntry {
        mnemonic: "TAX",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // AA
    OpcodeEntry {
        mnemonic: "LAX",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // AB (aka ANE variant)
    OpcodeEntry {
        mnemonic: "LDY",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // AC
    OpcodeEntry {
        mnemonic: "LDA",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // AD
    OpcodeEntry {
        mnemonic: "LDX",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // AE
    OpcodeEntry {
        mnemonic: "LAX",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: true,
    }, // AF
    // ---------------------------
    // 0xB0–0xBF
    // ---------------------------
    OpcodeEntry {
        mnemonic: "BCS",
        mode: AddressingMode::Relative,
        size: 2,
        illegal: false,
    }, // B0
    OpcodeEntry {
        mnemonic: "LDA",
        mode: AddressingMode::IndirectY,
        size: 2,
        illegal: false,
    }, // B1
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // B2
    OpcodeEntry {
        mnemonic: "LAX",
        mode: AddressingMode::IndirectY,
        size: 2,
        illegal: true,
    }, // B3
    OpcodeEntry {
        mnemonic: "LDY",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: false,
    }, // B4
    OpcodeEntry {
        mnemonic: "LDA",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: false,
    }, // B5
    OpcodeEntry {
        mnemonic: "LDX",
        mode: AddressingMode::ZeroPageY,
        size: 2,
        illegal: false,
    }, // B6
    OpcodeEntry {
        mnemonic: "LAX",
        mode: AddressingMode::ZeroPageY,
        size: 2,
        illegal: true,
    }, // B7
    OpcodeEntry {
        mnemonic: "CLV",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // B8
    OpcodeEntry {
        mnemonic: "LDA",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: false,
    }, // B9
    OpcodeEntry {
        mnemonic: "TSX",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // BA
    OpcodeEntry {
        mnemonic: "LAS",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: true,
    }, // BB
    OpcodeEntry {
        mnemonic: "LDY",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: false,
    }, // BC
    OpcodeEntry {
        mnemonic: "LDA",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: false,
    }, // BD
    OpcodeEntry {
        mnemonic: "LDX",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: false,
    }, // BE
    OpcodeEntry {
        mnemonic: "LAX",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: true,
    }, // BF
    // ---------------------------
    // 0xC0–0xCF
    // ---------------------------
    OpcodeEntry {
        mnemonic: "CPY",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: false,
    }, // C0
    OpcodeEntry {
        mnemonic: "CMP",
        mode: AddressingMode::IndirectX,
        size: 2,
        illegal: false,
    }, // C1
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // C2
    OpcodeEntry {
        mnemonic: "DCP",
        mode: AddressingMode::IndirectX,
        size: 2,
        illegal: true,
    }, // C3
    OpcodeEntry {
        mnemonic: "CPY",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // C4
    OpcodeEntry {
        mnemonic: "CMP",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // C5
    OpcodeEntry {
        mnemonic: "DEC",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // C6
    OpcodeEntry {
        mnemonic: "DCP",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: true,
    }, // C7
    OpcodeEntry {
        mnemonic: "INY",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // C8
    OpcodeEntry {
        mnemonic: "CMP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: false,
    }, // C9
    OpcodeEntry {
        mnemonic: "DEX",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // CA
    OpcodeEntry {
        mnemonic: "AXS",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // CB
    OpcodeEntry {
        mnemonic: "CPY",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // CC
    OpcodeEntry {
        mnemonic: "CMP",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // CD
    OpcodeEntry {
        mnemonic: "DEC",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // CE
    OpcodeEntry {
        mnemonic: "DCP",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: true,
    }, // CF
    // ---------------------------
    // 0xD0–0xDF
    // ---------------------------
    OpcodeEntry {
        mnemonic: "BNE",
        mode: AddressingMode::Relative,
        size: 2,
        illegal: false,
    }, // D0
    OpcodeEntry {
        mnemonic: "CMP",
        mode: AddressingMode::IndirectY,
        size: 2,
        illegal: false,
    }, // D1
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // D2
    OpcodeEntry {
        mnemonic: "DCP",
        mode: AddressingMode::IndirectY,
        size: 2,
        illegal: true,
    }, // D3
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: true,
    }, // D4
    OpcodeEntry {
        mnemonic: "CMP",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: false,
    }, // D5
    OpcodeEntry {
        mnemonic: "DEC",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: false,
    }, // D6
    OpcodeEntry {
        mnemonic: "DCP",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: true,
    }, // D7
    OpcodeEntry {
        mnemonic: "CLD",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // D8
    OpcodeEntry {
        mnemonic: "CMP",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: false,
    }, // D9
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: true,
    }, // DA
    OpcodeEntry {
        mnemonic: "DCP",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: true,
    }, // DB
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: true,
    }, // DC
    OpcodeEntry {
        mnemonic: "CMP",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: false,
    }, // DD
    OpcodeEntry {
        mnemonic: "DEC",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: false,
    }, // DE
    OpcodeEntry {
        mnemonic: "DCP",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: true,
    }, // DF
    // ---------------------------
    // 0xE0–0xEF
    // ---------------------------
    OpcodeEntry {
        mnemonic: "CPX",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: false,
    }, // E0
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::IndirectX,
        size: 2,
        illegal: false,
    }, // E1
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // E2
    OpcodeEntry {
        mnemonic: "ISC",
        mode: AddressingMode::IndirectX,
        size: 2,
        illegal: true,
    }, // E3
    OpcodeEntry {
        mnemonic: "CPX",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // E4
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // E5
    OpcodeEntry {
        mnemonic: "INC",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: false,
    }, // E6
    OpcodeEntry {
        mnemonic: "ISC",
        mode: AddressingMode::ZeroPage,
        size: 2,
        illegal: true,
    }, // E7
    OpcodeEntry {
        mnemonic: "INX",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // E8
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: false,
    }, // E9
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: true,
    }, // EA (NOP official)
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // EB (SBC unofficial)
    OpcodeEntry {
        mnemonic: "CPX",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // EC
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // ED
    OpcodeEntry {
        mnemonic: "INC",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: false,
    }, // EE
    OpcodeEntry {
        mnemonic: "ISC",
        mode: AddressingMode::Absolute,
        size: 3,
        illegal: true,
    }, // EF
    // ---------------------------
    // 0xF0–0xFF
    // ---------------------------
    OpcodeEntry {
        mnemonic: "BEQ",
        mode: AddressingMode::Relative,
        size: 2,
        illegal: false,
    }, // F0
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::IndirectY,
        size: 2,
        illegal: false,
    }, // F1
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Immediate,
        size: 2,
        illegal: true,
    }, // F2
    OpcodeEntry {
        mnemonic: "ISC",
        mode: AddressingMode::IndirectY,
        size: 2,
        illegal: true,
    }, // F3
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: true,
    }, // F4
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: false,
    }, // F5
    OpcodeEntry {
        mnemonic: "INC",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: false,
    }, // F6
    OpcodeEntry {
        mnemonic: "ISC",
        mode: AddressingMode::ZeroPageX,
        size: 2,
        illegal: true,
    }, // F7
    OpcodeEntry {
        mnemonic: "SED",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: false,
    }, // F8
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: false,
    }, // F9
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::Implied,
        size: 1,
        illegal: true,
    }, // FA
    OpcodeEntry {
        mnemonic: "ISC",
        mode: AddressingMode::AbsoluteY,
        size: 3,
        illegal: true,
    }, // FB
    OpcodeEntry {
        mnemonic: "NOP",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: true,
    }, // FC
    OpcodeEntry {
        mnemonic: "SBC",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: false,
    }, // FD
    OpcodeEntry {
        mnemonic: "INC",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: false,
    }, // FE
    OpcodeEntry {
        mnemonic: "ISC",
        mode: AddressingMode::AbsoluteX,
        size: 3,
        illegal: true,
    }, // FF
];

fn format_operand(mode: AddrMode, bytes: &[u8]) -> String {
    match mode {
        AddrMode::Imp => "".into(),
        AddrMode::Acc => "A".into(),
        AddrMode::Imm => format!("#${:02X}", bytes[0]),
        AddrMode::Zp => format!("${:02X}", bytes[0]),
        AddrMode::ZpX => format!("${:02X},X", bytes[0]),
        AddrMode::ZpY => format!("${:02X},Y", bytes[0]),
        AddrMode::Abs => format!("${:02X}{:02X}", bytes[1], bytes[0]),
        AddrMode::AbsX => format!("${:02X}{:02X},X", bytes[1], bytes[0]),
        AddrMode::AbsY => format!("${:02X}{:02X},Y", bytes[1], bytes[0]),
        AddrMode::Ind => format!("(${:02X}{:02X})", bytes[1], bytes[0]),
        AddrMode::XInd => format!("(${:02X},X)", bytes[0]),
        AddrMode::IndY => format!("(${:02X}),Y", bytes[0]),
        AddrMode::Rel => {
            let offset = bytes[0] as i8;
            let target = ((offset as i32) & 0xFFFF) as i32;
            format!("${:04X}", target)
        }
    }
}

/// A fully decoded instruction
pub struct Disassembled {
    pub addr: u16,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operand: String,
    pub size: u8,
    pub next_pc: u16,
}

impl Disassembled {
    pub fn pretty(&self) -> String {
        let mut b = String::new();
        for byte in &self.bytes {
            b.push_str(&format!("{:02X} ", byte));
        }

        format!(
            "{:04X}  {:<9}  {:<3} {}",
            self.addr, b, self.mnemonic, self.operand
        )
    }
}

/// Formats operands according to addressing mode
fn fmt_operand(bus: &impl Bus, pc: u16, info: &OpcodeInfo) -> String {
    match info.mode {
        AddrMode::Imp => "".into(),
        AddrMode::Acc => "A".into(),
        AddrMode::Imm => format!("#${:02X}", bus.read(pc + 1)),
        AddrMode::Zp => format!("${:02X}", bus.read(pc + 1)),
        AddrMode::ZpX => format!("${:02X},X", bus.read(pc + 1)),
        AddrMode::ZpY => format!("${:02X},Y", bus.read(pc + 1)),
        AddrMode::Abs => {
            let lo = bus.read(pc + 1) as u16;
            let hi = bus.read(pc + 2) as u16;
            format!("${:04X}", (hi << 8) | lo)
        }
        AddrMode::AbsX => {
            let lo = bus.read(pc + 1) as u16;
            let hi = bus.read(pc + 2) as u16;
            format!("${:04X},X", (hi << 8) | lo)
        }
        AddrMode::AbsY => {
            let lo = bus.read(pc + 1) as u16;
            let hi = bus.read(pc + 2) as u16;
            format!("${:04X},Y", (hi << 8) | lo)
        }
        AddrMode::Ind => {
            let lo = bus.read(pc + 1) as u16;
            let hi = bus.read(pc + 2) as u16;
            format!("(${:04X})", (hi << 8) | lo)
        }
        AddrMode::XInd => {
            let zp = bus.read(pc + 1);
            format!("(${:02X},X)", zp)
        }
        AddrMode::IndY => {
            let zp = bus.read(pc + 1);
            format!("(${:02X}),Y", zp)
        }
        AddrMode::Rel => {
            let offset = bus.read(pc + 1) as i8;
            let target = pc.wrapping_add(2).wrapping_add(offset as i16 as u16);
            format!("${:04X}", target)
        }
    }
}

/// Main disassembler entry point
pub fn disassemble(bus: &impl Bus, pc: u16) -> Disassembled {
    let opcode = bus.read(pc);
    let info = OPCODES[opcode as usize];

    let mut bytes = Vec::new();
    for i in 0..info.size {
        bytes.push(bus.read(pc + i as u16));
    }

    let operand = fmt_operand(bus, pc, &info);

    Disassembled {
        addr: pc,
        bytes,
        mnemonic: info.mnemonic.into(),
        operand,
        size: info.size,
        next_pc: pc.wrapping_add(info.size as u16),
    }
}
