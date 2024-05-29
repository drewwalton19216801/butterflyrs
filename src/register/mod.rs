pub struct Register8 {
    value: u8,
}

impl Register8 {
    pub fn new() -> Register8 {
        Register8 { value: 0 }
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }
}

pub struct Register16 {
    value: u16,
}

impl Register16 {
    pub fn new() -> Register16 {
        Register16 { value: 0 }
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }
}