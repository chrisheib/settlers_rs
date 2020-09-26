use coffee::graphics::{Color, Rectangle, Shape};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub enum TileType {
    TtDirt,
    TtSand,
    TtMountain,
    TtRoad,
    TtOcean,
    TtRiver,
    TtGras,
    TtSnow,
}

fn color_by_tiletype(tt: &mut TileType) -> Color {
    match tt {
        TileType::TtDirt => Color::from_rgb(148, 69, 0),
        TileType::TtSand => Color::from_rgb(92, 92, 92),
        TileType::TtMountain => Color::from_rgb(227, 200, 0),
        TileType::TtRoad => Color::from_rgb(0, 0, 0),
        TileType::TtOcean => Color::from_rgb(0, 72, 255),
        TileType::TtRiver => Color::from_rgb(0, 247, 255),
        TileType::TtGras => Color::from_rgb(0, 200, 0),
        TileType::TtSnow => Color::from_rgb(255, 255, 255),
    }
}

impl Distribution<TileType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0, 8) {
            0 => TileType::TtDirt,
            1 => TileType::TtSand,
            2 => TileType::TtMountain,
            3 => TileType::TtRoad,
            4 => TileType::TtOcean,
            5 => TileType::TtRiver,
            6 => TileType::TtGras,
            7 => TileType::TtSnow,
            _ => TileType::TtDirt,
        }
    }
}

pub struct Tile {
    pub tile_type: TileType,
    pub x: u16,
    pub y: u16,
    width: u16,
    height: u16,
}

impl Tile {
    pub fn new(_tt: TileType, _x: u16, _y: u16) -> Self {
        Tile {
            tile_type: _tt,
            x: _x,
            y: _y,
            width: 30,
            height: 30,
        }
    }
}

impl crate::Drawable for Tile {
    fn draw(&mut self, param: &mut crate::DrawParameter) {
        //if camera. todo: Nur malen, wenn auch im sichtbaren Bereich!
        //if (self.x < 10) & (self.y < 10) {
        param.camera.mesh.fill(
            Shape::Rectangle(Rectangle {
                x: (self.x as f32) * 30f32 + f32::from(param.camera.cameraoffset_x as i16),
                y: (self.y as f32) * 30f32 + f32::from(param.camera.cameraoffset_y as i16),
                width: self.width as f32,
                height: self.height as f32,
            }),
            color_by_tiletype(&mut self.tile_type),
        );
        //}
    }

    /* // Für Quadrate...
    let a = usize::from(
        (((point.coords.a as i16) - self.player.camera.cameraoffset_x) / 30i16)
            as u16,
    );
    let b = usize::from(
        (((point.coords.b as i16) - self.player.camera.cameraoffset_y) / 30i16)
            as u16,
    );
    if (a < self.map.width as i16)
        & (b < self.map.height as i16)
        & (a >= 0i16)
        & (b >= 0i16)
    {
        self.map.tiles[a as usize][b as usize].randomize();
    }*/
}
