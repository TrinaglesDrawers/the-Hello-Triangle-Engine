extern crate sdl2;

use legion::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::{Duration, Instant};

mod movable_rects;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectToDraw {
    pos: (f32, f32),
    size: (u32, u32),
    color: Color,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut world = World::default();
    let mut resources = Resources::default();

    let mut schedule = movable_rects::construct(&mut world);

    let mut delta_time: f32;
    let mut now = Instant::now();

    'running: loop {
        let new_now = Instant::now();
        delta_time = new_now.duration_since(now).as_secs_f32();
        now = new_now;

        resources.insert::<f32>(delta_time);

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i / 2, 64, (255 - i) / 2));
        canvas.clear();

        resources.insert::<Canvas<Window>>(canvas);

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

        schedule.execute(&mut world, &mut resources);

        canvas = resources.remove::<Canvas<Window>>().unwrap();

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
