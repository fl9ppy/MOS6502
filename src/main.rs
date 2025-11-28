mod bus;
mod cpu;

use crate::bus::{Bus, SimpleBus};
use crate::cpu::CPU;

fn main() {
    // ---------------------------------------------------------
    // ROM PROGRAM (placed at $8000)
    //
    // 8000: A9 42    LDA #$42
    // 8002: 8D 00 60 STA $6000
    // 8005: 4C 05 80 JMP $8005
    // ---------------------------------------------------------
    let rom_program: Vec<u8> = vec![
        0xA9, 0x42,       // LDA #$42
        0x8D, 0x00, 0x60, // STA $6000
        0x4C, 0x05, 0x80, // JMP $8005 (infinite loop)
    ];

    // ---------------------------------------------------------
    // Create the system bus
    // RAM = 32KB (0x0000 — 0x7FFF)
    // ROM = maps to   (0x8000 — 0xFFFF)
    // ---------------------------------------------------------
    let mut bus = SimpleBus::new(32 * 1024, &rom_program);

    // ---------------------------------------------------------
    // Setup RESET VECTOR at $FFFC–$FFFD
    //
    // The 6502 reads $FFFC-$FFFD on reset to get the startup PC.
    // We want PC = $8000
    // ---------------------------------------------------------
    bus.ram.write(0xFFFC, 0x00);
    bus.ram.write(0xFFFD, 0x80);

    // ---------------------------------------------------------
    // Initialize CPU
    // ---------------------------------------------------------
    let mut cpu = CPU::new();
    cpu.reset(&mut bus);

    println!("--- CPU Reset Complete ---");
    println!(
        "PC={:04X}, A={:02X}, X={:02X}, Y={:02X}",
        cpu.program_counter, cpu.register_a, cpu.register_x, cpu.register_y
    );
    println!("---------------------------");

    // ---------------------------------------------------------
    // Run the CPU for 20 instructions
    // and print debug output
    // ---------------------------------------------------------
    for step in 0..20 {
        let pc_before = cpu.program_counter;

        // Read the opcode before executing
        let opcode = bus.read(pc_before);

        println!(
            "Step {:02}: PC={:04X}, Opcode={:02X}",
            step, pc_before, opcode
        );

        cpu.run_once(&mut bus);

        println!(
            "       -> A={:02X}, X={:02X}, Y={:02X}, PC={:04X}, RAM[$6000]={:02X}",
            cpu.register_a,
            cpu.register_x,
            cpu.register_y,
            cpu.program_counter,
            bus.ram.read(0x6000)
        );
        println!();
    }
}
