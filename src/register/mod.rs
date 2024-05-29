use std::ops::AddAssign;

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

    pub fn remove(&mut self, value: u8) {
        self.value &= !value;
    }

    pub fn contains(&self, value: u8) -> bool {
        self.value & value != 0
    }

    pub fn sub_assign(&mut self, value: u8) {
        self.value -= value;
    }
}

impl AddAssign<u8> for Register8 {
    fn add_assign(&mut self, rhs: u8) {
        self.value += rhs;
    }
}

/// Represents a 16-bit register.
pub struct Register16 {
    pub value: u16,
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

impl AddAssign<u16> for Register16 {
    fn add_assign(&mut self, rhs: u16) {
        self.value += rhs;
    }
}