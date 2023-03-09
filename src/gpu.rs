pub const SCREEN_WIDTH: u8 = 64;
pub const SCREEN_HEIGHT: u8 = 32;

use std::sync::mpsc;

pub struct GPU {
    pixels: [[bool; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
    display_port: mpsc::Sender<[[bool; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize]>,
}

impl GPU {
    pub fn new(display_port: mpsc::Sender<[[bool; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize]>) -> GPU {
        GPU {
            pixels: [[false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
            display_port: display_port
        }
    }

    pub fn clear_pixels(&mut self) {
        /*for y in  0..SCREEN_HEIGHT as usize {
            for x in 0..SCREEN_WIDTH as usize {
                self.pixels[y][x] = false;
            }
        }*/
        self.pixels = [[false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
        self.display_port.send(self.pixels).unwrap();
    }

    pub fn set_pixel(&mut self, y: u8, x: u8, data: bool) -> bool {
        let x = x as usize % SCREEN_WIDTH as usize;
        let y = y as usize % SCREEN_HEIGHT as usize;

        let collission_flag = data && self.pixels[y][x];
        
        self.pixels[y][x] ^= data;
        
        self.display_port.send(self.pixels).unwrap();

        collission_flag
    }

    pub fn get_pixel(&self, y: u8, x: u8) -> bool {
        return self.pixels[y as usize][x as usize];
    }
}