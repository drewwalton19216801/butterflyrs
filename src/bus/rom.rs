use crate::bus::BusDevice;

pub struct Rom {
    pub data: Vec<u8>,
    pub start: u16,
    pub end: u16,
}

impl Rom {
    pub fn new(start: u16, end: u16) -> Rom {
        Rom {
            data: vec![0x00; (end - start + 1) as usize],
            start,
            end,
        }
    }
}

impl BusDevice for Rom {
    fn read(&self, address: u16) -> u8 {
        self.data[(address - self.start) as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        // ROM is read-only
        println!("Illegal ROM write: {:04X} = {:02X}", address, value);
    }

    fn is_memory(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        // ROM is read-only
        println!("ROM reset, you probably didn't want to do that. Bye bye data!");
        self.data = vec![0x00; (self.end - self.start + 1) as usize];
    }

    fn name(&self) -> String {
        String::from("RAM")
    }

    fn start_address(&self) -> u16 {
        self.start
    }

    fn end_address(&self) -> u16 {
        self.end
    }
}