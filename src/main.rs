extern crate sfml;

use sfml::window::{ContextSettings, VideoMode, event, Close, keyboard};
use sfml::graphics::{RenderWindow, RenderTarget, Color, Font, Text};
use sfml::system::Vector2f;

mod tile;
mod level;
mod map;

use level::{Level};

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32), "Polliquad", Close, &ContextSettings::default()).expect("Window could not be created.");

    let level = Level::new();

    let font = Font::new_from_file("res/fonts/sansation.ttf").expect("Could not load font.");
    let mut fps = Text::new().expect("Could not create text.");

    fps.set_font(&font);
    fps.set_character_size(20);
    fps.set_position(&(Vector2f::new(5.0, 0.0)));
    fps.set_color(&Color::white());
    fps.set_string("FPS: 60");

    while window.is_open() {
        for event in window.events() {
            match event {
                event::KeyPressed{code, ..} => match code {
                    keyboard::Key::Escape => {
                        window.close();
                        break;
                    },
                    _ => println!("key pressed: {:?}", code)
                },
                event::Closed => window.close(),
                _ => {}
            }
        }

        window.clear(&Color::black());

        level.render(&mut window);
        window.draw(&fps);

        window.display()
    }
}
