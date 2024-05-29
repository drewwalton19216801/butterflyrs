pub struct BusDevice {
    pub start_address: u16,
    pub end_address: u16,
    pub is_memory: bool,
    pub data: Vec<u8>,
}

impl BusDevice {
    pub fn new(start_address: u16, end_address: u16, is_memory: bool, data: Vec<u8>) -> BusDevice {
        BusDevice {
            start_address,
            end_address,
            is_memory,
            data
        }
    }
    pub fn new_memory(start_address: u16, end_address: u16, data: Vec<u8>) -> BusDevice {
        BusDevice::new(start_address, end_address, true, data)
    }

    pub fn new_io(start_address: u16, end_address: u16) -> BusDevice {
        BusDevice::new(start_address, end_address, false, Vec::new())
    }

    pub fn is_memory(&self) -> bool {
        self.is_memory
    }

    pub fn read(&self, address: u16) -> u8 {
        if address >= self.start_address && address <= self.end_address {
            self.data[(address - self.start_address) as usize]
        } else {
            0
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address >= self.start_address && address <= self.end_address {
            self.data[(address - self.start_address) as usize] = value;
        }
    }

    pub fn reset(&mut self) {
        self.data = Vec::new();
    }
}

pub struct MainBus {
    pub devices: Vec<BusDevice>,
}

impl MainBus {
    pub fn new() -> MainBus {
        MainBus {
            devices: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        for device in self.devices.iter_mut() {
            device.reset()
        }
    }

    pub fn is_memory(&self, address: u16) -> bool {
        for device in self.devices.iter() {
            if device.start_address <= address && address <= device.end_address {
                return device.is_memory();
            }
        }
        false
    }

    pub fn is_io(&self, address: u16) -> bool {
        for device in self.devices.iter() {
            if device.start_address <= address && address <= device.end_address {
                return !device.is_memory();
            }
        }
        false
    }

    pub fn add_device(&mut self, device: BusDevice) {
        self.devices.push(device);
    }

    pub fn read(&self, address: u16) -> u8 {
        for device in self.devices.iter() {
            if device.start_address <= address && address <= device.end_address {
                return device.read(address);
            }
        }
        0
    }

    pub fn write(&mut self, address: u16, value: u8) {
        for device in self.devices.iter_mut() {
            if device.start_address <= address && address <= device.end_address {
                device.write(address, value);
                return;
            }
        }
    }
}