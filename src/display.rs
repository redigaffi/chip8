
extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::sys;
use sdl2::rect::Rect;
use std::{thread, time};
use std::sync::mpsc;


const DISPLAY_SCALE: u32 = 20;

// 60 FPS
const FRAME_RATE: Duration = time::Duration::from_millis(1000/60); 

pub fn create_monitor(data: mpsc::Receiver<[[bool; 64]; 32]>) {
    

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("chip-8 emulator", 64*DISPLAY_SCALE, 32*DISPLAY_SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut pixels = [[false; 64]; 32];

    let mut now = time::Instant::now();

    let mut i = 0;
    loop {
        let tmp = data.try_recv();
        if tmp.is_ok() {
            pixels = tmp.unwrap();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in  0..32 {
            for x in 0..64 {
                if pixels[y][x] {
                    let x: i32 = (x + (x * (DISPLAY_SCALE as usize -1))) as i32;
                    let y: i32 = (y + (y * (DISPLAY_SCALE as usize -1))) as i32;

                    canvas.fill_rect(
                        Rect::new(x, y, DISPLAY_SCALE, DISPLAY_SCALE)
                    ).unwrap();
                }
            }
        }
                
        canvas.present();

        i += 1;
        if now.elapsed() >= time::Duration::from_millis(1000) {
            //println!("FPS: {}", i);
            i = 0;
            now = time::Instant::now();
        }

        //thread::sleep(FRAME_RATE);

    }
}