/// Represents an 8-bit register.
pub struct Register8 {
    /// The value stored in the register.
    value: u8,
}

impl Register8 {
    /// Creates a new instance of the `Register8` struct with an initial value of 0.
    pub fn new() -> Register8 {
        Register8 { value: 0 }
    }

    /// Returns the value stored in the register.
    pub fn get(&self) -> u8 {
        self.value
    }

    /// Sets the value stored in the register.
    pub fn set(&mut self, value: u8) {
        self.value = value;
    }
}

/// Represents a 16-bit register.
pub struct Register16 {
    value: u16,
}

impl Register16 {
    /// Creates a new instance of the `Register16` struct with an initial value of 0.
    pub fn new() -> Register16 {
        Register16 { value: 0 }
    }

    /// Returns the value stored in the register.
    pub fn get(&self) -> u16 {
        self.value
    }

    /// Sets the value stored in the register.
    pub fn set(&mut self, value: u16) {
        self.value = value;
    }
}