use crate::bus::Bus;

const NMI_VECTOR: u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;
const IRQ_VECTOR: u16 = 0xfffe;

// Status flag bitmask (6502)
const FLAG_NEGATIVE: u8 = 0b1000_0000;
const FLAG_OVERFLOW: u8 = 0b0100_0000;
const FLAG_UNUSED: u8 = 0b0010_0000;
const FLAG_BREAK: u8 = 0b0001_0000;
const FLAG_DECIMAL: u8 = 0b0000_1000;
const FLAG_INTERRUPT: u8 = 0b0000_0100;
const FLAG_ZERO: u8 = 0b0000_0010;
const FLAG_CARRY: u8 = 0b0000_0001;

/// Represents an operand fetched by an addressing mode helper.
#[derive(Debug, Clone, Copy)]
enum Operand {
    /// Immediate value (value already resolved)
    Immediate(u8),

    /// Memory address (for reads or writes)
    Address(u16),

    /// Accumulator implicit operand (for instructions that operate on A)
    Accumulator,
}

/// The CPU struct represents the central processing unit.
pub struct CPU {
    /// Accumulator register (A), used for arithmetic and logic operations.
    pub register_a: u8,

    /// Index register X, used for indexing and loop counters.
    pub register_x: u8,

    /// Index register Y (not present previously) — add it now for addressing modes.
    pub register_y: u8,

    /// Status register holding CPU flags:
    /// - Bit 7: Negative flag (N)
    /// - Bit 1: Zero flag (Z)
    /// - Bit 0: Carry flag (C)
    /// and others (not fully implemented here).
    pub status: u8,

    /// Program counter (PC), points to the next instruction address.
    pub program_counter: u16,

    /// Stack Pointer (SP), points to the current top of the stack (0x0100-0x01FF).
    /// Stack grows downward; used by PHA/PLA/JSR/RTS/RTI/PHP/PLP instructions.
    pub stack_pointer: u8,

    /// Pending interrupt requests (NMI cannot be masked, IRQ can be).
    pub nmi_pending: bool,
    pub irq_pending: bool,

    /// Accumulated CPU cycles executed (host-side counter for emulation timing)
    pub cycles: u64,
}

impl CPU {
    /// Creates a new CPU instance with all registers and flags initialized to zero.
    pub fn new() -> Self {
        let mut cpu = CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            stack_pointer: 0,
            nmi_pending: false,
            irq_pending: false,
            cycles: 0,
        };

        // Caller MUST call reset(bus) before running, but we clear state here.
        cpu.stack_pointer = 0xfd;
        cpu.status = FLAG_UNUSED | FLAG_INTERRUPT; // matches reset()
        cpu
    }

    /// Returns true if the given status flag is set.
    fn get_flag(&self, flag: u8) -> bool {
        (self.status & flag) != 0
    }

    /// Sets or clears a status flag depending on value.
    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.status |= flag;
        } else {
            self.status &= !flag;
        }
    }

    /// Core binary ADC operation (no BCD handling).
    /// Adds `operand` + carry to the accumulator and updates flags.
    /// Returns the new accumulator value (8-bit).
    fn adc_binary(&mut self, operand: u8) -> u8 {
        // If Decimal mode is set, we don't support it yet.
        if self.get_flag(FLAG_DECIMAL) {
            // TODO: implement BDC mode
            panic!("ADC in decimal (BCD) mode not implemented");
        }

        let a = self.register_a as u16;
        let m = operand as u16;
        let carry_in = if self.get_flag(FLAG_CARRY) {
            1u16
        } else {
            0u16
        };

        let sum = a + m + carry_in; // 9-bit result possible

        // Set carry flag if result exceeds 0xFF
        self.set_flag(FLAG_CARRY, sum > 0xff);

        let result = (sum & 0xff) as u8;

        // Overflow: set if sign changed unexpectedly:
        // ( (~(A ^ M) & (A ^ R)) & 0x80 ) != 0
        let overflow = (!(self.register_a ^ operand) & (self.register_a ^ result) & 0x80) != 0;

        self.set_flag(FLAG_OVERFLOW, overflow);

        // Update Zero and Negative flags
        self.update_zero_and_negative_flags(result);

        result
    }

    /// Core binary SBC operation (no BCD handling).
    /// Subtracts `operand` + (1 - carry) from accumulator and updates flags.
    /// Returns the new accumulator value (8-bit).
    fn sbc_binary(&mut self, operand: u8) -> u8 {
        if self.get_flag(FLAG_DECIMAL) {
            // TODO: implement BCD SBC
            panic!("SBC in decimal (BCD) mode not implemented");
        }

        // Implement SBC as A + (~operand) + carry
        let inverted = !operand as u16;
        let a = self.register_a as u16;
        let carry_in = if self.get_flag(FLAG_CARRY) {
            1u16
        } else {
            0u16
        };

        let sum = a + inverted + carry_in; // 9-bit result

        // Carry for subtraction: if result > 0xFF then borrow didn't occur -> set carry
        self.set_flag(FLAG_CARRY, sum > 0xff);

        let result = (sum & 0xff) as u8;

        // Overflow detection same trick as ADC when using two's complement math
        let overflow = (!(self.register_a ^ operand) & (self.register_a ^ result) & 0x80) != 0;
        self.set_flag(FLAG_OVERFLOW, overflow);

        self.update_zero_and_negative_flags(result);

        result
    }

    /// Resets the CPU to its initial power-on state.
    /// This simulates the 6502 RESET interrupt, which initializes
    /// registers and loads the starting address from the RESET vector/
    pub fn reset(&mut self, bus: &mut impl Bus) {
        // Program Counter is loaded from the RESET vector ($FFFC-$FFFD)
        let lo = bus.read(RESET_VECTOR) as u16;
        let hi = bus.read(RESET_VECTOR + 1) as u16;
        self.program_counter = (hi << 8) | lo;

        // Stack pointer is initialized to 0xFD on startup
        self.stack_pointer = 0xfd;

        // Status register:
        // Bit 2 (Interrupt Disable flag) must be set during reset
        // Other bits are typically cleared (except unused bit 5 which is always 1)
        self.status = FLAG_INTERRUPT | FLAG_UNUSED; // IRQ disabled, unused flag set

        // Registers A and X are not defined after reset on real hardware,
        // but setting them to 0 ensures stable emulation behavior/
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
    }

    #[allow(dead_code)]

    /// Requests a maskable interrupt (IRQ). Ignored if I flag is set.
    pub fn trigger_irq(&mut self) {
        self.irq_pending = true;
    }

    /// Handles an interrupt by pushing PC and status, clearing the break bit,
    /// and loading a new PC from the interrupt vector.
    fn handle_interrupt(&mut self, bus: &mut impl Bus, vector: u16) {
        // Push PC onto stack (high, then low)
        let pc = self.program_counter;
        self.push_word(bus, pc);

        // Push status (B flag cleared on actual interrupts)
        let flags = (self.status & !FLAG_BREAK) | FLAG_UNUSED;
        self.push_byte(bus, flags);

        // Set interrupt disable flag
        self.status |= 0b0000_0100;

        // Load new PC from vector
        let lo = bus.read(vector) as u16;
        let hi = bus.read(vector + 1) as u16;
        self.program_counter = (hi << 8) | lo;
    }

    #[allow(dead_code)]

    /// Requests a non-maskable interrupt (NMI). Always taken.
    pub fn trigger_nmi(&mut self) {
        self.nmi_pending = true;
    }

    /// Computes the absolute memory address of the stack location pointed by 'stack_pointer'.
    /// Stack resides in page 0x0100 (0x0100 - 0x01FF).
    fn stack_address(&self) -> u16 {
        0x0100 | (self.stack_pointer as u16)
    }

    /// Pushes a byte onto the stack.
    /// Decrements `stack_pointer` after writing (stack grows downward).
    pub fn push_byte(&mut self, bus: &mut impl Bus, value: u8) {
        let addr = self.stack_address();
        bus.write(addr, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1); // wrap-around at 0x00 -> 0xFF
    }

    /// Pops a byte from the stack.
    /// Increments `stack_pointer` before reading.
    pub fn pop_byte(&mut self, bus: &mut impl Bus) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1); // wrap-around at 0xFF -> 0x00
        let addr = self.stack_address();
        bus.read(addr)
    }

    /// Pushes a 16-bit value onto the stack.
    /// High byte is pushed first, then low byte.
    /// Used by JSR and interrupt routines.
    pub fn push_word(&mut self, bus: &mut impl Bus, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0xff) as u8;

        self.push_byte(bus, high);
        self.push_byte(bus, low);
    }

    /// Pops a 16-bit value from the stack.
    /// Low byte is popped first, then high byte.
    /// Used by RTS and RTI instructions.
    pub fn pop_word(&mut self, bus: &mut impl Bus) -> u16 {
        let low = self.pop_byte(bus) as u16;
        let high = self.pop_byte(bus) as u16;
        (high << 8) | low
    }

    /// Updates the zero and negative flags based on the `result` byte.
    /// - Zero flag is set if `result` is zero.
    /// - Negative flag is set if the most significant bit (bit 7) is set.
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= FLAG_ZERO; // Set zero flag
        } else {
            self.status &= !FLAG_ZERO; // Clear zero flag
        }

        if (result & FLAG_NEGATIVE) != 0 {
            self.status |= FLAG_NEGATIVE; // Set negative flag
        } else {
            self.status &= !FLAG_NEGATIVE; // Clear negative flag
        }
    }

    /// Adjusts the program counter by a signed offset for branching instructions.
    fn branch(&mut self, offset: i8) {
        let pc = self.program_counter as i32;
        let offset = offset as i32;
        self.program_counter = (pc + offset) as u16;
    }

    //
    // ---- Addressing mode helpers ----
    //
    // Each helper fetches/decodes the operand according to the addressing mode,
    // advances the program counter appropriately, and returns an Operand.
    // Some functions return `page_crossed` which is useful later for cycle accounting.
    //

    /// Read a little-endian 16-bit value from memory at `addr` (lo then hi).
    fn read_u16(&self, bus: &impl Bus, addr: u16) -> u16 {
        let lo = bus.read(addr) as u16;
        let hi = bus.read(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    /// Immediate: operand is the next byte. PC advances by 2.
    fn fetch_immediate(&mut self, bus: &impl Bus) -> Operand {
        let value = bus.read(self.program_counter.wrapping_add(1));
        self.program_counter = self.program_counter.wrapping_add(2);
        Operand::Immediate(value)
    }

    /// Implied / accumulator: no operand bytes (PC +1)
    fn fetch_implied(&mut self) -> Operand {
        self.program_counter = self.program_counter.wrapping_add(1);
        Operand::Accumulator
    }

    /// Zero Page: single byte address in page $00. PC +2.
    fn fetch_zeropage(&mut self, bus: &impl Bus) -> Operand {
        let addr = bus.read(self.program_counter.wrapping_add(1)) as u16;
        self.program_counter = self.program_counter.wrapping_add(2);
        Operand::Address(addr & 0x00ff)
    }

    /// Zero Page,X: zero page address + X, wraps within zero page. PC +2.
    fn fetch_zeropage_x(&mut self, bus: &impl Bus) -> Operand {
        let base = bus.read(self.program_counter.wrapping_add(1)) as u8;
        let addr = base.wrapping_add(self.register_x) as u16;
        self.program_counter = self.program_counter.wrapping_add(2);
        Operand::Address(addr as u16 & 0x00ff)
    }

    /// Zero Page,Y: zero page address + Y, wraps within zero page. PC +2.
    fn fetch_zeropage_y(&mut self, bus: &impl Bus) -> Operand {
        let base = bus.read(self.program_counter.wrapping_add(1)) as u8;
        let addr = base.wrapping_add(self.register_y) as u16;
        self.program_counter = self.program_counter.wrapping_add(2);
        Operand::Address(addr as u16 & 0x00ff)
    }

    /// Absolute: 16-bit address (lo/hi). PC +3.
    fn fetch_absolute(&mut self, bus: &impl Bus) -> Operand {
        let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
        let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
        let addr = (hi << 8) | lo;
        self.program_counter = self.program_counter.wrapping_add(3);
        Operand::Address(addr)
    }

    /// Absolute,X: absolute + X. Returns operand and `page_crossed` flag. PC +3.
    fn fetch_absolute_x(&mut self, bus: &impl Bus) -> (Operand, bool) {
        let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
        let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
        let base = (hi << 8) | lo;
        let addr = base.wrapping_add(self.register_x as u16);
        let page_crossed = (base & 0xff00) != (addr & 0xff00);
        self.program_counter = self.program_counter.wrapping_add(3);
        (Operand::Address(addr), page_crossed)
    }

    /// Absolute,Y: absolute + Y. Returns operand and `page_crossed` flag. PC +3.
    fn fetch_absolute_y(&mut self, bus: &impl Bus) -> (Operand, bool) {
        let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
        let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
        let base = (hi << 8) | lo;
        let addr = base.wrapping_add(self.register_y as u16);
        let page_crossed = (base & 0xff00) != (addr & 0xff00);
        self.program_counter = self.program_counter.wrapping_add(3);
        (Operand::Address(addr), page_crossed)
    }

    /// (Indirect,X) - "Indexed Indirect"
    /// Effective address = read16((zp + X) & 0xFF)
    /// PC +2.
    fn fetch_indexed_indirect(&mut self, bus: &impl Bus) -> Operand {
        let zp = bus.read(self.program_counter.wrapping_add(1));
        let ptr = zp.wrapping_add(self.register_x) as u16 & 0x00ff;
        // zero page wrap for pointer low/high
        let lo = bus.read(ptr) as u16;
        let hi = bus.read((ptr.wrapping_add(1) & 0x00ff)) as u16;
        let addr = (hi << 8) | lo;
        self.program_counter = self.program_counter.wrapping_add(2);
        Operand::Address(addr)
    }

    /// (Indirect),Y - "Indirect Indexed"
    /// Effective address = read16(zp) + Y
    /// Returns operand and page_cross flag; PC +2.
    fn fetch_indirect_indexed(&mut self, bus: &impl Bus) -> (Operand, bool) {
        let zp = bus.read(self.program_counter.wrapping_add(1)) as u16 & 0x00ff;
        let lo = bus.read(zp) as u16;
        let hi = bus.read((zp.wrapping_add(1) & 0x00ff)) as u16;
        let base = (hi << 8) | lo;
        let addr = base.wrapping_add(self.register_y as u16);
        let page_crossed = (base & 0xff00) != (addr & 0xff00);
        self.program_counter = self.program_counter.wrapping_add(2);
        (Operand::Address(addr), page_crossed)
    }

    /// Indirect addressing used by JMP (indirect). Implements the 6502 page-boundary bug:
    /// If the indirect vector falls on a page boundary (xxFF), the high byte is fetched from xx00 instead of xx+1 00.
    fn fetch_indirect_jmp(&mut self, bus: &impl Bus) -> Operand {
        let ptr_lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
        let ptr_hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
        let ptr = (ptr_hi << 8) | ptr_lo;

        // 6502 bug: if low byte is 0xFF, the high byte wraps within same page
        let lo = bus.read(ptr) as u16;
        let hi_addr = if (ptr & 0x00ff) == 0x00ff {
            // wrap within page
            (ptr & 0xff00)
        } else {
            ptr.wrapping_add(1)
        };
        let hi = bus.read(hi_addr) as u16;
        let addr = (hi << 8) | lo;
        self.program_counter = self.program_counter.wrapping_add(3);
        Operand::Address(addr)
    }

    /// Relative addressing: used by branch instructions; fetch signed offset and advance PC by 2.
    fn fetch_relative_offset(&mut self, bus: &impl Bus) -> i8 {
        let offset = bus.read(self.program_counter.wrapping_add(1)) as i8;
        self.program_counter = self.program_counter.wrapping_add(2);
        offset
    }

    //
    // ---- End addressing helpers ----
    //

    /// Adds cycles for branches: +1 when taken, +1 additional if page crossed.
    /// Assumes PC has already been advanced past the branch operand (i.e., after fetch_relative_offset).
    fn branch_with_cycles(&mut self, offset: i8) {
        // old_pc is the PC *after* the operand (this matches your fetch_relative_offset behavior)
        let old_pc = self.program_counter;
        // compute new PC by adding signed offset
        let new_pc = (old_pc as i32 + offset as i32) as u16;

        // set PC to new location
        self.program_counter = new_pc;

        // branch taken: +1 cycle
        self.cycles = self.cycles.wrapping_add(1);

        // if page crossed, add another cycle
        let crossed = (old_pc & 0xff00) != (new_pc & 0xff00);
        if crossed {
            self.cycles = self.cycles.wrapping_add(1);
        }
    }

    /// Runs the CPU emulation loop, fetching and executing instructions from the bus.
    /// The loop continues until a BRK (0x00) instruction is encountered.
    /// The CPU reads instructions from memory via the Bus trait interface.
    pub fn run(&mut self, bus: &mut impl Bus) {
        loop {
            self.run_once(bus);
        }
    }

    /// Runs the CPU emulation step by step.
    pub fn run_once(&mut self, bus: &mut impl Bus) {
        // Handle interrupts before executing next instruction
        if self.nmi_pending {
            self.nmi_pending = false;
            self.handle_interrupt(bus, NMI_VECTOR);
        } else if self.irq_pending && !self.get_flag(FLAG_INTERRUPT) {
            self.irq_pending = false;
            self.handle_interrupt(bus, IRQ_VECTOR);
        }

        let opcode = bus.read(self.program_counter);

        match opcode {
            0xa9 => {
                // LDA Immediate: Load accumulator with immediate value
                if let Operand::Immediate(value) = self.fetch_immediate(bus) {
                    self.register_a = value;
                    self.update_zero_and_negative_flags(self.register_a);
                } else {
                    unreachable!();
                }
            }
            0xa5 => {
                // LDA Zero Page
                if let Operand::Address(addr) = self.fetch_zeropage(bus) {
                    self.register_a = bus.read(addr);
                    self.update_zero_and_negative_flags(self.register_a);
                }
            }
            0xb5 => {
                // LDA Zero Page,X
                if let Operand::Address(addr) = self.fetch_zeropage_x(bus) {
                    self.register_a = bus.read(addr);
                    self.update_zero_and_negative_flags(self.register_a);
                }
            }
            0xad => {
                // LDA Absolute
                if let Operand::Address(addr) = self.fetch_absolute(bus) {
                    self.register_a = bus.read(addr);
                    self.update_zero_and_negative_flags(self.register_a);
                }
            }
            0xbd => {
                // LDA Absolute,X
                let (op, page_crossed) = self.fetch_absolute_x(bus);
                if let Operand::Address(addr) = op {
                    self.register_a = bus.read(addr);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                // page-cross penalty if accessed across page boundary
                if page_crossed {
                    self.cycles = self.cycles.wrapping_add(1);
                }
            }
            0xb9 => {
                // LDA Absolute,Y
                let (op, page_crossed) = self.fetch_absolute_y(bus);
                if let Operand::Address(addr) = op {
                    self.register_a = bus.read(addr);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                if page_crossed {
                    self.cycles = self.cycles.wrapping_add(1);
                }
            }
            0xa1 => {
                // LDA (Indirect,X)
                if let Operand::Address(addr) = self.fetch_indexed_indirect(bus) {
                    self.register_a = bus.read(addr);
                    self.update_zero_and_negative_flags(self.register_a);
                }
            }
            0xb1 => {
                // LDA (Indirect),Y
                let (op, page_crossed) = self.fetch_indirect_indexed(bus);
                if let Operand::Address(addr) = op {
                    self.register_a = bus.read(addr);
                    self.update_zero_and_negative_flags(self.register_a);
                }
                if page_crossed {
                    self.cycles = self.cycles.wrapping_add(1);
                }
            }
            0xaa => {
                // TAX: Transfer accumulator to X register (implied)
                self.fetch_implied(); // advance PC
                self.register_x = self.register_a;
                self.update_zero_and_negative_flags(self.register_x);
            }
            0xe8 => {
                // INX: Increment X register (implied)
                self.fetch_implied();
                self.register_x = self.register_x.wrapping_add(1);
                self.update_zero_and_negative_flags(self.register_x);
            }
            0x8d => {
                // STA Absolute: Store accumulator to memory address
                if let Operand::Address(addr) = self.fetch_absolute(bus) {
                    bus.write(addr, self.register_a);
                }
            }
            0x85 => {
                // STA Zero Page
                if let Operand::Address(addr) = self.fetch_zeropage(bus) {
                    bus.write(addr, self.register_a);
                }
            }
            0x95 => {
                // STA Zero Page,X
                if let Operand::Address(addr) = self.fetch_zeropage_x(bus) {
                    bus.write(addr, self.register_a);
                }
            }
            0x4c => {
                // JMP Absolute: Jump to new address
                if let Operand::Address(addr) = self.fetch_absolute(bus) {
                    self.program_counter = addr;
                }
            }
            0x6c => {
                // JMP Indirect
                if let Operand::Address(addr) = self.fetch_indirect_jmp(bus) {
                    self.program_counter = addr;
                }
            }
            0xf0 => {
                // BEQ: Branch if equal (zero flag set)
                let offset = self.fetch_relative_offset(bus);
                if self.get_flag(FLAG_ZERO) {
                    self.branch_with_cycles(offset);
                }
            }
            0xd0 => {
                // BNE: Branch if not equal (zero flag clear)
                let offset = self.fetch_relative_offset(bus);
                if !self.get_flag(FLAG_ZERO) {
                    self.branch_with_cycles(offset);
                }
            }
            0x90 => {
                // BCC: Branch if carry clear
                let offset = self.fetch_relative_offset(bus);
                if !self.get_flag(FLAG_CARRY) {
                    self.branch_with_cycles(offset);
                }
            }
            0xb0 => {
                // BCS: Branch if carry set
                let offset = self.fetch_relative_offset(bus);
                if self.get_flag(FLAG_CARRY) {
                    self.branch_with_cycles(offset);
                }
            }
            0x30 => {
                // BMI: Branch if negative set
                let offset = self.fetch_relative_offset(bus);
                if self.get_flag(FLAG_NEGATIVE) {
                    self.branch_with_cycles(offset);
                }
            }
            0x10 => {
                // BPL: Branch if negative clear
                let offset = self.fetch_relative_offset(bus);
                if !self.get_flag(FLAG_NEGATIVE) {
                    self.branch_with_cycles(offset);
                }
            }
            0x00 => {
                // BRK: Force interrupt
                // On BRK the return address pushed is PC+2 and break flag is set for pushed flags.
                // We emulate the basic behavior here.
                self.program_counter = self.program_counter.wrapping_add(1);

                // Push PC and status (break flag set)
                self.push_word(bus, self.program_counter);
                self.push_byte(bus, self.status | FLAG_BREAK | FLAG_UNUSED);

                self.set_flag(FLAG_INTERRUPT, true);
                self.set_flag(FLAG_BREAK, false);

                // Jump to IRQ/BRK vector
                let lo = bus.read(IRQ_VECTOR) as u16;
                let hi = bus.read(IRQ_VECTOR + 1) as u16;
                self.program_counter = (hi << 8) | lo;

                return;
            }
            0x48 => {
                // PHA: Push accumulator to stack
                self.fetch_implied();
                self.push_byte(bus, self.register_a);
            }
            0x68 => {
                // PLA: Pull accumulator from stack
                self.fetch_implied();
                self.register_a = self.pop_byte(bus);
                self.update_zero_and_negative_flags(self.register_a);
            }
            0x08 => {
                // PHP: Push processor status onto the stack
                let flags = self.status | 0b0011_0000;
                self.push_byte(bus, flags);
                self.program_counter = self.program_counter.wrapping_add(1);
            }
            0x28 => {
                // PLP: Pull processor status from the stack
                let value = self.pop_byte(bus);
                self.status = (value & 0b1100_1111) | 0b0010_0000;
                self.program_counter = self.program_counter.wrapping_add(1);
            }
            0x20 => {
                // JSR Absolute: Jump to subroutine
                let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
                let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
                let target = (hi << 8) | lo;

                // Return address = address of last byte of JSR instruction
                let return_addr = self.program_counter.wrapping_add(2);

                self.push_word(bus, return_addr);

                // Jump to subroutine
                self.program_counter = target;
            }
            0x60 => {
                // RTS: Return from subroutine
                let lo = self.pop_byte(bus) as u16;
                let hi = self.pop_byte(bus) as u16;
                let return_addr = (hi << 8) | lo;

                self.program_counter = return_addr.wrapping_add(1);
            }
            0x40 => {
                // RTI: Return from interrupt
                self.status = (self.pop_byte(bus) & 0b1100_1111) | 0b0010_0000;
                self.program_counter = self.pop_word(bus);
            }
            0x69 => {
                // ADC - Add with Carry (immediate)
                if let Operand::Immediate(operand) = self.fetch_immediate(bus) {
                    self.register_a = self.adc_binary(operand);
                }
            }
            0x6d => {
                // ADC - Add with Carry (absolute)
                if let Operand::Address(addr) = self.fetch_absolute(bus) {
                    let operand = bus.read(addr);
                    self.register_a = self.adc_binary(operand);
                }
            }
            0x65 => {
                // ADC - Add with Carry (Zero Page)
                if let Operand::Address(addr) = self.fetch_zeropage(bus) {
                    let operand = bus.read(addr);
                    self.register_a = self.adc_binary(operand);
                }
            }
            0x75 => {
                // ADC - Add with Carry (Zero Page,X)
                if let Operand::Address(addr) = self.fetch_zeropage_x(bus) {
                    let operand = bus.read(addr);
                    self.register_a = self.adc_binary(operand);
                }
            }
            0xe9 => {
                // SBC - Subtract with Borrow (immediate)
                if let Operand::Immediate(operand) = self.fetch_immediate(bus) {
                    self.register_a = self.sbc_binary(operand);
                }
            }
            0xed => {
                // SBC - Subtract with Borrow (absolute)
                if let Operand::Address(addr) = self.fetch_absolute(bus) {
                    let operand = bus.read(addr);
                    self.register_a = self.sbc_binary(operand);
                }
            }
            _ => panic!("Opcode {:#x} not implemented", opcode),
        }
    }
}
