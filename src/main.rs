use std::cell::RefCell;
use std::rc::Rc;
use crate::bus::{BusDevice, MainBus};
use crate::cpu::Cpu;

mod cpu;
mod bus;
mod register;


struct Emulator {
    cpu: Cpu,
    bus: MainBus,
}

impl Emulator {
    fn new() -> Emulator {
        Emulator {
            cpu: Cpu::new(Rc::new(RefCell::new(MainBus::new()))),
            bus: MainBus::new(),
        }
    }
}

fn main() {
    let mut emulator = Emulator::new();

    let mut ram_device = BusDevice::new(0x0000, 0x7FFF, true, Vec::new());
    ram_device.data = vec![0x00; 0x8000];
    emulator.bus.devices.push(ram_device);

    let io_test_device = BusDevice::new_io(0x8000, 0x8100); //
    emulator.bus.devices.push(io_test_device);

    let mut rom_device = BusDevice::new_memory(0xC000, 0xFFFF, Vec::new());
    rom_device.data = vec![0x00; 0x8000];
    emulator.bus.devices.push(rom_device);

    emulator.cpu.connect_bus(Rc::new(RefCell::new(emulator.bus)));
    emulator.cpu.reset();
    println!("{}", emulator.cpu);
}
