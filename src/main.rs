use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::organism::Organism;
use crate::vectors::Vector2;

mod cells;
mod organism;
mod vectors;

fn main() -> Result<(), String> {
    let (mut canvas, mut event_pump) = sdl_init()?;
    let mut org = Organism::new(
        Vector2::new(
            (canvas.window().size().0 / 2) as f64,
            (canvas.window().size().1 / 2) as f64,
        ),
        3,
        50.0,
        350.0,
        10.0,
        true,
    );

    'running: loop {
        fill_bg(&mut canvas, sdl2::pixels::Color::BLACK);
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
        org.fill_gaps();
        org.expand();
        org.draw(&mut canvas, false);
        canvas.present();
    }

    Ok(())
}

fn sdl_init() -> Result<(sdl2::render::WindowCanvas, sdl2::EventPump), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Behaviours",
            video_subsystem.display_bounds(0).unwrap().width(),
            video_subsystem.display_bounds(0).unwrap().height(),
        )
        .position_centered()
        .opengl()
        .fullscreen()
        .build()
        .map_err(|e| e.to_string())?;
    let event_pump = sdl_context.event_pump()?;
    let canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    Ok((canvas, event_pump))
}

fn fill_bg(w_canvas: &mut sdl2::render::WindowCanvas, color: sdl2::pixels::Color) {
    w_canvas.set_draw_color(color);
    w_canvas.clear();
}