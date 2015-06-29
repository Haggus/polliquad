extern crate sfml;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::graphics::{RenderWindow, Texture, RenderTarget, CircleShape, Color};

mod tile;
use tile::{Tile, TileType};

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32), "Polliquad", Close, &ContextSettings::default()).expect("Window could not be created.");

    let mut circle = CircleShape::new().expect("Error, cannot create ball.");
    circle.set_radius(30.);
    circle.set_fill_color(&Color::red());
    circle.set_position(&Vector2f::new(100., 100.));

    let tiles = match Texture::new_from_file("res/tiles.png") {
        Some(tex) => tex,
        None => panic!("Cannot find: tiles.png")
    };

    let grass_tile = Tile::new(200 as f32, 200 as f32, TileType::Grass, &tiles);
    let road_tile  = Tile::new(200 as f32, 264 as f32, TileType::Road, &tiles);
    let water_tile = Tile::new(264 as f32, 200 as f32, TileType::Water, &tiles);

    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _ => {}
            }
        }

        // Clear the window
        //window.clear(&Color::new_RGB(0, 200, 200));
        window.clear(&Color::black());
        // Draw the shape
        window.draw(&circle);
        window.draw(&grass_tile.sprite);
        window.draw(&road_tile.sprite);
        window.draw(&water_tile.sprite);
        // Display things on screen
        window.display()
    }
}
