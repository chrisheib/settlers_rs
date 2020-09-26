use coffee::graphics::{Color, Point, Rectangle, Shape::Polyline};
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
    pub width: u16,
    pub height: u16,
}

impl Tile {
    pub fn new(_tt: TileType, _x: u16, _y: u16) -> Self {
        Tile {
            tile_type: _tt,
            x: _x,
            y: _y,
            width: crate::FIELDWIDTH,
            height: crate::FIELDHEIGHT,
        }
    }
}

impl crate::Drawable for Tile {
    fn draw(&mut self, param: &mut crate::DrawParameter) {
        //if camera. todo: Nur malen, wenn auch im sichtbaren Bereich!
        //if (self.x < 10) & (self.y < 10) {
        param.camera.mesh.fill(
            Polyline {
                points: self
                    .get_hex_point_vec(param.camera.cameraoffset_x, param.camera.cameraoffset_y),
            },
            color_by_tiletype(&mut self.tile_type),
        );
        //}
    }
}

// https://www.redblobgames.com/grids/hexagons/
impl Tile {
    pub fn randomize(&mut self) {
        self.tile_type = rand::random();
    }

    fn get_hex_point_vec(&mut self, x_offset: i16, y_offset: i16) -> Vec<Point> {
        let mut polypoints: Vec<Point> = Vec::new();
        for n in 0..6 {
            polypoints.push(self.pointy_hex_corner(n, x_offset, y_offset));
        }
        return polypoints;
    }

    fn pointy_hex_corner(&mut self, i: i16, x_offset: i16, y_offset: i16) -> Point {
        let angle_deg = 60 * i - 30;
        let angle_rad = std::f32::consts::PI / 180f32 * (angle_deg as f32);
        return Point::new(
            (self.x * self.width) as f32 //center
                + self.width as f32 * angle_rad.cos() / 3f32.sqrt() //size
                + x_offset as f32 // offset
                + self.y as f32 / 2f32 * self.width as f32 //* angle_rad.cos() / 3f32.sqrt() // row-offset
                + self.width as f32 / 2f32, // das erste tile soll vollständig sichtbar sein
            self.y as f32 * self.height as f32 * 0.75f32
                + self.height as f32 * angle_rad.sin() / 2f32
                + y_offset as f32
                + self.height as f32 / 2f32,
        );
    }

    pub fn get_center(&mut self, x_offset: i16, y_offset: i16) -> Point {
        return Point::new(
            self.x as f32 * self.width as f32 //center
                + x_offset as f32 // offset
                + self.y as f32 / 2f32 * self.width as f32 // row-offset
                + self.width as f32 / 2f32, // das erste tile soll vollständig sichtbar sein
            self.y as f32 * self.height as f32 * 0.75f32 // center
                + y_offset as f32 // offset
                + self.height as f32 / 2f32, // erstes tile
        );
    }
}
