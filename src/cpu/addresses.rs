/// The address of the Non-Maskable Interrupt (NMI) vector.
///
/// This vector is used to handle hardware interrupts that cannot be masked.
pub const NMI_VECTOR: u16 = 0xFFFA;

/// The address of the Reset vector.
///
/// This vector is used to initialize the CPU and memory when the system is reset.
pub const RESET_VECTOR: u16 = 0xFFFC;

/// The address of the Interrupt Request (IRQ) vector.
///
/// This vector is used to handle software interrupts that can be masked.
pub const IRQ_VECTOR: u16 = 0xFFFE;