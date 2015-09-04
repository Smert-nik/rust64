// opcode enumeration suffix: // addressing mode:
// imm = #$00                 // immediate 
// zp = $00                   // zero page
// zpx = $00,X                // zero page with X
// zpy = $00,Y                // zero page with Y
// izx = ($00,X)              // indexed indirect (X)
// izy = ($00),Y              // indirect indexed (Y)
// abs = $0000                // absolute
// abx = $0000,X              // indexed absolute with X
// aby = $0000,Y              // indexed absolute with Y
// ind = ($0000)              // absolute indirect
// rel = $0000                // relative to PC/IP

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use cpu;

pub enum AddrMode
{
    Implied,
    Accumulator,
    Immediate,
    Absolute,
    IndexedAbsoluteX,
    IndexedAbsoluteY,
    Zeropage,
    ZeropageIndexedX,
    ZeropageIndexedY,
    Relative,
    AbsoluteIndirect,
    IndexedIndirectX,
    IndirectIndexedY
}

pub enum Op {
    // Load/store
    LDA, LDX, LDY,
    STA, STX, STY,
    // Register transfers
    TAX, TAY, TXA,
    TYA,
    // Stack operations
    TSX, TXS, PHA,
    PHP, PLA, PLP,
    // Logical
    AND, EOR, ORA,
    BIT,
    // Arithmetic
    ADC, SBC, CMP,
    CPX, CPY,
    // Inc/dec
    INC, INX, INY,
    DEC, DEX, DEY,
    // Shifts
    ASL, LSR, ROL,
    ROR,
    // Jump calls
    JMP, JSR, RTS,
    // Branches
    BCC, BCS, BEQ,
    BMI, BNE, BPL,
    BVC, BVS,
    // Status flag changes
    CLC, CLD, CLI,
    CLV, SEC, SED,
    SEI,
    // System functions
    BRK, NOP, RTI,
    // forbidden/undocumented
    HLT, SLO, ANC,
    RLA, SRE, RRA,
    ALR, SAX, XAA,
    AHX, TAS, SHY,
    SHX, ARR, LAX,
    LAS, DCP, AXS,
    ISC
}

impl Op
{
    pub fn run(&self, addr_mode: &AddrMode, cpu: &mut cpu::CPU)
    {
       /*  match *self
        {
            // Instruction::LDA => (),
            _ => println!("Opcode {}", self)
        } */        
    }
    
}


pub fn get_instruction(opcode: u8, cpu: &mut cpu::CPU) -> Option<(Op, u8, AddrMode)>    
{    
    Some(match opcode
         {
             /* ** documented instructions ** */
             /* BRK     */ 0x00 => (Op::BRK, 7, AddrMode::Implied),
             /* ORA_izx */ 0x01 => (Op::ORA, 6, AddrMode::IndexedIndirectX),
             /* ORA_zp  */ 0x05 => (Op::ORA, 3, AddrMode::Zeropage),
             /* ASL_zp  */ 0x06 => (Op::ASL, 5, AddrMode::Zeropage), 
             /* PHP     */ 0x08 => (Op::PHP, 3, AddrMode::Implied),
             /* ORA_imm */ 0x09 => (Op::ORA, 2, AddrMode::Immediate),
             /* ASL     */ 0x0A => (Op::ASL, 2, AddrMode::Implied),
             /* ORA_abs */ 0x0D => (Op::ORA, 4, AddrMode::Absolute),
             /* ASL_abs */ 0x0E => (Op::ASL, 6, AddrMode::Absolute),
             /* BPL_rel */ 0x10 => (Op::BPL, 2, AddrMode::Relative), // add 1 cycle if page boundary is crossed
             /* ORA_izy */ 0x11 => (Op::ORA, 5, AddrMode::IndirectIndexedY), // add 1 cycle if page boundary is crossed
             /* ORA_zpx */ 0x15 => (Op::ORA, 4, AddrMode::ZeropageIndexedX),
             /* ASL_zpx */ 0x16 => (Op::ASL, 6, AddrMode::ZeropageIndexedX),
             /* CLC     */ 0x18 => (Op::CLC, 2, AddrMode::Implied),
             /* ORA_aby */ 0x19 => (Op::ORA, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* ORA_abx */ 0x1D => (Op::ORA, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* ASL_abx */ 0x1E => (Op::ASL, 7, AddrMode::IndexedAbsoluteX),
             /* JSR_abs */ 0x20 => (Op::JSR, 6, AddrMode::Absolute),
             /* AND_izx */ 0x21 => (Op::AND, 6, AddrMode::IndexedIndirectX),
             /* BIT_zp  */ 0x24 => (Op::BIT, 3, AddrMode::Zeropage),
             /* AND_zp  */ 0x25 => (Op::AND, 3, AddrMode::Zeropage),
             /* ROL_zp  */ 0x26 => (Op::ROL, 5, AddrMode::Zeropage),
             /* PLP     */ 0x28 => (Op::PLP, 4, AddrMode::Implied),
             /* AND_imm */ 0x29 => (Op::AND, 2, AddrMode::Immediate),
             /* ROL     */ 0x2A => (Op::ROL, 2, AddrMode::Implied),
             /* BIT_abs */ 0x2C => (Op::BIT, 4, AddrMode::Absolute),
             /* AND_abs */ 0x2D => (Op::AND, 4, AddrMode::Absolute),
             /* ROL_abs */ 0x2E => (Op::ROL, 6, AddrMode::Absolute),
             /* BMI_rel */ 0x30 => (Op::BMI, 2, AddrMode::Relative), // add 1 cycle if page boundary is crossed
             /* AND_izy */ 0x31 => (Op::AND, 5, AddrMode::IndirectIndexedY), // add 1 cycle if page boundary is crossed
             /* AND_zpx */ 0x35 => (Op::AND, 4, AddrMode::ZeropageIndexedX),
             /* ROL_zpx */ 0x36 => (Op::ROL, 6, AddrMode::ZeropageIndexedX),
             /* SEC     */ 0x38 => (Op::SEC, 2, AddrMode::Implied),
             /* AND_aby */ 0x39 => (Op::AND, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* AND_abx */ 0x3D => (Op::AND, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* ROL_abx */ 0x3E => (Op::ROL, 7, AddrMode::IndexedAbsoluteX),
             /* RTI     */ 0x40 => (Op::RTI, 6, AddrMode::Implied),
             /* EOR_izx */ 0x41 => (Op::EOR, 6, AddrMode::IndexedIndirectX),
             /* EOR_zp  */ 0x45 => (Op::EOR, 3, AddrMode::Zeropage),
             /* LSR_zp  */ 0x46 => (Op::LSR, 5, AddrMode::Zeropage),
             /* PHA     */ 0x48 => (Op::PHA, 3, AddrMode::Implied),
             /* EOR_imm */ 0x49 => (Op::EOR, 2, AddrMode::Immediate),
             /* LSR     */ 0x4A => (Op::LSR, 2, AddrMode::Implied),
             /* JMP_abs */ 0x4C => (Op::JMP, 3, AddrMode::Absolute),
             /* EOR_abs */ 0x4D => (Op::EOR, 4, AddrMode::Absolute),
             /* LSR_abs */ 0x4E => (Op::LSR, 6, AddrMode::Absolute),
             /* BVC_rel */ 0x50 => (Op::BVC, 2, AddrMode::Relative), // add 1 cycle if page boundary is crossed
             /* EOR_izy */ 0x51 => (Op::EOR, 5, AddrMode::IndirectIndexedY), // add 1 cycle if page boundary is crossed
             /* EOR_zpx */ 0x55 => (Op::EOR, 4, AddrMode::ZeropageIndexedX),
             /* LSR_zpx */ 0x56 => (Op::LSR, 6, AddrMode::ZeropageIndexedX),
             /* CLI     */ 0x58 => (Op::CLI, 2, AddrMode::Implied),
             /* EOR_aby */ 0x59 => (Op::EOR, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* EOR_abx */ 0x5D => (Op::EOR, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* LSR_abx */ 0x5E => (Op::LSR, 7, AddrMode::IndexedAbsoluteX),
             /* RTS     */ 0x60 => (Op::RTS, 6, AddrMode::Implied),
             /* ADC_izx */ 0x61 => (Op::ADC, 6, AddrMode::IndexedIndirectX),
             /* ADC_zp  */ 0x65 => (Op::ADC, 3, AddrMode::Zeropage),
             /* ROR_zp  */ 0x66 => (Op::ROR, 5, AddrMode::Zeropage),
             /* PLA     */ 0x68 => (Op::PLA, 4, AddrMode::Implied),
             /* ADC_imm */ 0x69 => (Op::ADC, 2, AddrMode::Immediate),
             /* ROR     */ 0x6A => (Op::ROR, 2, AddrMode::Implied),
             /* JMP_ind */ 0x6C => (Op::JMP, 5, AddrMode::AbsoluteIndirect),
             /* ADC_abs */ 0x6D => (Op::ADC, 4, AddrMode::Absolute),
             /* ROR_abs */ 0x6E => (Op::ROR, 6, AddrMode::Absolute),
             /* BVS_rel */ 0x70 => (Op::BVS, 2, AddrMode::Relative), // add 1 cycle if page boundary is crossed
             /* ADC_izy */ 0x71 => (Op::ADC, 5, AddrMode::IndirectIndexedY), // add 1 cycle if page boundary is crossed
             /* ADC_zpx */ 0x75 => (Op::ADC, 4, AddrMode::ZeropageIndexedX),
             /* ROR_zpx */ 0x76 => (Op::ROR, 6, AddrMode::ZeropageIndexedX),
             /* SEI     */ 0x78 => (Op::SEI, 2, AddrMode::Implied),
             /* ADC_aby */ 0x79 => (Op::ADC, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* ADC_abx */ 0x7D => (Op::ADC, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* ROR_abx */ 0x7E => (Op::ROR, 7, AddrMode::IndexedAbsoluteX),
             /* STA_izx */ 0x81 => (Op::STA, 6, AddrMode::IndexedIndirectX),
             /* STY_zp  */ 0x84 => (Op::STY, 3, AddrMode::Zeropage),
             /* STA_zp  */ 0x85 => (Op::STA, 3, AddrMode::Zeropage),
             /* STX_zp  */ 0x86 => (Op::STX, 3, AddrMode::Zeropage),
             /* DEY     */ 0x88 => (Op::DEY, 2, AddrMode::Implied),
             /* TXA     */ 0x8A => (Op::TXA, 2, AddrMode::Implied),
             /* STY_abs */ 0x8C => (Op::STY, 4, AddrMode::Absolute),
             /* STA_abs */ 0x8D => (Op::STA, 4, AddrMode::Absolute),
             /* STX_abs */ 0x8E => (Op::STX, 4, AddrMode::Absolute),
             /* BCC_rel */ 0x90 => (Op::BCC, 2, AddrMode::Relative), // add 1 cycle if page boundary is crossed
             /* STA_izy */ 0x91 => (Op::STA, 6, AddrMode::IndirectIndexedY),
             /* STY_zpx */ 0x94 => (Op::STY, 4, AddrMode::ZeropageIndexedX),
             /* STA_zpx */ 0x95 => (Op::STA, 4, AddrMode::ZeropageIndexedX),
             /* STX_zpy */ 0x96 => (Op::STX, 4, AddrMode::ZeropageIndexedY),
             /* TYA     */ 0x98 => (Op::TYA, 2, AddrMode::Implied),
             /* STA_aby */ 0x99 => (Op::STA, 5, AddrMode::IndexedAbsoluteY),
             /* TXS     */ 0x9A => (Op::TXS, 2, AddrMode::Implied),
             /* STA_abx */ 0x9D => (Op::STA, 5, AddrMode::IndexedAbsoluteX),
             /* LDY_imm */ 0xA0 => (Op::LDY, 2, AddrMode::Immediate),
             /* LDA_izx */ 0xA1 => (Op::LDA, 6, AddrMode::IndexedIndirectX),
             /* LDX_imm */ 0xA2 => (Op::LDX, 2, AddrMode::Immediate),
             /* LDY_zp  */ 0xA4 => (Op::LDY, 3, AddrMode::Zeropage),
             /* LDA_zp  */ 0xA5 => (Op::LDA, 3, AddrMode::Zeropage),
             /* LDX_zp  */ 0xA6 => (Op::LDX, 3, AddrMode::Zeropage),
             /* TAY     */ 0xA8 => (Op::TAY, 2, AddrMode::Implied),
             /* LDA_imm */ 0xA9 => (Op::LDA, 2, AddrMode::Immediate),
             /* TAX     */ 0xAA => (Op::TAX, 2, AddrMode::Implied),
             /* LDY_abs */ 0xAC => (Op::LDY, 4, AddrMode::Absolute),
             /* LDA_abs */ 0xAD => (Op::LDA, 4, AddrMode::Absolute),
             /* LDX_abs */ 0xAE => (Op::LDX, 4, AddrMode::Absolute),
             /* BCS_rel */ 0xB0 => (Op::BCS, 2, AddrMode::Relative), // add 1 cycle if page boundary is crossed
             /* LDA_izy */ 0xB1 => (Op::LDA, 5, AddrMode::IndirectIndexedY), // add 1 cycle if page boundary is crossed
             /* LDY_zpx */ 0xB4 => (Op::LDY, 4, AddrMode::ZeropageIndexedX),
             /* LDA_zpx */ 0xB5 => (Op::LDA, 4, AddrMode::ZeropageIndexedX),
             /* LDX_zpy */ 0xB6 => (Op::LDX, 4, AddrMode::ZeropageIndexedY),
             /* CLV     */ 0xB8 => (Op::CLV, 2, AddrMode::Implied),
             /* LDA_aby */ 0xB9 => (Op::LDA, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* TSX     */ 0xBA => (Op::TSX, 2, AddrMode::Implied),
             /* LDY_abx */ 0xBC => (Op::LDY, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* LDA_abx */ 0xBD => (Op::LDA, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* LDX_aby */ 0xBE => (Op::LDX, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* CPY_imm */ 0xC0 => (Op::CPY, 2, AddrMode::Immediate),
             /* CMP_izx */ 0xC1 => (Op::CMP, 6, AddrMode::IndexedIndirectX),
             /* CPY_zp  */ 0xC4 => (Op::CPY, 3, AddrMode::Zeropage),
             /* CMP_zp  */ 0xC5 => (Op::CMP, 3, AddrMode::Zeropage),
             /* DEC_zp  */ 0xC6 => (Op::DEC, 5, AddrMode::Zeropage),
             /* INY     */ 0xC8 => (Op::INY, 2, AddrMode::Implied),
             /* CMP_imm */ 0xC9 => (Op::CMP, 2, AddrMode::Immediate),
             /* DEX     */ 0xCA => (Op::DEX, 2, AddrMode::Implied),
             /* CPY_abs */ 0xCC => (Op::CPY, 4, AddrMode::Absolute),
             /* CMP_abs */ 0xCD => (Op::CMP, 4, AddrMode::Absolute),
             /* DEC_abs */ 0xCE => (Op::DEC, 6, AddrMode::Absolute),
             /* BNE_rel */ 0xD0 => (Op::BNE, 2, AddrMode::Relative), // add 1 cycle if page boundary is crossed
             /* CMP_izy */ 0xD1 => (Op::CMP, 5, AddrMode::IndirectIndexedY), // add 1 cycle if page boundary is crossed
             /* CMP_zpx */ 0xD5 => (Op::CMP, 4, AddrMode::ZeropageIndexedX),
             /* DEC_zpx */ 0xD6 => (Op::DEC, 6, AddrMode::ZeropageIndexedX),
             /* CLD     */ 0xD8 => (Op::CLD, 2, AddrMode::Implied),
             /* CMP_aby */ 0xD9 => (Op::CMP, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* CMP_abx */ 0xDD => (Op::CMP, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* DEC_abx */ 0xDE => (Op::DEC, 7, AddrMode::IndexedAbsoluteX),
             /* CPX_imm */ 0xE0 => (Op::CPX, 2, AddrMode::Immediate),
             /* SBC_izx */ 0xE1 => (Op::SBC, 6, AddrMode::IndexedIndirectX),
             /* CPX_zp  */ 0xE4 => (Op::CPX, 3, AddrMode::Zeropage),
             /* SBC_zp  */ 0xE5 => (Op::SBC, 3, AddrMode::Zeropage),
             /* INC_zp  */ 0xE6 => (Op::INC, 5, AddrMode::Zeropage),
             /* INX     */ 0xE8 => (Op::INX, 2, AddrMode::Implied),
             /* SBC_imm */ 0xE9 => (Op::SBC, 2, AddrMode::Immediate),
             /* NOP     */ 0xEA => (Op::NOP, 2, AddrMode::Implied),
             /* CPX     */ 0xEC => (Op::CPX, 4, AddrMode::Implied),
             /* SBC_abs */ 0xED => (Op::SBC, 4, AddrMode::Absolute),
             /* INC_abs */ 0xEE => (Op::INC, 6, AddrMode::Absolute),
             /* BEQ_rel */ 0xF0 => (Op::BEQ, 2, AddrMode::Relative), // add 1 cycle if page boundary is crossed
             /* SBC_izy */ 0xF1 => (Op::SBC, 5, AddrMode::IndirectIndexedY), // add 1 cycle if page boundary is crossed
             /* SBC_zpx */ 0xF5 => (Op::SBC, 4, AddrMode::ZeropageIndexedX),
             /* INC_zpx */ 0xF6 => (Op::INC, 6, AddrMode::ZeropageIndexedX),
             /* SED     */ 0xF8 => (Op::SED, 2, AddrMode::Implied),
             /* SBC_aby */ 0xF9 => (Op::SBC, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* SBC_abx */ 0xFD => (Op::SBC, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* INC_abx */ 0xFE => (Op::INC, 7, AddrMode::IndexedAbsoluteX),
             /* ** undocumented/forbidden instructions ** */
             /* HLT     */ 0x02 => (Op::HLT, 1, AddrMode::Implied),
             /* SLO_izx */ 0x03 => (Op::SLO, 8, AddrMode::IndexedIndirectX),
             /* NOP_zp  */ 0x04 => (Op::NOP, 3, AddrMode::Zeropage),
             /* SLO_zp  */ 0x07 => (Op::SLO, 5, AddrMode::Zeropage),
             /* ANC_imm */ 0x0B => (Op::ANC, 2, AddrMode::Immediate),
             /* NOP_abs */ 0x0C => (Op::NOP, 4, AddrMode::Absolute),
             /* SLO_abs */ 0x0F => (Op::SLO, 6, AddrMode::Absolute),
             /* HLT     */ 0x12 => (Op::HLT, 1, AddrMode::Implied),
             /* SLO_izy */ 0x13 => (Op::SLO, 8, AddrMode::IndirectIndexedY),
             /* NOP_zpx */ 0x14 => (Op::NOP, 4, AddrMode::ZeropageIndexedX),
             /* SLO_zpx */ 0x17 => (Op::SLO, 6, AddrMode::ZeropageIndexedX),
             /* NOP     */ 0x1A => (Op::NOP, 2, AddrMode::Implied),
             /* SLO_aby */ 0x1B => (Op::SLO, 7, AddrMode::IndexedAbsoluteY),
             /* NOP_abx */ 0x1C => (Op::NOP, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boudary is crossed
             /* SLO_abx */ 0x1F => (Op::SLO, 7, AddrMode::IndexedAbsoluteX),
             /* HLT     */ 0x22 => (Op::HLT, 1, AddrMode::Implied),
             /* RLA_izx */ 0x23 => (Op::RLA, 8, AddrMode::IndexedIndirectX),
             /* RLA_zp  */ 0x27 => (Op::RLA, 5, AddrMode::Zeropage),
             /* ANC_imm */ 0x2B => (Op::ANC, 2, AddrMode::Immediate),
             /* RLA_abs */ 0x2F => (Op::RLA, 6, AddrMode::Absolute),
             /* HLT     */ 0x32 => (Op::HLT, 1, AddrMode::Implied),
             /* RLA_izy */ 0x33 => (Op::RLA, 8, AddrMode::IndirectIndexedY),
             /* NOP_zpx */ 0x34 => (Op::NOP, 4, AddrMode::ZeropageIndexedX),
             /* RLA_zpx */ 0x37 => (Op::RLA, 6, AddrMode::ZeropageIndexedX),
             /* NOP     */ 0x3A => (Op::NOP, 2, AddrMode::Implied),
             /* RLA_aby */ 0x3B => (Op::RLA, 7, AddrMode::IndexedAbsoluteY),
             /* NOP_abx */ 0x3C => (Op::NOP, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* RLA_abx */ 0x3F => (Op::RLA, 7, AddrMode::IndexedAbsoluteX),
             /* HLT     */ 0x42 => (Op::HLT, 1, AddrMode::Implied),
             /* SRE_izx */ 0x43 => (Op::SRE, 8, AddrMode::IndexedIndirectX),
             /* NOP     */ 0x44 => (Op::NOP, 3, AddrMode::Implied),
             /* SRE_zp  */ 0x47 => (Op::SRE, 5, AddrMode::Zeropage),
             /* ALR_imm */ 0x4B => (Op::ALR, 2, AddrMode::Immediate),
             /* SRE_abs */ 0x4F => (Op::SRE, 6, AddrMode::Absolute),
             /* HLT     */ 0x52 => (Op::HLT, 1, AddrMode::Implied),
             /* SRE_izy */ 0x53 => (Op::SRE, 8, AddrMode::IndirectIndexedY),
             /* NOP_zpx */ 0x54 => (Op::NOP, 4, AddrMode::ZeropageIndexedX),
             /* SRE_zpx */ 0x57 => (Op::SRE, 6, AddrMode::ZeropageIndexedX),
             /* NOP     */ 0x5A => (Op::NOP, 2, AddrMode::Implied),
             /* SRE_aby */ 0x5B => (Op::SRE, 7, AddrMode::IndexedAbsoluteY),
             /* NOP_abx */ 0x5C => (Op::NOP, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* SRE_abx */ 0x5F => (Op::SRE, 7, AddrMode::IndexedAbsoluteX),
             /* HLT     */ 0x62 => (Op::HLT, 1, AddrMode::Implied),
             /* RRA_izx */ 0x63 => (Op::RRA, 8, AddrMode::IndexedIndirectX),
             /* NOP_zp  */ 0x64 => (Op::NOP, 3, AddrMode::Zeropage),
             /* RRA_zp  */ 0x67 => (Op::RRA, 5, AddrMode::Zeropage),
             /* ARR     */ 0x6B => (Op::ARR, 2, AddrMode::Implied),
             /* RRA_abs */ 0x6F => (Op::RRA, 6, AddrMode::Absolute),
             /* HLT     */ 0x72 => (Op::HLT, 1, AddrMode::Implied),
             /* RRA_izy */ 0x73 => (Op::RRA, 8, AddrMode::IndirectIndexedY),
             /* NOP_zpx */ 0x74 => (Op::NOP, 4, AddrMode::ZeropageIndexedX),
             /* RRA_zpx */ 0x77 => (Op::RRA, 6, AddrMode::ZeropageIndexedX),
             /* NOP     */ 0x7A => (Op::NOP, 2, AddrMode::Implied),
             /* RRA_aby */ 0x7B => (Op::RRA, 7, AddrMode::IndexedAbsoluteY),
             /* NOP_abx */ 0x7C => (Op::NOP, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* RRA_abx */ 0x7F => (Op::RRA, 7, AddrMode::IndexedAbsoluteX),
             /* NOP_imm */ 0x80 => (Op::NOP, 2, AddrMode::Immediate),
             /* NOP_imm */ 0x82 => (Op::NOP, 2, AddrMode::Immediate),
             /* SAX_izx */ 0x83 => (Op::SAX, 6, AddrMode::IndexedIndirectX),
             /* SAX_zp  */ 0x87 => (Op::SAX, 3, AddrMode::Zeropage),
             /* NOP_imm */ 0x89 => (Op::NOP, 2, AddrMode::Immediate),
             /* XAA_imm */ 0x8B => (Op::XAA, 2, AddrMode::Immediate),
             /* SAX_abs */ 0x8F => (Op::SAX, 4, AddrMode::Absolute),
             /* HLT     */ 0x92 => (Op::HLT, 1, AddrMode::Implied),
             /* AHX_izy */ 0x93 => (Op::AHX, 6, AddrMode::IndirectIndexedY),
             /* SAX_zpy */ 0x97 => (Op::SAX, 4, AddrMode::ZeropageIndexedY),
             /* TAS_aby */ 0x9B => (Op::TAS, 5, AddrMode::IndexedAbsoluteY),
             /* SHY_abx */ 0x9C => (Op::SHY, 5, AddrMode::IndexedAbsoluteX),
             /* SHX_aby */ 0x9E => (Op::SHX, 5, AddrMode::IndexedAbsoluteY),
             /* AHX_aby */ 0x9F => (Op::AHX, 5, AddrMode::IndexedAbsoluteY),
             /* LAX_izx */ 0xA3 => (Op::LAX, 6, AddrMode::IndexedIndirectX),
             /* LAX_zp  */ 0xA7 => (Op::LAX, 3, AddrMode::Zeropage),
             /* LAX_imm */ 0xAB => (Op::LAX, 2, AddrMode::Immediate),
             /* LAX_abs */ 0xAF => (Op::LAX, 4, AddrMode::Absolute),
             /* HLT     */ 0xB2 => (Op::HLT, 1, AddrMode::Implied),
             /* LAX_izy */ 0xB3 => (Op::LAX, 5, AddrMode::IndirectIndexedY), // add 1 cycle if page boundary is crossed
             /* LAX_zpy */ 0xB7 => (Op::LAX, 4, AddrMode::ZeropageIndexedY),
             /* LAS_aby */ 0xBB => (Op::LAS, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* LAX_aby */ 0xBF => (Op::LAX, 4, AddrMode::IndexedAbsoluteY), // add 1 cycle if page boundary is crossed
             /* NOP_imm */ 0xC2 => (Op::NOP, 2, AddrMode::Immediate),
             /* DCP_izx */ 0xC3 => (Op::DCP, 8, AddrMode::IndexedIndirectX),
             /* DCP_zp  */ 0xC7 => (Op::DCP, 5, AddrMode::Zeropage),
             /* AXS_imm */ 0xCB => (Op::AXS, 2, AddrMode::Immediate),
             /* DCP_abs */ 0xCF => (Op::DCP, 6, AddrMode::Absolute),
             /* HLT     */ 0xD2 => (Op::HLT, 1, AddrMode::Implied),
             /* DCP_izy */ 0xD3 => (Op::DCP, 8, AddrMode::IndirectIndexedY),
             /* NOP_zpx */ 0xD4 => (Op::NOP, 4, AddrMode::ZeropageIndexedX),
             /* DCP_zpx */ 0xD7 => (Op::DCP, 6, AddrMode::ZeropageIndexedX),
             /* NOP     */ 0xDA => (Op::NOP, 2, AddrMode::Implied),
             /* DCP_aby */ 0xDB => (Op::DCP, 7, AddrMode::IndexedAbsoluteY),
             /* NOP_abx */ 0xDC => (Op::NOP, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* DCP_abx */ 0xDF => (Op::DCP, 7, AddrMode::IndexedAbsoluteX),
             /* NOP_imm */ 0xE2 => (Op::NOP, 2, AddrMode::Immediate),
             /* ISC_izx */ 0xE3 => (Op::ISC, 8, AddrMode::IndexedIndirectX),
             /* ISC_zp  */ 0xE7 => (Op::ISC, 5, AddrMode::Zeropage),
             /* SBC_imm */ 0xEB => (Op::SBC, 2, AddrMode::Immediate),
             /* ISC_abs */ 0xEF => (Op::ISC, 6, AddrMode::Absolute),
             /* HLT     */ 0xF2 => (Op::HLT, 1, AddrMode::Implied),
             /* ISC_izy */ 0xF3 => (Op::ISC, 8, AddrMode::IndirectIndexedY),
             /* NOP_zpx */ 0xF4 => (Op::NOP, 4, AddrMode::ZeropageIndexedX),
             /* ISC_zpx */ 0xF7 => (Op::ISC, 6, AddrMode::ZeropageIndexedX),
             /* NOP     */ 0xFA => (Op::NOP, 2, AddrMode::Implied),
             /* ISC_aby */ 0xFB => (Op::ISC, 7, AddrMode::IndexedAbsoluteY),
             /* NOP_abx */ 0xFC => (Op::NOP, 4, AddrMode::IndexedAbsoluteX), // add 1 cycle if page boundary is crossed
             /* ISC_abx */ 0xFF => (Op::ISC, 7, AddrMode::IndexedAbsoluteX),
             
             _ => return None
         })
}

// fetch operand address 
fn get_operand_addr(mode: AddrMode, cpu: &mut cpu::CPU) -> u16
{
    match mode
    {
        AddrMode::Implied           => panic!("Trying to fetch operand addr in implied addr mode."),
        AddrMode::Accumulator       => panic!("Trying to fetch operand addr in accumulator addr mode."),
        AddrMode::Immediate         => panic!("Trying to fetch operand addr in immediate addr mode."),
        AddrMode::Absolute          => cpu.next_word(),
        AddrMode::IndexedAbsoluteX  => {
            let nw = cpu.next_word();
            cpu.mem.read_word_le(nw) + cpu.X as u16 },
        AddrMode::IndexedAbsoluteY  => {
            let nw = cpu.next_word();
            cpu.mem.read_word_le(nw) + cpu.Y as u16 },
        AddrMode::Zeropage          => cpu.next_byte() as u16,
        AddrMode::ZeropageIndexedX  => {
            let nb = cpu.next_byte();
            cpu.mem.read_word_le(nb as u16) + cpu.X as u16 },
        AddrMode::ZeropageIndexedY  => {
            let nb = cpu.next_byte();
            cpu.mem.read_word_le(nb as u16) + cpu.Y as u16 },
        AddrMode::Relative          => {
            let offset: i8 = cpu.next_byte() as i8;
            (cpu.PC as i16 + offset as i16) as u16 },
        AddrMode::AbsoluteIndirect  => panic!("abs_ind not implemented"),
        AddrMode::IndexedIndirectX  => {
            let nb = cpu.next_byte();
            cpu.mem.read_word_le(nb as u16) + cpu.X as u16 },
        AddrMode::IndirectIndexedY  => {
            let nb = cpu.next_byte();
            let addr = cpu.mem.read_word_le(nb as u16);
            cpu.mem.read_word_le(addr) + cpu.Y as u16 },                        
    }    
}

// fetch operand value
pub fn get_operand(mode: AddrMode, cpu: &mut cpu::CPU) -> u8
{
    match mode
    {
        AddrMode::Implied     => panic!("Trying to fetch operand in implied addr mode."),
        AddrMode::Accumulator => cpu.A,
        AddrMode::Immediate   => cpu.next_byte(),
        _ => {
            let opAddr = get_operand_addr(mode, cpu);
            cpu.mem.read_byte(opAddr)
        }
    }    
}

// set operand value
pub fn set_operand(mode: AddrMode, cpu: &mut cpu::CPU, value: u8)
{
    match mode
    {
        AddrMode::Implied     => panic!("Trying to set operand in implied addr mode."),        
        AddrMode::Accumulator => cpu.A = value,
        AddrMode::Immediate   => panic!("Trying to set operand in immediate addr mode."),
        AddrMode::Relative    => panic!("Trying to set operand in relative addr mode."),
        _ => {
            let opAddr = get_operand_addr(mode, cpu);
            cpu.mem.write_byte(opAddr, value)
        }
    }
}
