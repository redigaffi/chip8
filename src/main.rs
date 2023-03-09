mod cpu;
mod ram;
mod gpu;
mod display;
mod utils;

use cpu::CPU;
use ram::RAM;
use gpu::GPU;
use display::create_monitor;
use std::sync::mpsc::channel;
use std::{thread, time};


fn main() {
    println!("Starting chip-8 emulator");

    let (tx, rx) = channel::<[[bool; gpu::SCREEN_WIDTH as usize]; gpu::SCREEN_HEIGHT as usize]>();
    
    std::thread::spawn(move || {
        create_monitor(rx);
    });

    // Wait until monitor is ready
    thread::sleep(time::Duration::from_millis(1000));

    let mut gpu = GPU::new(tx);
    let mut ram = RAM::new();
    let mut cpu = CPU::new();

    utils::load_program_to_ram(
        &mut ram, 
        //utils::load_program(String::from("./roms/IBMLogo.ch8"))
        //utils::load_program(String::from("./roms/shitty_test_opcode1.ch8"))
        utils::load_program(String::from("./roms/OpcodeTest.ch8"))
    );

    cpu.execute(&mut ram, &mut gpu);

    println!("End chip-8 emulator");

}