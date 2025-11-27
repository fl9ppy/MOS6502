mod cpu;
mod bus;

use crate::bus::{Bus, Ram};
use crate::cpu::CPU;

fn main() {
    let mut ram = Ram::new();
    let mut cpu = CPU::new();

    // Set reset vector to start program at 0x0600
    ram.write(0xFFFC, 0x00);
    ram.write(0xFFFD, 0x06);

    // IRQ/BRK vector (just point back to 0x0600 for test simplicity)
    ram.write(0xFFFE, 0x00);
    ram.write(0xFFFF, 0x06);

    // Program: tests LDA, STA, TAX, INX, ADC, SBC, JSR, RTS, PHA, PLA, PHP, PLP, branches, BRK
    let test_program: [u8; 31] = [
        // Init A and X
        0xA9, 0x05,       // LDA #$05       ; A = 5
        0xAA,             // TAX            ; X = A
        0xE8,             // INX            ; X = X+1 -> 6

        // Store A into memory
        0x8D, 0x00, 0x07, // STA $0700      ; Mem[0x0700] = 5

        // Arithmetic
        0x69, 0x03,       // ADC #$03       ; A = 5+3=8
        0xE9, 0x04,       // SBC #$04       ; A = 8-4=4

        // Stack operations
        0x48,             // PHA            ; push A
        0xA9, 0x00,       // LDA #$00
        0x68,             // PLA            ; pull A = 4

        0x08,             // PHP            ; push status
        0x28,             // PLP            ; pull status

        // Subroutine
        0x20, 0x20, 0x06, // JSR $0620      ; call subroutine at 0x0620
        0x00,             // BRK            ; stop

        // Padding to subroutine location
        0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0xEA, // NOPs (0xEA)

        // Subroutine at 0x0620
        0xE8,             // INX            ; X++
        0x60,             // RTS            ; return
    ];

    ram.load(0x0600, &test_program);

    println!("Initial CPU State:");
    println!("A:  {:#04X}", cpu.register_a);
    println!("X:  {:#04X}", cpu.register_x);
    println!("Status: {:#04X}", cpu.status);
    println!("Memory[0x0700]: {:#04X}", ram.read(0x0700));

    cpu.reset(&mut ram);

    // Run program until BRK
    loop {
        let pc = cpu.program_counter;
        let opcode = ram.read(pc);
        if opcode == 0x00 { break; }
        cpu.run_once(&mut ram);
    }

    println!("\nFinal CPU State:");
    println!("A:  {:#04X}", cpu.register_a);
    println!("X:  {:#04X}", cpu.register_x);
    println!("Status: {:#04X}", cpu.status);
    println!("Memory[0x0700]: {:#04X}", ram.read(0x0700));
}
