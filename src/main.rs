extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

fn main() {
    let mut sdl_context = sdl2::init().video().unwrap();

    let window = sdl_context.window("Polliquad", 800, 600).position_centered().opengl().build().unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();

    let mut running = true;

    while running {
        for event in sdl_context.event_pump().poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false
                },
                _ => {}
            }
        }

        //update
        //render
    }
}
