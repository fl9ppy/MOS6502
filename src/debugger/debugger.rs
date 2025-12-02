use crate::cpu::CPU;
use crate::bus::Bus;
use crate::disassembler::disassemble;

/// Represents one snapshot of CPU state
pub struct DebugState {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub status: u8,
    pub disasm: String,
}

/// The main debugger wrapper
pub struct Debugger<'a, B: Bus> {
    pub cpu: &'a mut CPU,
    pub bus: &'a mut B,
    breakpoints: Vec<u16>,
    tracing: bool,
    trace_log: Vec<DebugState>,
}

impl<'a, B: Bus> Debugger<'a, B> {
    /// Create a new debugger around CPU and Bus
    pub fn new(cpu: &'a mut CPU, bus: &'a mut B) -> Self {
        Self {
            cpu,
            bus,
            breakpoints: Vec::new(),
            tracing: false,
            trace_log: Vec::new(),
        }
    }

    /// Add a breakpoint
    pub fn add_breakpoint(&mut self, addr: u16) {
        if !self.breakpoints.contains(&addr) {
            self.breakpoints.push(addr);
        }
    }

    /// Step one instruction
    pub fn step(&mut self) -> DebugState {
        let (line, next_pc) = disassemble(self.bus, self.cpu.program_counter);

        // Capture CPU state BEFORE execution
        let state = DebugState {
            pc: self.cpu.program_counter,
            a: self.cpu.register_a,
            x: self.cpu.register_x,
            y: self.cpu.register_y,
            sp: self.cpu.stack_pointer,
            status: self.cpu.status,
            disasm: line.pretty(),
        };

        self.cpu.run_once(self.bus);

        // Store trace if enabled
        if self.tracing {
            self.trace_log.push(state.clone());
        }

        state
    }

    /// Run until a breakpoint is reached
    pub fn run_until_break(&mut self) -> DebugState {
        loop {
            let state = self.step();

            if self.breakpoints.contains(&state.pc) {
                return state;
            }
        }
    }

    /// Trace N instructions
    pub fn trace_next(&mut self, count: usize) -> Vec<DebugState> {
        self.trace_log.clear();
        self.tracing = true;

        for _ in 0..count {
            self.step();
        }

        self.tracing = false;
        self.trace_log.clone()
    }

    /// Print registers
    pub fn dump_registers(&self) {
        println!(
            "PC={:04X}  A={:02X}  X={:02X}  Y={:02X}  SP={:02X}  STATUS={:08b}",
            self.cpu.program_counter,
            self.cpu.register_a,
            self.cpu.register_x,
            self.cpu.register_y,
            self.cpu.stack_pointer,
            self.cpu.status,
        );
    }

    /// Dump RAM region
    pub fn dump_ram(&self, addr: u16, len: usize) {
        for i in 0..len {
            if i % 16 == 0 {
                print!("\n{:04X}: ", addr + i as u16);
            }
            print!("{:02X} ", self.bus.read(addr + i as u16));
        }
        println!();
    }
}
