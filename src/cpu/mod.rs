#![warn(missing_docs)]
//! A 6502 core written in Rust
//!
//! Aims to provide a simple, easy-to-use interface for emulating the 6502 CPU.
//! The CPU connects to a bus, and the bus can contain any number of memory
//! regions, each of which can be accessed by the CPU.

mod addresses;
mod addressing;
mod instructions;

use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;
use bitflags::bitflags;

use crate::bus::MainBus;
use crate::cpu::addresses::RESET_VECTOR;
use crate::cpu::addressing::AddressingMode;
use crate::cpu::instructions::INSTRUCTION_LIST;
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

    /// The number of CPU cycles remaining in the current instruction.
    pub cycles: u8,

    /// The absolute address as calculated by the instruction's address mode.
    address_absolute: u16,

    /// The relative address as calculated by the instruction's address mode. Used for branching.
    address_relative: u16,

    /// The current addressing mode.
    address_mode: AddressingMode,

    /// The current opcode.
    opcode: u8,

    /// The current fetched data.
    fetched_data: u8,

    /// Whether illegal opcodes should be enabled.
    pub enable_illegal_opcodes: bool,

    /// The current instruction string.
    pub current_instruction_string: String,

    /// Debug modes
    /// 0: No debug
    /// 1: Print CPU state after each instruction
    /// 2: Print CPU state after each cycle
    pub debug: usize,
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
            // Set the `cycles` field of the `Cpu` struct to 0.
            cycles: 0,
            // Set the `address_absolute` field of the `Cpu` struct to 0.
            address_absolute: 0,
            // Set the `address_relative` field of the `Cpu` struct to 0.
            address_relative: 0,
            // Set the `address_mode` field of the `Cpu` struct to `AddressingMode::None`.
            address_mode: AddressingMode::None,
            // Set the `opcode` field of the `Cpu` struct to 0.
            opcode: 0,
            // Set the `fetched_data` field of the `Cpu` struct to 0.
            fetched_data: 0,
            // Set the `enable_illegal_opcodes` field of the `Cpu` struct to false.
            enable_illegal_opcodes: false,
            current_instruction_string: String::new(),
            debug: 0,
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

    /// Sets or removes a flag in the processor status register (`p`).
    ///
    /// # Arguments
    ///
    /// * `flag` - The flag to set or remove.
    /// * `value` - If `true`, the flag is set. If `false`, the flag is removed.
    fn set_flag(&mut self, flag: StatusFlags, value: bool) {
        // If the value is true, set the flag in the processor status register.
        if value {
            self.p.set(flag.bits());
        }
        // If the value is false, remove the flag from the processor status register.
        else {
            self.p.remove(flag.bits());
        }
    }

    /// Returns the value of a specific flag in the processor status register.
    ///
    /// # Arguments
    ///
    /// * `flag` - The flag to retrieve the value of.
    ///
    /// # Returns
    ///
    /// `true` if the flag is set, `false` otherwise.
    fn get_flag(&self, flag: StatusFlags) -> bool {
        // Check if the flag is present in the processor status register.
        self.p.contains(flag.bits())
    }

    /// Increments the stack pointer (`sp`) by 1.
    /// If the stack pointer reaches 0x00, it wraps around to 0xFF.
    fn increment_sp(&mut self) {
        // Increment the stack pointer by 1
        self.sp.add_assign(1);

        // Check if the stack pointer is 0x00
        if self.sp.get() == 0x00 {
            // If it is, wrap around to 0xFF
            self.sp.set(0xFF);
        }
    }

    /// Decrements the stack pointer (`sp`) by 1.
    /// If the stack pointer reaches 0x00, it wraps around to 0xFF.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::cpu::Cpu;
    ///
    /// let mut cpu = Cpu::new();
    /// cpu.sp.set(0x01);
    /// cpu.decrement_sp();
    /// assert_eq!(cpu.sp.get(), 0xFF);
    /// ```
    fn decrement_sp(&mut self) {
        // Decrement the stack pointer by 1
        self.sp.sub_assign(1);

        // If the stack pointer is 0xFF, wrap around to 0x00
        if self.sp.get() == 0xFF {
            self.sp.set(0x00);
        }
    }

    pub fn set_illegal_opcodes(&mut self, value: bool) {
        self.enable_illegal_opcodes = value;
    }

    /// Get the status string for the CPU (NV-BDIZC)
    pub fn get_status_string(&self) -> String {
        let mut status = String::new();
        status.push_str("STATUS: ");
        status.push_str(if self.get_flag(StatusFlags::Negative) {
            "N"
        } else {
            "n"
        });
        status.push_str(if self.get_flag(StatusFlags::Overflow) {
            "V"
        } else {
            "v"
        });
        status.push('-');
        status.push_str(if self.get_flag(StatusFlags::Break) {
            "B"
        } else {
            "b"
        });
        status.push_str(if self.get_flag(StatusFlags::DecimalMode) {
            "D"
        } else {
            "d"
        });
        status.push_str(if self.get_flag(StatusFlags::InterruptDisable) {
            "I"
        } else {
            "i"
        });
        status.push_str(if self.get_flag(StatusFlags::Zero) {
            "Z"
        } else {
            "z"
        });
        status.push_str(if self.get_flag(StatusFlags::Carry) {
            "C"
        } else {
            "c"
        });
        status
    }

    /// Fetches the next byte from memory.
    ///
    /// # Returns
    ///
    /// The fetched byte.
    fn fetch(&mut self) -> u8 {
        if self.address_mode != AddressingMode::Implied {
            self.fetched_data = self.read8(self.address_absolute);
        }
        self.fetched_data
    }

    /// Pushes a byte onto the stack.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte to be pushed onto the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cpu = Cpu::new();
    /// cpu.push(0x42);
    /// assert_eq!(cpu.read8(0x100 + cpu.sp.get() as u16), 0x42);
    /// assert_eq!(cpu.sp.get(), 0xFD);
    /// ```
    fn push(&mut self, value: u8) {
        // Write the value to the stack pointer address
        self.write8(0x100 + self.sp.get() as u16, value);

        // Decrement the stack pointer
        self.decrement_sp();
    }

    /// Pushes a 16-bit word onto the stack.
    ///
    /// # Arguments
    ///
    /// * `value` - The 16-bit word to be pushed onto the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cpu = Cpu::new();
    /// cpu.push_word(0x1234);
    /// assert_eq!(cpu.read8(0x100 + cpu.sp.get() as u16), 0x34);
    /// assert_eq!(cpu.read8(0x100 + (cpu.sp.get() - 1) as u16), 0x12);
    /// assert_eq!(cpu.sp.get(), 0xFC);
    /// ```
    fn push_word(&mut self, value: u16) {
        // Extract the high byte of the word and push it onto the stack
        self.push(((value >> 8) & 0xff) as u8);

        // Extract the low byte of the word and push it onto the stack
        self.push((value & 0xff) as u8);
    }

    /// Pops a byte from the stack.
    ///
    /// # Returns
    ///
    /// The byte popped from the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cpu = Cpu::new();
    /// cpu.push(0x42);
    /// assert_eq!(cpu.pop(), 0x42);
    /// assert_eq!(cpu.sp.get(), 0xFF);
    /// ```
    fn pop(&mut self) -> u8 {
        // Increment the stack pointer
        self.increment_sp();

        // Read the byte from the stack pointer address
        self.read8(0x100 + self.sp.get() as u16)
    }

    /// Pops a word (2 bytes) from the stack.
    ///
    /// # Returns
    ///
    /// The word popped from the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cpu = Cpu::new();
    /// cpu.push_word(0x1234);
    /// assert_eq!(cpu.pop_word(), 0x1234);
    /// assert_eq!(cpu.sp.get(), 0xFF);
    /// ```
    fn pop_word(&mut self) -> u16 {
        // Pop the low byte from the stack
        let lo = self.pop() as u16;

        // Pop the high byte from the stack
        let hi = self.pop() as u16;

        // Combine the high and low bytes to form the word
        (hi << 8) | lo
    }

    /// Executes an instruction.
    ///
    /// # Arguments
    ///
    /// * `opcode` - The opcode of the instruction to execute.
    ///
    /// # Returns
    ///
    /// The number of cycles the instruction took to execute.
    fn execute_instruction(&mut self, opcode: u8) -> u8 {
        let instruction = &INSTRUCTION_LIST[opcode as usize];
        (instruction.function)(self)
    }

    /// Returns a string representation of the operand based on the addressing mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The addressing mode of the operand.
    /// * `address` - The address of the operand.
    ///
    /// # Returns
    ///
    /// A string representation of the operand.
    fn get_operand_string(&mut self, mode: AddressingMode, address: u16) -> String {
        match mode {
            // No operand
            AddressingMode::None => String::from(""),
            // Implied operand
            AddressingMode::Implied => String::from(""),
            // Immediate operand
            AddressingMode::Immediate => format!("#${:02X}", self.read8(address)),
            // Zero page operand
            AddressingMode::ZeroPage => format!("${:02X}", self.read8(address)),
            // Zero page with X offset operand
            AddressingMode::ZeroPageX => format!("${:02X},X", self.read8(address)),
            // Zero page with Y offset operand
            AddressingMode::ZeroPageY => format!("${:02X},Y", self.read8(address)),
            // Relative operand
            AddressingMode::Relative => format!("${:02X}", self.read8(address)),
            // Absolute operand
            AddressingMode::Absolute => format!("${:04X}", self.read16(address)),
            // Absolute with X offset operand
            AddressingMode::AbsoluteX => format!("${:04X},X", self.read16(address)),
            // Absolute with Y offset operand
            AddressingMode::AbsoluteY => format!("${:04X},Y", self.read16(address)),
            // Indirect operand
            AddressingMode::Indirect => format!("(${:04X})", self.read16(address)),
            // Indexed indirect operand
            AddressingMode::IndexedIndirect => format!("(${:02X},X)", self.read8(address)),
            // Indirect indexed operand
            AddressingMode::IndirectIndexed => format!("(${:02X}),Y", self.read8(address)),
        }
    }

    /// Disassembles the instruction at the specified address.
    ///
    /// # Arguments
    ///
    /// * `from_pc` - The address of the instruction to disassemble.
    ///
    /// # Returns
    ///
    /// The disassembled instruction.
    fn disassemble_instruction_at(&mut self, from_pc: u16) -> String {
        let opcode = self.read8(from_pc);
        let instruction = &INSTRUCTION_LIST[opcode as usize];
        let addr_mode = instructions::get_addr_mode(opcode);
        let addr_str = self.get_operand_string(addr_mode, from_pc + 1);
        format!("{} {}", instruction.name, addr_str)
    }

    /// Executes the given addressing mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The addressing mode to execute.
    ///
    /// # Returns
    ///
    /// Returns `1` if an extra cycle is needed, otherwise returns `0`.
    fn execute_addr_mode(&mut self, mode: AddressingMode) -> u8 {
        // Set the current addressing mode
        self.address_mode = mode;

        // Execute the addressing mode and get the extra cycle flag
        let extra_cycle = mode.execute(self);

        // If an extra cycle is needed, return 1, otherwise return 0
        if extra_cycle {
            return 1;
        }
        0
    }

    /// Returns the number of cycles required to execute the instruction with the given opcode.
    ///
    /// # Arguments
    ///
    /// * `opcode` - The opcode of the instruction.
    ///
    /// # Returns
    ///
    /// The number of cycles required to execute the instruction.
    pub fn get_cycles(&self, opcode: u8) -> u8 {
        // Get the number of cycles required to execute the instruction from the instructions module.
        instructions::get_cycles(opcode)
    }

    /// Performs an interrupt by pushing the program counter and status flags to the stack,
    /// setting the necessary flags, and loading the interrupt vector into the program counter.
    ///
    /// # Arguments
    ///
    /// * `vector` - The address of the interrupt vector.
    ///
    /// # Returns
    ///
    /// None
    fn do_interrupt(&mut self, vector: u16) {
        // Push the program counter to the stack
        self.push_word(self.pc.get());

        // Clear the Break flag
        self.set_flag(StatusFlags::Break, false);

        // Set the Unused flag
        self.set_flag(StatusFlags::Unused, true);

        // Set the Break flag again
        self.set_flag(StatusFlags::Break, true);

        // Set the Interrupt Disable flag
        self.set_flag(StatusFlags::InterruptDisable, true);

        // Push the status flags to the stack
        self.push(self.p.get());

        // Clear the Interrupt Disable flag
        self.set_flag(StatusFlags::InterruptDisable, false);

        // Load the interrupt vector into the program counter
        self.pc = Register16 { value: self.read16(vector) };

        // Set the number of cycles required to execute the interrupt
        self.cycles = 7;
    }

    /// Handles the IRQ (Interrupt Request) interrupt.
    ///
    /// If the Interrupt Disable flag is not set, the function calls the `do_interrupt` method with the IRQ vector address.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The mutable reference to the `Cpu` struct.
    #[allow(dead_code)]
    pub fn irq(&mut self) {
        // Check if the Interrupt Disable flag is not set
        if !self.get_flag(StatusFlags::InterruptDisable) {
            // Call the `do_interrupt` method with the IRQ vector address
            self.do_interrupt(addresses::IRQ_VECTOR);
        }
    }

    /// Handles the Non-Maskable Interrupt (NMI) interrupt.
    ///
    /// This function calls the `do_interrupt` method with the NMI vector address.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The mutable reference to the `Cpu` struct.
    #[allow(dead_code)]
    pub fn nmi(&mut self) {
        // Call the `do_interrupt` method with the NMI vector address
        self.do_interrupt(addresses::NMI_VECTOR);
    }

    /// Returns the value of a specific register.
    ///
    /// # Arguments
    ///
    /// * `register` - The register to retrieve the value of.
    ///
    /// # Returns
    ///
    /// The value of the register.
    pub fn get_register(&self, register: &str) -> u8 {
        match register {
            "A" => self.a.get(),
            "X" => self.x.get(),
            "Y" => self.y.get(),
            "SP" => self.sp.get(),
            _ => panic!("Invalid register: {}", register),
        }
    }

    /// Sets the zero and negative flags based on the value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set the flags for.
    fn set_zn_flags(&mut self, value: u8) {
        // Set the zero flag if the value is zero
        self.set_flag(StatusFlags::Zero, value == 0);

        // Set the negative flag if the most significant bit of the value is set
        self.set_flag(StatusFlags::Negative, value & 0x80 != 0);
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.current_instruction_string = self.disassemble_instruction_at(self.pc.get());
            match self.debug {
                0 => (),
                1 => println!("{}", self.current_instruction_string),
                2 => {
                    println!("{}", self.current_instruction_string);
                    println!("CPU pre-execute state: {}", self);
                }
                _ => panic!("Invalid debug value: {}", self.debug),
            }
            self.opcode = self.read8(self.pc.get());
            self.pc.add_assign(1);
            self.cycles = self.get_cycles(self.opcode);
            self.address_mode = instructions::get_addr_mode(self.opcode);
            let cycles_address_mode = self.execute_addr_mode(self.address_mode);
            let cycles_instruction = self.execute_instruction(self.opcode);
            self.cycles += cycles_address_mode + cycles_instruction;
            if self.debug > 1 {
                println!("CPU post-execute state: {}", self);
            }
        }
        self.cycles -= 1;
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