pub mod ram;
pub mod rom;

use crate::bus::ram::Ram;
use crate::bus::rom::Rom;

/// The Bus trait defines how the CPU interacts with memory or devices.
pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

/// Composite Bus supporting multiple devices
pub struct SimpleBus {
    pub ram: Ram,
    pub rom: Rom,
}

impl SimpleBus {
    pub fn new(ram_size: usize, rom_data: &[u8]) -> Self {
        Self {
            ram: Ram::new(ram_size),
            rom: Rom::new(rom_data),
        }
    }
}

impl Bus for SimpleBus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.ram.read(addr),
            0x8000..=0xFFFF => self.rom.read(addr),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => self.ram.write(addr, data),
            0x8000..=0xFFFF => {
                // Writes to ROM are ignored
                eprintln!("Warning: write to ROM at {:#06X} ignored", addr);
            }
        }
    }
}
