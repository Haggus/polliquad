use sfml::graphics::Texture;
use tile::{Tile, TileType};

pub struct Map {
    tiles: Vec<Tile>
}

impl Map {
    pub fn new() -> Map {
        let mut temp: Vec<Tile> = Vec::new();

        //init
        for i in 0..8 {
            let tile = Tile::new(i as f32 * 32.0, 0.0, TileType::Grass);
            temp.push(tile);
        }

        Map {
            tiles: temp
        }
    }
}
