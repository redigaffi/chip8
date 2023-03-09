use std::fs::File;
use std::io::Read;
use super::ram::RAM;

pub fn load_program(path: String) -> Vec<u8> {
    let mut file = File::open(path)
        .expect("Cant find file");
    
    let mut program_bytes: Vec<u8> = Vec::new();
    let _bytes_read = file.read_to_end(&mut program_bytes).unwrap();
    
    program_bytes
}

pub fn load_program_to_ram(ram: &mut RAM, program_bytes: Vec<u8>) {
    let mut position: u16 = 0x200; // chip-8 programs start at position 0x200
    for byte in program_bytes {
        ram.store(position, byte);
        position += 0x1;
    }
}