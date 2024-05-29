use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use crate::bus::{BusDevice, MainBus};
use crate::bus::blink8::Blink8;
use crate::bus::ram::Ram;
use crate::bus::rom::Rom;
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

    let ram_device = Ram::new(0x0000, 0x7FFF);
    emulator.bus.add_device(Box::new(ram_device));

    let blink8_device = Blink8::new();
    emulator.bus.add_device(Box::new(blink8_device));

    let mut rom_device = Rom::new(0xC000, 0xFFFF);
    let mut file = std::fs::File::open("demos/blink.bin").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    rom_device.data = data;
    emulator.bus.add_device(Box::new(rom_device));

    emulator.cpu.connect_bus(Rc::new(RefCell::new(emulator.bus)));
    emulator.cpu.debug = 0;
    emulator.cpu.reset();

    // Clock the CPU a few times just to make sure it works
    for _ in 0..100 {
        emulator.cpu.clock();
    }
}
