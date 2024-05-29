use crate::bus::BusDevice;

pub struct Blink8 {
    pub enabled: bool,
    pub start: u16,
    pub end: u16,
}

impl Blink8 {
    pub fn new() -> Blink8 {
        Blink8 {
            enabled: false,
            start: 0x8000,
            end: 0x8002,
        }
    }
}

impl BusDevice for Blink8 {
    fn read(&self, address: u16) -> u8 {
        // Blink8 is a write-only device
        println!("Attempted read from Blink8: {:04X}", address);
        0x00
    }

    fn write(&mut self, address: u16, value: u8) {
        // If we wrote FF to 8002, enable the blink8 device
        if address == self.end && value == 0xFF {
            self.enabled = true;
            println!("Blink8 enabled");
        }

        // If we wrote to 8000, print the value
        if address == self.start && self.enabled {
            print!("Blink8: ");
            // print the bit values in reverse order
            for i in 0..8 {
                if value & (1 << i) != 0 {
                    print!("1");
                } else {
                    print!("0");
                }
            }
            println!();
        }
    }

    fn is_memory(&self) -> bool {
        false
    }

    fn reset(&mut self) {
        // This is a no-op since it's a write-only device
        self.enabled = false;
    }

    fn name(&self) -> String {
        String::from("Blink8")
    }

    fn start_address(&self) -> u16 {
        self.start
    }

    fn end_address(&self) -> u16 {
        self.end
    }
}