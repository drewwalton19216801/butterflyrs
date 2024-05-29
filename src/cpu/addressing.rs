use std::fmt::Display;
use crate::cpu::Cpu;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AddressingMode {
    None,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

impl Display for AddressingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressingMode::None => write!(f, "None"),
            AddressingMode::Absolute => write!(f, "Absolute"),
            AddressingMode::AbsoluteX => write!(f, "AbsoluteX"),
            AddressingMode::AbsoluteY => write!(f, "AbsoluteY"),
            AddressingMode::Immediate => write!(f, "Immediate"),
            AddressingMode::Implied => write!(f, "Implied"),
            AddressingMode::Indirect => write!(f, "Indirect"),
            AddressingMode::IndexedIndirect => write!(f, "IndexedIndirect"),
            AddressingMode::IndirectIndexed => write!(f, "IndirectIndexed"),
            AddressingMode::Relative => write!(f, "Relative"),
            AddressingMode::ZeroPage => write!(f, "ZeroPage"),
            AddressingMode::ZeroPageX => write!(f, "ZeroPageX"),
            AddressingMode::ZeroPageY => write!(f, "ZeroPageY"),
        }
    }
}

impl AddressingMode {
    /// Execute an addressing mode, returns true if an extra cycle is needed
    pub fn execute(&self, cpu: &mut Cpu) -> bool {
        match self {
            AddressingMode::None => false,
            AddressingMode::Absolute => {
                let address = cpu.read16(cpu.pc.get());
                cpu.address_absolute = address;
                cpu.pc += 2;
                false
            }
            AddressingMode::AbsoluteX => {
                let address = cpu.read16(cpu.pc.get());
                cpu.address_absolute = address + cpu.x.get() as u16;
                cpu.pc += 2;

                // If page boundary is crossed, we need an extra cycle
                if (cpu.address_absolute & 0xFF00) != (address & 0xFF00) {
                    return true;
                }
                false
            }
            AddressingMode::AbsoluteY => {
                let address = cpu.read16(cpu.pc.get());
                cpu.address_absolute = address + cpu.y.get() as u16;
                cpu.pc += 2;

                // If page boundary is crossed, we need an extra cycle
                if (cpu.address_absolute & 0xFF00) != (address & 0xFF00) {
                    return true;
                }
                false
            }
            AddressingMode::Immediate => {
                cpu.address_absolute = cpu.pc.get();
                cpu.pc += 1;
                false
            }
            AddressingMode::Implied => {
                cpu.fetched_data = cpu.a.get();
                false
            }
            AddressingMode::Indirect => {
                let addr_lo = cpu.read8(cpu.pc.get());
                let addr_hi = cpu.read8(cpu.pc.get() + 1);
                let addr = (addr_hi as u16) << 8 | (addr_lo as u16);

                if addr_lo == 0x00FF {
                    // We crossed a page boundary, so we need to simulate the hardware bug
                    cpu.address_absolute = (cpu.read8(addr & 0xFF00) as u16) << 8 | cpu.read8(addr) as u16;
                } else {
                    cpu.address_absolute = (cpu.read8(addr + 1) as u16) << 8 | cpu.read8(addr) as u16;
                }
                cpu.pc += 2;
                false
            }
            AddressingMode::IndexedIndirect => {
                let temp = cpu.read8(cpu.pc.get());
                let lo = cpu.read8(temp as u16 + cpu.x.get() as u16);
                let hi = cpu.read8(temp as u16 + cpu.x.get() as u16 + 1);
                cpu.address_absolute = (hi as u16) << 8 | (lo as u16);
                cpu.pc += 1;
                false
            }
            AddressingMode::IndirectIndexed => {
                let temp = cpu.read8(cpu.pc.get());
                let lo = cpu.read8(temp as u16);
                let hi = cpu.read8(temp as u16 + 1);
                cpu.address_absolute = (hi as u16) << 8 | (lo as u16);
                cpu.pc += 1;

                // If page boundary is crossed, we need an extra cycle
                if (cpu.address_absolute & 0xFF00) != ((hi as u16) << 8) {
                    return true;
                }
                false
            }
            AddressingMode::Relative => {
                cpu.address_relative = cpu.read8(cpu.pc.get()) as u16;
                cpu.pc += 1;
                if cpu.address_relative & 0x80 > 0 {
                    cpu.address_relative |= 0xFF00;
                }
                false
            }
            AddressingMode::ZeroPage => {
                cpu.address_absolute = (cpu.read8(cpu.pc.get()) as u16) & 0x00FF;
                cpu.pc += 1;
                false
            }
            AddressingMode::ZeroPageX => {
                cpu.address_absolute = (cpu.read8(cpu.pc.get()) as u16 + cpu.x.get() as u16) & 0x00FF;
                cpu.pc += 1;
                false
            }
            AddressingMode::ZeroPageY => {
                cpu.address_absolute = (cpu.read8(cpu.pc.get()) as u16 + cpu.y.get() as u16) & 0x00FF;
                cpu.pc += 1;
                false
            }
        }
    }
}