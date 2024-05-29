pub mod ram;
pub mod rom;
pub mod blink8;

/// Represents a device connected to the bus.
pub trait BusDevice {
    /// Reads a byte from the device at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to read from.
    ///
    /// # Returns
    ///
    /// The byte read from the bus, or 0 if the address is out of range.
    fn read(&self, address: u16) -> u8;

    /// Writes a byte to the device at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to write to.
    /// * `value` - The byte value to write.
    fn write(&mut self, address: u16, value: u8);

    /// Returns whether the device is memory or I/O.
    fn is_memory(&self) -> bool;

    /// Resets the device by clearing its data.
    fn reset(&mut self);

    /// Returns the name of the device.
    fn name(&self) -> String;

    /// Returns the start address of the device.
    fn start_address(&self) -> u16;

    /// Returns the end address of the device.
    fn end_address(&self) -> u16;

    /// Returns the size of the device in bytes.
    ///
    /// The size is calculated by subtracting the start address from the end address,
    /// and adding 1 to account for the inclusive range.
    ///
    /// # Returns
    ///
    /// The size of the device in bytes.
    fn size(&self) -> u16 {
        // Calculate the size of the device
        self.end_address() - self.start_address() + 1
    }
}



/// Represents the main bus of the system.
///
/// The main bus is responsible for managing the various devices connected to it.
pub struct MainBus {
    /// The list of devices connected to the bus.
    ///
    /// Each device is represented by a `Box<dyn BusDevice>` trait object.
    pub devices: Vec<Box<dyn BusDevice>>,
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
            if device.start_address() <= address && address <= device.end_address() {
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
            if device.start_address() <= address && address <= device.end_address() {
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
    pub fn add_device(&mut self, device: Box<dyn BusDevice>) {
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
            if device.start_address() <= address && address <= device.end_address() {
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
            if device.start_address() <= address && address <= device.end_address() {
                // Call the `write` method of the device to perform the write operation
                device.write(address, value);
                return;
            }
        }
        // If the address is not within the range of any device, panic with an error message
        panic!("Address out of range: {:04X}", address);
    }
}