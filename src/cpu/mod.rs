#![warn(missing_docs)]
//! A 6502 core written in Rust
//!
//! Aims to provide a simple, easy-to-use interface for emulating the 6502 CPU.
//! The CPU connects to a bus, and the bus can contain any number of memory
//! regions, each of which can be accessed by the CPU.

mod addresses;

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use bitflags::bitflags;

use crate::bus::MainBus;
use crate::cpu::addresses::RESET_VECTOR;
use crate::register::{Register8, Register16};

pub struct Cpu {
    bus: Rc<RefCell<MainBus>>,

    a: Register8,
    x: Register8,
    y: Register8,
    p: Register8,
    sp: Register8,
    pc: Register16,
}

bitflags! {
    pub struct StatusFlags: u8 {
        const None = 0b0000_0000;
        const Carry = 0b0000_0001;
        const Zero = 0b0000_0010;
        const InterruptDisable = 0b0000_0100;
        const DecimalMode = 0b0000_1000;
        const Break = 0b0001_0000;
        const Unused = 0b0010_0000;
        const Overflow = 0b0100_0000;
        const Negative = 0b1000_0000;
    }
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<MainBus>>) -> Cpu {
        Cpu {
            bus,
            a: Register8::new(),
            x: Register8::new(),
            y: Register8::new(),
            p: Register8::new(),
            sp: Register8::new(),
            pc: Register16::new(),
        }
    }

    pub fn connect_bus(&mut self, bus: Rc<RefCell<MainBus>>) {
        self.bus = bus;
    }

    pub fn reset(&mut self) {
        self.a.set(0x00);
        self.x.set(0x00);
        self.y.set(0x00);
        self.p.set(StatusFlags::None.bits() | StatusFlags::Unused.bits() | StatusFlags::InterruptDisable.bits());
        self.sp.set(0xFD);
        self.pc.set(self.read16(RESET_VECTOR));
    }

    pub fn read8(&self, address: u16) -> u8 {
        self.bus.borrow().read(address)
    }

    pub fn write8(&mut self, address: u16, value: u8) {
        self.bus.borrow_mut().write(address, value)
    }

    pub fn read16(&self, address: u16) -> u16 {
        let low = self.read8(address) as u16;
        let high = self.read8(address + 1) as u16;
        (high << 8) | low
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write8(address, (value & 0xFF) as u8);
        self.write8(address + 1, ((value >> 8) & 0xFF) as u8);
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a: {:02X}, x: {:02X}, y: {:02X}, p: {:02X}, sp: {:02X}, pc: {:04X}", self.a.get(), self.x.get(), self.y.get(), self.p.get(), self.sp.get(), self.pc.get())
    }
}