extern crate sdl2;

use legion::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

mod movable_rects;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectToDraw {
    pos: (i32, i32),
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

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
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

        let mut query = <&RectToDraw>::query();

        // you can then iterate through the components found in the world
        for rect in query.iter(&world) {
            canvas.set_draw_color(rect.color);
            canvas.fill_rect(Rect::new(
                rect.pos.0 - rect.size.0 as i32 / 2,
                rect.pos.1 - rect.size.1 as i32 / 2,
                rect.size.0,
                rect.size.1,
            ));
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
