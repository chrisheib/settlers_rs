use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use coffee::graphics::{Mesh, Shape, Rectangle, Color};

pub enum TileType {
    TtDirt,
    TtSand,
    TtMountain,
    TtRoad,
    TtOcean,
    TtRiver,
}

fn color_by_tiletype(tt:&mut TileType) -> Color{
    match tt {
        TileType::TtDirt => Color::from_rgb(148, 69, 0),
        TileType::TtSand => Color::from_rgb(92, 92, 92),
        TileType::TtMountain => Color::from_rgb(227, 200, 0),
        TileType::TtRoad => Color::from_rgb(0, 0, 0),
        TileType::TtOcean => Color::from_rgb(0, 72, 255),
        TileType::TtRiver => Color::from_rgb(0, 247, 255),
    }
}

impl Distribution<TileType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0, 5) {
            0 => TileType::TtDirt,
            1 => TileType::TtSand,
            2 => TileType::TtMountain,
            3 => TileType::TtRoad,
            4 => TileType::TtOcean,
            5 => TileType::TtRiver,
            _ => TileType::TtDirt,
        }
    }
}

pub struct Tile {
    pub tile_type: TileType,
    pub x : u16,
    pub y : u16,
}

impl Tile{
    pub fn new(_tt : TileType, _x : u16, _y : u16) -> Self{
        Tile{
            tile_type : _tt,
            x : _x,
            y : _y,
        }
    }

    pub fn draw_tile(&mut self, mesh: &mut Mesh){
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: (self.x as f32) * 30f32,
                y: (self.y as f32) * 30f32,
                width: 30.0,
                height: 30.0,
            }),
            color_by_tiletype(&mut self.tile_type),
        );

    }
}