use crate::bus::Bus;

#[derive(Clone, Copy)]
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

pub fn disassemble(bus: &impl Bus, pc: u16) -> String {
    let opcode = bus.read(pc);
    let info = &OPCODES[opcode as usize];

    let mut bytes = vec![opcode];
    for i in 1..info.size {
        bytes.push(bus.read(pc.wrapping_add(i as u16)));
    }

    let operand = if info.size > 1 {
        format_operand(info.mode, &bytes[1..])
    } else {
        "".into()
    };

    let hex_dump = match info.size {
        1 => format!("{:02X}      ", bytes[0]),
        2 => format!("{:02X} {:02X}   ", bytes[0], bytes[1]),
        3 => format!("{:02X} {:02X} {:02X}", bytes[0], bytes[1], bytes[2]),
        _ => unreachable!(),
    };

    format!("{:04X}  {}  {} {}", pc, hex_dump, info.mnemonic, operand)
}
