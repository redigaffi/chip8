pub struct RAM {
    memory: [u8; 0x1000], // 4kb (4096 bytes)
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            memory: [0x0; 0x1000],
        }
    }

    pub fn store(&mut self, position: u16, value: u8) {
        let pos: usize = position as usize;
        self.memory[pos] = value;
    }

    pub fn fetch(&self, position: u16) -> u8 {
        self.memory[position as usize]
    }
}
