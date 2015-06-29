use sfml::graphics::{Sprite, Texture, IntRect};

pub struct Tile<'a> {
    kind: TileType,
    pub sprite: Sprite<'a>
}

pub enum TileType {
    Grass,
    Road,
    Water
}

impl<'a> Tile<'a> {
    pub fn new(x: f32, y: f32, t: TileType, texture: &'a Texture) -> Tile<'a> {
        let mut sprite = Sprite::new().expect("Cannot create a new sprite.");
        sprite.set_texture(texture, true);

        match t {
            TileType::Grass => sprite.set_texture_rect(&IntRect::new(0, 0, 32, 32)),
            TileType::Road  => sprite.set_texture_rect(&IntRect::new(32, 0, 32, 32)),
            TileType::Water => sprite.set_texture_rect(&IntRect::new(0, 32, 32, 32)),
        }

        sprite.set_position2f(x, y);

        Tile {
            kind: t,
            sprite: sprite
        }
    }
}
