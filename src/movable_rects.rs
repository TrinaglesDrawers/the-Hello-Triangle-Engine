use crate::RectToDraw;
use legion::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

#[derive(Clone, Copy, Debug, PartialEq)]
struct MovingRectComponent {
    pos_min: (i32, i32),
    pos_max: (i32, i32),
    speed: i32,
    direction: (bool, bool),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ScalableRectComponent {
    scale_min: (u32, u32),
    scale_max: (u32, u32),
    scale_step: u32,
    direction: (bool, bool),
}

pub fn construct(world: &mut legion::World) -> Schedule {
    // let mut world = World::default();

    let _entity: Entity = world.push((
        MovingRectComponent {
            pos_min: (100, 100),
            pos_max: (700, 500),
            speed: 11,
            direction: (true, true),
        },
        ScalableRectComponent {
            scale_min: (16, 16),
            scale_max: (32, 32),
            scale_step: 2,
            direction: (true, true),
        },
        RectToDraw {
            pos: (200.0, 200.0),
            size: (24, 24),
            color: Color::RGB(250, 150, 0),
        },
    ));

    world.push((
        MovingRectComponent {
            pos_min: (50, 100),
            pos_max: (700, 300),
            speed: 40,
            direction: (true, true),
        },
        ScalableRectComponent {
            scale_min: (16, 16),
            scale_max: (24, 24),
            scale_step: 2,
            direction: (true, true),
        },
        RectToDraw {
            pos: (300.0, 300.0),
            size: (20, 20),
            color: Color::RGB(0, 150, 0),
        },
    ));

    world.push((
        MovingRectComponent {
            pos_min: (130, 190),
            pos_max: (500, 500),
            speed: 20,
            direction: (true, false),
        },
        ScalableRectComponent {
            scale_min: (16, 10),
            scale_max: (28, 30),
            scale_step: 2,
            direction: (true, true),
        },
        RectToDraw {
            pos: (150.0, 150.0),
            size: (24, 24),
            color: Color::RGB(200, 150, 111),
        },
    ));

    world.push((
        MovingRectComponent {
            pos_min: (130, 490),
            pos_max: (500, 500),
            speed: 60,
            direction: (true, false),
        },
        RectToDraw {
            pos: (150.0, 490.0),
            size: (14, 14),
            color: Color::RGB(200, 250, 100),
        },
    ));

    world.push((
        ScalableRectComponent {
            scale_min: (10, 10),
            scale_max: (30, 30),
            scale_step: 1,
            direction: (true, true),
        },
        RectToDraw {
            pos: (550.0, 350.0),
            size: (24, 24),
            color: Color::RGB(100, 150, 100),
        },
    ));

    let schedule = Schedule::builder()
        .add_system(update_rect_position_system())
        .add_system(update_rect_size_system())
        .add_thread_local(upload_rect_to_draw_system())
        .build();

    return schedule;
}

#[system(for_each)]
fn update_rect_position(
    rect: &mut RectToDraw,
    move_component: &mut MovingRectComponent,
    #[resource] delta_time: &f32,
) {
    if rect.pos.0 as i32 >= move_component.pos_max.0 {
        move_component.direction.0 = false;
    } else if rect.pos.0 as i32 <= move_component.pos_min.0 {
        move_component.direction.0 = true;
    }

    if move_component.direction.0 {
        rect.pos.0 = rect.pos.0 + delta_time * move_component.speed as f32;
    } else {
        rect.pos.0 = rect.pos.0 - delta_time * move_component.speed as f32;
    }

    if rect.pos.1 >= move_component.pos_max.1 as f32 {
        move_component.direction.1 = false;
    } else if rect.pos.1 <= move_component.pos_min.1 as f32 {
        move_component.direction.1 = true;
    }

    if move_component.direction.1 {
        rect.pos.1 = rect.pos.1 + delta_time * move_component.speed as f32;
    } else {
        rect.pos.1 = rect.pos.1 - delta_time * move_component.speed as f32;
    }
}

#[system(for_each)]
fn update_rect_size(rect: &mut RectToDraw, scale_component: &mut ScalableRectComponent) {
    if rect.size.0 >= scale_component.scale_max.0 {
        scale_component.direction.0 = false;
    } else if rect.size.0 <= scale_component.scale_min.0 {
        scale_component.direction.0 = true;
    }

    if scale_component.direction.0 {
        rect.size.0 += scale_component.scale_step;
    } else {
        rect.size.0 -= scale_component.scale_step;
    }

    if rect.size.1 >= scale_component.scale_max.1 {
        scale_component.direction.1 = false;
    } else if rect.size.1 <= scale_component.scale_min.1 {
        scale_component.direction.1 = true;
    }

    if scale_component.direction.1 {
        rect.size.1 += scale_component.scale_step;
    } else {
        rect.size.1 -= scale_component.scale_step;
    }
}

#[system(for_each)]
fn upload_rect_to_draw(rect: &RectToDraw, #[resource] canvas: &mut Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(rect.color);
    let _result = canvas.fill_rect(Rect::new(
        rect.pos.0 as i32 - rect.size.0 as i32 / 2,
        rect.pos.1 as i32 - rect.size.1 as i32 / 2,
        rect.size.0,
        rect.size.1,
    ));
}
