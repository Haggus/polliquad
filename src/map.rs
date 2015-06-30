use tile::{Tile, TileType};

pub struct Map {
    pub tiles: Vec<Tile>
}

impl Map {
    pub fn new() -> Map {
        let mut temp: Vec<Tile> = Vec::new();

        //init
        for i in 0..24 {
            let mut tile = Tile::new(i as f32 * 32.0, 0.0, TileType::Grass);
            temp.push(tile);
            tile = Tile::new(i as f32 * 32.0, 32.0, TileType::Water);
            temp.push(tile);
        }
        //init -> to be removed later

        Map {
            tiles: temp
        }
    }
}
