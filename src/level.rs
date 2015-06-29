use sfml::graphics::Texture;
use map::Map;

pub struct Level<'a> {
    name: String,
    map: Map<'a>
}

impl<'a> Level<'a> {
    pub fn new(name: &str) -> Level<'a> {
        //let tiles: &'a Texture = match Texture::new_from_file("res/tiles.png") {
            //Some(tex) => tex,
            //None => panic!("Cannot find: tiles.png")
        //};

        let tiles: Texture = Texture::new_from_file("res/tiles.png").expect("Cannot find: tiles.png");

        Level {
            name: name.to_string(),
            map: Map::new(&tiles)
        }
    }
}
