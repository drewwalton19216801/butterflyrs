use crate::bus::BusDevice;

/// Represents a Blink8 device.
///
/// The Blink8 device is a custom device that provides a simple way to control an LED.
/// It has an enable flag, a start address, and an end address.
pub struct Blink8 {
    /// Indicates whether the Blink8 device is enabled or not.
    ///
    /// If the device is enabled, the LED will be turned on. Otherwise, it will be turned off.
    pub enabled: bool,

    /// The start address of the Blink8 device.
    ///
    /// This address is used to identify the device on the bus.
    pub start: u16,

    /// The end address of the Blink8 device.
    ///
    /// This address is used to identify the device on the bus.
    pub end: u16,
}

impl Blink8 {
    /// Creates a new instance of the Blink8 device.
    ///
    /// # Returns
    ///
    /// A new instance of the Blink8 device with the default values:
    /// - `enabled` set to `false`
    /// - `start` set to `0x8000`
    /// - `end` set to `0x8002`
    pub fn new() -> Blink8 {
        Blink8 {
            enabled: false,
            start: 0x8000,
            end: 0x8002,
        }
    }
}

impl BusDevice for Blink8 {
    /// Reads data from the Blink8 device.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to read from.
    ///
    /// # Returns
    ///
    /// The data read from the specified address.
    fn read(&self, address: u16) -> u8 {
        // Blink8 is a write-only device, so we always return 0xFF
        0xFF
    }

    /// Writes data to the Blink8 device.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to write to.
    /// * `value` - The data to write.
    fn write(&mut self, address: u16, value: u8) {
        // If we wrote FF to 8002, enable the blink8 device
        if address == self.end && value == 0xFF {
            self.enabled = true;
        }

        // If we wrote to 8000 and the blink8 device is enabled, print the value
        if address == self.start && self.enabled {
            // Print the Blink8 prefix
            print!("{}", self.name() + " ");

            // Print the bit values in reverse order
            for i in 0..8 {
                // Check if the i-th bit is set in the value
                if value & (1 << i) != 0 {
                    print!("1");
                } else {
                    print!("0");
                }
            }

            // Print a newline character to end the line
            println!();
        }
    }

    /// Returns whether the Blink8 device is a memory device or not.
    ///
    /// This function always returns `false` because the Blink8 device is not a memory device.
    ///
    /// # Returns
    ///
    /// Returns `true` if the device is a memory device, `false` otherwise.
    fn is_memory(&self) -> bool {
        // The Blink8 device is not a memory device, so it always returns `false`.
        false
    }

    /// Resets the Blink8 device.
    ///
    /// This function sets the `enabled` flag to `false`, effectively disabling the device.
    /// Since the Blink8 device is write-only, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut blink8 = Blink8::new();
    /// blink8.reset();
    /// ```
    fn reset(&mut self) {
        // This is a no-op since it's a write-only device
        // Setting the `enabled` flag to `false` effectively disables the device.
        self.enabled = false;
    }

    /// Returns the name of the Blink8 device.
    ///
    /// # Returns
    ///
    /// The name of the Blink8 device as a string.
    fn name(&self) -> String {
        // The name of the Blink8 device is "Blink8".
        String::from("Blink8")
    }

    /// Returns the start address of the Blink8 device.
    ///
    /// # Returns
    ///
    /// The start address of the Blink8 device as a `u16`.
    ///
    /// # Examples
    ///
    /// ```
    /// let blink8 = Blink8::new();
    /// assert_eq!(blink8.start_address(), 0x8000);
    /// ```
    fn start_address(&self) -> u16 {
        // Returns the start address of the Blink8 device.
        self.start
    }

    /// Returns the end address of the Blink8 device.
    ///
    /// # Returns
    ///
    /// The end address of the Blink8 device as a `u16`.
    ///
    /// # Examples
    ///
    /// ```
    /// let blink8 = Blink8::new();
    /// assert_eq!(blink8.end_address(), 0x8002);
    /// ```
    fn end_address(&self) -> u16 {
        // Returns the end address of the Blink8 device.
        self.end
    }
}