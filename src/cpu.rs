
use super::ram::RAM;
use super::gpu::GPU;
use rand::Rng;

const OP_CLEAR_SCREN: u16 = 0x00E0;
const OP_JMP: u16 = 0x1000;
const OP_JUMP_EQ_VX_NN: u16 = 0x3000;
const OP_JUMP_NOT_EQ_VX: u16 = 0x4000;
const OP_JUMP_EQ_VX_VX: u16 = 0x5000;
const OP_JUMP_NOT_EQ_VX_VX: u16 = 0x9000;
const OP_SET_REGISTER: u16 = 0x6000;
const OP_SUM_TO_REGISTER: u16 = 0x7000;
const OP_SET_INDEX_REGISTER: u16 = 0xA000;
const OP_DRAW: u16 = 0xD000;
const OP_RETURN: u16 = 0x00EE;
const OP_CALL_SUBROUTINE: u16 = 0x2000;
const OP_SET: u16 = 0x8000;
const OP_BINARY_OR: u16 = 0x8001;
const OP_BINARY_AND: u16 = 0x8002;
const OP_BINARY_XOR: u16 = 0x8003;
const OP_ADD: u16 = 0x8004;
const OP_SUBSTRACT_VX: u16 = 0x8005;
const OP_SUBSTRACT_VY: u16 = 0x8007;
const OP_SHIFT_RIGHT: u16 = 0x8006;
const OP_SHIFT_LEFT: u16 = 0x800E;
const OP_JMP_WITH_OFFSET: u16 = 0xB000;
const OP_RANDOM: u16 = 0xC000;
const OP_STR_RAM: u16 = 0xF055;
const OP_LOAD_RAM: u16 = 0xF065;
const OP_BINARY_CODED_DECIMAL_CONVERSION: u16 = 0xF033;


pub struct CPU {
    // Program Counter (also known as instruction pointer)
    pc: u16,

    // Stack pointer
    sp: u8,
    stack: [u16; 16],

    // Address register
    i: u16,

    // General purpose register
    registers: [u8; 0x10], // from 0x0 to 0xF: one for each hexadecimal digit
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            pc: 0x200,
            sp: 0x0,
            stack: [0x0; 16],
            registers: [0x0; 0x10],
            i: 0x0,
        };
    }

    pub fn execute(&mut self, ram: &mut RAM, display: &mut GPU) {
        loop {
            let high: u16 = ram.fetch(self.pc) as u16;
            let low: u8 = ram.fetch(self.pc + 0x1);
            let full_instruction = (high << 8) | low as u16;
            
            let a = (1,2);
            
            match full_instruction {
                1 => {},
                0..=u16::MAX => {},
            };

            // From most significat to least.
            let first_nibble = full_instruction & 0xF000;
            let second_nibble = full_instruction & 0x0F00;
            let third_nibble = full_instruction & 0x00F0;
            let forth_nibble = full_instruction & 0x000F;
            
            if full_instruction == OP_CLEAR_SCREN {
                println!("Clear screen {}", self.pc);
                display.clear_pixels();

            } else if first_nibble == OP_JMP {
                //println!("jump {}", self.pc);
                self.pc = second_nibble | third_nibble | forth_nibble;
                continue;

            } else if first_nibble == OP_JUMP_EQ_VX_NN {
                println!("OP_JUMP_EQ_VX_NN");

                let register = (second_nibble >> 8) as usize;
                let n = (third_nibble | forth_nibble) as u8;

                if self.registers[register] == n {
                    self.pc += 0x4;
                    continue;
                } 

            } else if first_nibble == OP_JUMP_NOT_EQ_VX {
                let register = (second_nibble >> 8) as usize;
                let n = (third_nibble | forth_nibble) as u8;

                if self.registers[register] != n {
                    self.pc += 0x4;
                    continue;
                }
                
            }  else if first_nibble == OP_JUMP_EQ_VX_VX  {
                let x = (second_nibble >> 8) as usize;
                let y = (third_nibble >> 4) as usize;

                if self.registers[x] == self.registers[y] {
                    self.pc += 0x2;
                    continue;
                }
                

            } else if first_nibble == OP_JUMP_NOT_EQ_VX_VX {
                let x = (second_nibble >> 8) as usize;
                let y = (third_nibble >> 4) as usize;

                if self.registers[x] != self.registers[y] {
                    self.pc += 0x2;
                    continue;
                }

            } else if first_nibble == OP_SET_REGISTER {
                let register = (second_nibble >> 8) as usize;
                self.registers[register] = (third_nibble | forth_nibble) as u8;

            } else if first_nibble == OP_SUM_TO_REGISTER {

                let register = (second_nibble >> 8) as usize;
                // use plus instead of binary OR as if not overflows will display wrong numbers
                
                let tmp: u16 = self.registers[register] as u16 + (third_nibble | forth_nibble) as u16;
                if tmp > 0xFF {
                    self.registers[register] = (tmp % 0x100) as u8;
                } else {
                    self.registers[register] = tmp as u8;
                }

            } else if first_nibble == OP_SET_INDEX_REGISTER {

                self.i = second_nibble | third_nibble | forth_nibble;
                println!("set index register I to: 0x{:x}", self.i);

            } else if first_nibble == OP_DRAW {                
                self.registers[0xF] = 0;

                let register_x = (second_nibble >> 8) as usize;
                let register_y = (third_nibble >> 4) as usize;
                
                let x: u8 = self.registers[register_x];
                let y: u8 = self.registers[register_y];

                let mut collision_flag = false;

                for n in  0..(forth_nibble as u8) {
                    let byte_data: u8 = ram.fetch(self.i + (n as u16));
                    
                    collision_flag |= display.set_pixel(y+n, x+0, (byte_data & 0b10000000) > 0);
                    collision_flag |= display.set_pixel(y+n, x+1, (byte_data & 0b01000000) > 0);
                    collision_flag |= display.set_pixel(y+n, x+2, (byte_data & 0b00100000) > 0);
                    collision_flag |= display.set_pixel(y+n, x+3, (byte_data & 0b00010000) > 0);
                    collision_flag |= display.set_pixel(y+n, x+4, (byte_data & 0b00001000) > 0);
                    collision_flag |= display.set_pixel(y+n, x+5, (byte_data & 0b00000100) > 0);
                    collision_flag |= display.set_pixel(y+n, x+6, (byte_data & 0b00000010) > 0);
                    collision_flag |= display.set_pixel(y+n, x+7, (byte_data & 0b00000001) > 0);
                }

                if collision_flag {
                    self.registers[0xF] = 0x1;
                }

                println!("0x{:x} display/draw {} {}", full_instruction, y, x);
            } else if first_nibble == OP_CALL_SUBROUTINE {
                
                // Store current program counter to the stack, so when a function returns we restore previous 
                // program counter

                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = second_nibble | third_nibble | forth_nibble;
                continue;
                
            } else if full_instruction == OP_RETURN {
                println!("OP_RETURN");

                // pop from stack
                self.sp -= 1;
                let previous_pc = self.stack[self.sp as usize];
                self.stack[self.sp as usize] = 0x0;
                self.pc = previous_pc;

            } else if first_nibble | forth_nibble == OP_SET {
                println!("OP_SET");

                let x = second_nibble >> 8;
                let y = third_nibble >> 4;
                self.registers[x as usize] = self.registers[y as usize];
            } else if first_nibble | forth_nibble == OP_BINARY_OR {
                println!("OP_BINARY_OR");

                let x = second_nibble >> 8;
                let y = third_nibble >> 4;
                self.registers[x as usize] |= self.registers[y as usize];
            } else if first_nibble | forth_nibble == OP_BINARY_AND {
                println!("OP_BINARY_AND");

                let x = second_nibble >> 8;
                let y = third_nibble >> 4;
                self.registers[x as usize] &= self.registers[y as usize];
            } else if first_nibble | forth_nibble == OP_BINARY_XOR {
                println!("OP_BINARY_AND");

                let x = second_nibble >> 8;
                let y = third_nibble >> 4;
                self.registers[x as usize] ^= self.registers[y as usize];
            
            } else if first_nibble | forth_nibble == OP_ADD {
                println!("OP_ADD");

                let x = second_nibble >> 8;
                let y = third_nibble >> 4;

                
                let tmp: u16 = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
                if tmp > 0xFF {
                    self.registers[0xF] = 0x1;
                    self.registers[x as usize] = ((tmp % 0xFF) - 1_u16) as u8;
                } else {
                    self.registers[x as usize] = tmp as u8;
                }
                /*if sum > u8::MAX {
                    self.registers[0xF] = 0x1;
                } else {
                    self.registers[0xF] = 0x0;
                }*/


            } else if first_nibble | forth_nibble == OP_SUBSTRACT_VX {
                println!("OP_SUBSTRACT_VX");
                
                let x = second_nibble >> 8;
                let y = third_nibble >> 4;

                if self.registers[x as usize] > self.registers[y as usize] {
                    self.registers[0xF] = 1;
                    self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
                } else {
                    self.registers[0xF] = 0;
                    self.registers[x as usize] = u8::MAX - (self.registers[y as usize] - self.registers[x as usize]) +1;
                }
                


            } else if first_nibble | forth_nibble == OP_SUBSTRACT_VY {
                println!("OP_SUBSTRACT_VY");

                let x = second_nibble >> 8;
                let y = third_nibble >> 4;

                if self.registers[y as usize] > self.registers[x as usize] {
                    self.registers[0xF] = 1;
                    self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
                } else {
                    self.registers[0xF] = 0;
                    self.registers[x as usize] = 0;
                }
            } else if first_nibble | forth_nibble == OP_SHIFT_LEFT {
                println!("OP_SHIFT_LEFT");

                //@TODO: This might change depending on program implementation
                let x = second_nibble >> 8;
                let y = third_nibble >> 4;

                if (self.registers[x as usize] & 0b10000000) > 0 {
                    self.registers[0xF] = 0x1;
                } else {
                    self.registers[0xF] = 0x0;
                }

                self.registers[x as usize] = self.registers[x as usize] << 0x1;
                
            } else if first_nibble | forth_nibble == OP_SHIFT_RIGHT { 
                println!("OP_SHIFT_RIGHT");
                //@TODO: This might change depending on program implementation
                let x = second_nibble >> 8;
                let y = third_nibble >> 4;

                if (self.registers[x as usize] & 0b00000001) > 0 {
                    self.registers[0xF] = 0x1;
                } else {
                    self.registers[0xF] = 0x0;
                }

                self.registers[x as usize] = self.registers[x as usize] >> 0x1;
            } else if first_nibble == OP_JMP_WITH_OFFSET {
                println!("OP_JMP_WITH_OFFSET");

                self.pc = (second_nibble | third_nibble | forth_nibble) + (self.registers[0x0] as u16);
                continue;
            } else if first_nibble == OP_RANDOM {
                println!("OP_RANDOM");
                let n = (third_nibble >> 4 | forth_nibble) as u8;
                
                let mut rng = rand::thread_rng();
                let rand: u8 = rng.gen::<u8>();
                
                let x = (second_nibble >> 8) as usize;
                self.registers[x] = rand | n;
                
            } else if OP_STR_RAM & 0xF000 == first_nibble && OP_STR_RAM & 0x00FF == (third_nibble | forth_nibble)  {
                let x = second_nibble >> 8;
                
                for n in  0..(x+1) {
                    ram.store(self.i + n, self.registers[n as usize]);
                }

            } else if OP_LOAD_RAM & 0xF000 == first_nibble && OP_LOAD_RAM & 0x00FF == (third_nibble | forth_nibble) {
                let x = second_nibble >> 8;
                
                for n in  0..(x+1) {
                    self.registers[n as usize] = ram.fetch(self.i + n);
                }

            } else if OP_BINARY_CODED_DECIMAL_CONVERSION & 0xF000 == first_nibble && OP_BINARY_CODED_DECIMAL_CONVERSION & 0x00FF == (third_nibble | forth_nibble) {
                let x = second_nibble >> 8;
                let num: u8 = self.registers[x as usize];
                let hundreds = num / 100;
                let tenths  =  (num - (hundreds*100))/10;
                let units = num - (hundreds*100) - (tenths*10);
                
                ram.store(self.i, hundreds);
                ram.store(self.i+0x1, tenths);
                ram.store(self.i+0x2, units);
            }
            
            else {
                //panic!("Instruction 0x{:x} not implemented", full_instruction);
            }

            
            self.pc += 0x2;
        }
    }
}

