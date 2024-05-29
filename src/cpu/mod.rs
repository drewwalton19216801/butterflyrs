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

/// Represents the 6502 CPU core.
pub struct Cpu {
    /// A reference-counted, mutable, smart pointer to a `MainBus` object.
    pub bus: Rc<RefCell<MainBus>>,

    /// The accumulator register.
    pub a: Register8,

    /// The X register.
    pub x: Register8,

    /// The Y register.
    pub y: Register8,

    /// The processor status flags register.
    pub p: Register8,

    /// The stack pointer register.
    pub sp: Register8,

    /// The program counter register.
    pub pc: Register16,
}

bitflags! {
    pub struct StatusFlags: u8 {
        /// No flags set.
        const None = 0b0000_0000;

        /// Carry flag. Set if a carry occurred during arithmetic operations.
        const Carry = 0b0000_0001;

        /// Zero flag. Set if the result of an arithmetic operation is zero.
        const Zero = 0b0000_0010;

        /// Interrupt disable flag. Set to disable interrupts.
        const InterruptDisable = 0b0000_0100;

        /// Decimal mode flag. Set to enable decimal arithmetic operations.
        const DecimalMode = 0b0000_1000;

        /// Break flag. Set to indicate a breakpoint.
        const Break = 0b0001_0000;

        /// Unused flag. Unused by the CPU, can be used by the programmer.
        const Unused = 0b0010_0000;

        /// Overflow flag. Set if the result of an arithmetic operation overflows.
        const Overflow = 0b0100_0000;

        /// Negative flag. Set if the result of an arithmetic operation is negative.
        const Negative = 0b1000_0000;
    }
}

impl Cpu {
    /// Creates a new instance of the `Cpu` struct.
    ///
    /// # Arguments
    ///
    /// * `bus` - A reference-counted, mutable, smart pointer to a `MainBus` object.
    ///
    /// # Returns
    ///
    /// A new instance of the `Cpu` struct.
    pub fn new(bus: Rc<RefCell<MainBus>>) -> Cpu {
        // Create a new instance of the `Cpu` struct.
        Cpu {
            // Assign the `bus` argument to the `bus` field of the `Cpu` struct.
            bus,
            // Create a new instance of the `Register8` struct and assign it to the `a` field of the `Cpu` struct.
            a: Register8::new(),
            // Create a new instance of the `Register8` struct and assign it to the `x` field of the `Cpu` struct.
            x: Register8::new(),
            // Create a new instance of the `Register8` struct and assign it to the `y` field of the `Cpu` struct.
            y: Register8::new(),
            // Create a new instance of the `Register8` struct and assign it to the `p` field of the `Cpu` struct.
            p: Register8::new(),
            // Create a new instance of the `Register8` struct and assign it to the `sp` field of the `Cpu` struct.
            sp: Register8::new(),
            // Create a new instance of the `Register16` struct and assign it to the `pc` field of the `Cpu` struct.
            pc: Register16::new(),
        }
    }

    /// Connects the CPU to the main bus.
    ///
    /// # Arguments
    ///
    /// * `bus` - The main bus that the CPU will connect to.
    ///
    /// # Example
    ///
    /// ```
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    /// use crate::bus::MainBus;
    /// use crate::cpu::Cpu;
    ///
    /// let mut cpu = Cpu::new(Rc::new(RefCell::new(MainBus::new())));
    /// let bus = Rc::new(RefCell::new(MainBus::new()));
    /// cpu.connect_bus(bus);
    /// ```
    pub fn connect_bus(&mut self, bus: Rc<RefCell<MainBus>>) {
        // Connects the CPU to the main bus.
        self.bus = bus;
    }

    /// Resets the CPU state to its initial values.
    ///
    /// This method sets the values of the CPU registers to their initial values
    /// and sets the program counter to the reset vector address.
    pub fn reset(&mut self) {
        // Set the accumulator register to 0x00
        self.a.set(0x00);

        // Set the X register to 0x00
        self.x.set(0x00);

        // Set the Y register to 0x00
        self.y.set(0x00);

        // Set the processor status flags to their initial values
        // The initial values are: None, Unused, and InterruptDisable
        self.p.set(StatusFlags::None.bits() | StatusFlags::Unused.bits() | StatusFlags::InterruptDisable.bits());

        // Set the stack pointer register to 0xFD
        self.sp.set(0xFD);

        // Set the program counter to the reset vector address
        self.pc.set(self.read16(RESET_VECTOR));
    }

    /// Reads a single byte from the specified address on the bus.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to read from.
    ///
    /// # Returns
    ///
    /// The byte read from the bus.
    fn read8(&self, address: u16) -> u8 {
        // Borrow the bus to read from it.
        // The borrow is released when the function returns.
        self.bus.borrow().read(address)
    }

    /// Writes a single byte to the specified address on the bus.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to write to.
    /// * `value` - The byte value to write.
    fn write8(&mut self, address: u16, value: u8) {
        // Borrow the bus as mutable to write to it.
        // The borrow is released when the function returns.
        self.bus.borrow_mut().write(address, value)
    }

    /// Reads a 16-bit value from the specified address on the bus.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to read from.
    ///
    /// # Returns
    ///
    /// The 16-bit value read from the bus.
    fn read16(&self, address: u16) -> u16 {
        // Read the low byte from the bus
        let low = self.read8(address) as u16;

        // Read the high byte from the bus, offset by 1
        let high = self.read8(address + 1) as u16;

        // Combine the low and high bytes into a 16-bit value
        // by shifting the high byte 8 bits to the left and ORing it with the low byte
        (high << 8) | low
    }

    /// Writes a 16-bit value to the specified address on the bus.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to write to.
    /// * `value` - The 16-bit value to write.
    pub fn write16(&mut self, address: u16, value: u16) {
        // Write the low byte to the bus
        self.write8(address, (value & 0xFF) as u8);

        // Write the high byte to the bus, offset by 1
        self.write8(address + 1, ((value >> 8) & 0xFF) as u8);
    }
}

impl Display for Cpu {
    /// Formats the CPU state for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to format the CPU state into.
    ///
    /// # Returns
    ///
    /// The formatted CPU state.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the CPU state into the formatter
        write!(
            f,
            "a: {:02X}, x: {:02X}, y: {:02X}, p: {:02X}, sp: {:02X}, pc: {:04X}",
            self.a.get(),
            self.x.get(),
            self.y.get(),
            self.p.get(),
            self.sp.get(),
            self.pc.get()
        )
    }
}