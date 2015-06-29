pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub kind: TileType,
}

pub enum TileType {
    Grass,
    Road,
    Water
}

impl Tile {
    pub fn new(x: f32, y: f32, kind: TileType) -> Tile {
        Tile {
            x: x,
            y: y,
            kind: kind
        }
    }
}
