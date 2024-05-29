/// Represents a device connected to the bus.
pub struct BusDevice {
    /// The start address of the device's memory range.
    pub start_address: u16,
    /// The end address of the device's memory range.
    pub end_address: u16,
    /// Indicates whether the device is memory or I/O.
    pub is_memory: bool,
    /// The data stored in the device's memory range.
    pub data: Vec<u8>,
}

impl BusDevice {
    /// Creates a new instance of the `BusDevice` struct.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The start address of the device's memory range.
    /// * `end_address` - The end address of the device's memory range.
    /// * `is_memory` - Indicates whether the device is memory or I/O.
    /// * `data` - The initial data stored in the device's memory range.
    ///
    /// # Returns
    ///
    /// A new instance of the `BusDevice` struct.
    pub fn new(start_address: u16, end_address: u16, is_memory: bool, data: Vec<u8>) -> BusDevice {
        BusDevice {
            start_address,
            end_address,
            is_memory,
            data
        }
    }

    /// Creates a new `BusDevice` with memory range and initial data.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The start address of the device's memory range.
    /// * `end_address` - The end address of the device's memory range.
    /// * `data` - The initial data stored in the device's memory range.
    ///
    /// # Returns
    ///
    /// A new instance of the `BusDevice` struct.
    pub fn new_memory(start_address: u16, end_address: u16, data: Vec<u8>) -> BusDevice {
        // Call the `new` function of `BusDevice` with the provided arguments
        BusDevice::new(start_address, end_address, true, data)
    }

    /// Creates a new `BusDevice` for I/O operations.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The start address of the device's memory range.
    /// * `end_address` - The end address of the device's memory range.
    ///
    /// # Returns
    ///
    /// A new instance of the `BusDevice` struct for I/O operations.
    pub fn new_io(start_address: u16, end_address: u16) -> BusDevice {
        // Call the `new` function of `BusDevice` with the provided arguments
        BusDevice::new(start_address, end_address, false, Vec::new())
    }

    /// Checks if the device is memory or I/O.
    ///
    /// # Returns
    ///
    /// Returns `true` if the device is memory, `false` otherwise.
    pub fn is_memory(&self) -> bool {
        // Returns the value of the `is_memory` field
        self.is_memory
    }

    /// Reads a byte from the bus at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to read from.
    ///
    /// # Returns
    ///
    /// The byte read from the bus, or 0 if the address is out of range.
    pub fn read(&self, address: u16) -> u8 {
        // Check if the address is within the range of the bus device's memory
        if address >= self.start_address && address <= self.end_address {
            // Return the byte at the specified address
            self.data[(address - self.start_address) as usize]
        } else {
            // Return 0 if the address is out of range
            0
        }
    }

    /// Writes a byte to the bus at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to write to.
    /// * `value` - The byte value to write.
    ///
    /// # Panics
    ///
    /// If the address is out of range, the function will panic.
    pub fn write(&mut self, address: u16, value: u8) {
        // Check if the address is within the range of the bus device's memory
        if address >= self.start_address && address <= self.end_address {
            // Calculate the index of the byte in the `data` vector based on the address
            let index = (address - self.start_address) as usize;
            // Write the value to the specified index in the `data` vector
            self.data[index] = value;
        } else {
            // If the address is out of range, panic with an error message
            panic!("Address out of range: {:04X}", address);
        }
    }

    /// Resets the bus device by clearing its data.
    ///
    /// This function clears the `data` vector of the bus device, resetting it to an empty state.
    pub fn reset(&mut self) {
        // Clear the data vector
        self.data = Vec::new();
    }
}

/// Represents the main bus of the system.
///
/// The main bus is responsible for managing the various devices connected to it.
pub struct MainBus {
    /// The list of devices connected to the bus.
    ///
    /// Each device is represented by a `BusDevice` struct.
    pub devices: Vec<BusDevice>,
}

impl MainBus {
    /// Creates a new instance of the `MainBus` struct.
    ///
    /// # Returns
    ///
    /// A new instance of the `MainBus` struct with an empty list of devices.
    pub fn new() -> MainBus {
        MainBus {
            devices: Vec::new(),
        }
    }

    /// Resets all the devices connected to the bus.
    ///
    /// This function clears the data of each device, resetting them to an empty state.
    pub fn reset(&mut self) {
        for device in self.devices.iter_mut() {
            device.reset();
        }
    }

    /// Checks if the given `address` is within the range of any memory devices connected to the bus.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the address is within the range of a memory device, `false` otherwise.
    pub fn is_memory(&self, address: u16) -> bool {
        // Iterate over each device connected to the bus
        for device in self.devices.iter() {
            // Check if the address is within the range of the current device
            if device.start_address <= address && address <= device.end_address {
                // If the device is memory, return `true`
                // If the device is I/O, continue to the next device
                return device.is_memory();
            }
        }
        // If the address is not within the range of any device, return `false`
        false
    }

    /// Checks if the given `address` is within the range of any I/O devices connected to the bus.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the address is within the range of an I/O device, `false` otherwise.
    pub fn is_io(&self, address: u16) -> bool {
        // Iterate over each device connected to the bus
        for device in self.devices.iter() {
            // Check if the address is within the range of the current device
            if device.start_address <= address && address <= device.end_address {
                // If the device is memory, return `false`
                // If the device is I/O, return `true`
                return !device.is_memory();
            }
        }
        // If the address is not within the range of any device, return `false`
        false
    }

    /// Adds a device to the bus.
    ///
    /// # Arguments
    ///
    /// * `device` - The device to add to the bus.
    pub fn add_device(&mut self, device: BusDevice) {
        // Push the device to the list of devices connected to the bus.
        self.devices.push(device);
    }

    /// Reads a byte from the bus at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to read from.
    ///
    /// # Returns
    ///
    /// The byte read from the bus, or 0 if the address is out of range.
    pub fn read(&self, address: u16) -> u8 {
        // Iterate over each device connected to the bus
        for device in self.devices.iter() {
            // Check if the address is within the range of the current device
            if device.start_address <= address && address <= device.end_address {
                // Return the byte read from the device
                return device.read(address);
            }
        }
        // If the address is not within the range of any device, return 0
        0
    }

    /// Writes a byte to the bus at the specified address.
    ///
    /// This function iterates over each device connected to the bus and checks if the address is within the range of the current device.
    /// If the address is within the range, it calls the `write` method of the device to perform the write operation.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to write to.
    /// * `value` - The byte value to write.
    ///
    /// # Panics
    ///
    /// If the address is out of range, the function will panic.
    pub fn write(&mut self, address: u16, value: u8) {
        // Iterate over each device connected to the bus
        for device in self.devices.iter_mut() {
            // Check if the address is within the range of the current device
            if device.start_address <= address && address <= device.end_address {
                // Call the `write` method of the device to perform the write operation
                device.write(address, value);
                return;
            }
        }
        // If the address is not within the range of any device, panic with an error message
        panic!("Address out of range: {:04X}", address);
    }
}