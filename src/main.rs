use nes_emulator::cpu::Cpu;


fn main() {
    let mut cpu = Cpu::new();

    cpu.startup().unwrap();

    loop {
        if let Err(_) = cpu.tick() {
            break;
        }
    }

    cpu.shutdown().unwrap();
}
