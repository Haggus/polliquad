extern crate sfml;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, Close, keyboard};
use sfml::graphics::{RenderWindow, Texture, Sprite, IntRect, RenderTarget, CircleShape, Color};

mod tile;
mod level;
mod map;

use tile::{Tile, TileType};
use level::{Level};

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32), "Polliquad", Close, &ContextSettings::default()).expect("Window could not be created.");

    let mut circle = CircleShape::new().expect("Error, cannot create ball.");
    circle.set_radius(30.);
    circle.set_fill_color(&Color::red());
    circle.set_position(&Vector2f::new(100., 100.));

    let level = Level::new("Level 1");

    let tiles = match Texture::new_from_file("res/tiles.png") {
        Some(tex) => tex,
        None => panic!("Cannot find: tiles.png")
    };

    let grass_tile = Tile::new(200 as f32, 200 as f32, TileType::Grass);
    let road_tile  = Tile::new(200 as f32, 264 as f32, TileType::Road);
    let water_tile = Tile::new(264 as f32, 200 as f32, TileType::Water);

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

        window.draw(&circle);
        render_tile(&mut window, &grass_tile, &tiles);
        render_tile(&mut window, &road_tile, &tiles);
        render_tile(&mut window, &water_tile, &tiles);

        window.display()
    }
}

fn render_tile(window: &mut RenderWindow, tile: &Tile, tiles: &Texture) {
    let mut sprite = Sprite::new().expect("Cannot create new sprite.");
    sprite.set_texture(tiles, true);

    match tile.kind {
        TileType::Grass => sprite.set_texture_rect(&IntRect::new(0, 0, 32, 32)),
        TileType::Road => sprite.set_texture_rect(&IntRect::new(32, 0, 32, 32)),
        TileType::Water => sprite.set_texture_rect(&IntRect::new(0, 32, 32, 32))
    }

    sprite.set_position2f(tile.x, tile.y);
    window.draw(&sprite);
}
