use sfml::graphics::{Texture, Sprite, IntRect, RenderWindow, RenderTarget};
use tile::TileType;
use map::Map;

pub struct Level {
    map: Map,
    tiles: Texture
}

impl Level {
    pub fn new() -> Level {
        Level {
            map: Map::new(),
            tiles: Texture::new_from_file("res/tiles.png").expect("Cannot find: tiles.png")
        }
    }

    pub fn render(&self, window: &mut RenderWindow) -> () {
        for tile in self.map.tiles.iter() {
            let mut sprite = Sprite::new().expect("Cannot create new sprite.");
            sprite.set_texture(&self.tiles, true);

            match tile.kind {
                TileType::Grass => sprite.set_texture_rect(&IntRect::new(0, 0, 32, 32)),
                TileType::Road => sprite.set_texture_rect(&IntRect::new(32, 0, 32, 32)),
                TileType::Water => sprite.set_texture_rect(&IntRect::new(0, 32, 32, 32))
            }

            sprite.set_position2f(tile.x, tile.y);
            window.draw(&sprite);
        }
    }
}
