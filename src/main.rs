// *****************************************************************
//  SDL2 "Hello World" on Rust
//
//  Cargo.tomlでのsdl2のバージョン指定で苦しんがが、何とかうまくいった
//  2020/6/28 konao
// *****************************************************************
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let window = video_subsystem
        .window("Hello SDL2", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl2_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
