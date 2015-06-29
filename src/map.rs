use sfml::graphics::Texture;
use tile::{Tile, TileType};

pub struct Map<'a> {
    tiles: Vec<Tile<'a>>
}

impl<'a> Map<'a> {
    pub fn new(texture: &'a Texture) -> Map<'a> {
        let mut temp: Vec<Tile> = Vec::new();

        //init
        for i in 0..8 {
            let tile = Tile::new(i as f32 * 32.0, 0.0, TileType::Grass, &texture);
            temp.push(tile);
        }

        Map {
            tiles: temp
        }
    }
}
