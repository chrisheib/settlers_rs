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
            width: 30,
            height: 20,
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
                + self.y as f32 / 2f32 * self.width as f32 // row-offset
                + self.width as f32 / 2f32, // das erste tile soll vollständig sichtbar sein
            self.y as f32 * self.height as f32 * 0.75f32
                + self.height as f32 * angle_rad.sin() / 2f32
                + y_offset as f32
                + self.height as f32 / 2f32,
        );
    }

    pub fn PointToCoord(&mut self, x: f32, z: f32) -> (i16, i16) {
        let cx =
            x / self.width as f32 * (std::f32::consts::PI / 180f32 * (-30f32)).cos() / 3f32.sqrt();
        let cy = z / self.height as f32 * (std::f32::consts::PI / 180f32 * (-30f32)).sin() / 2f32;

        let fx = (-2f32 / 3f32) * cx;
        let fy = (1f32 / 3f32) * cx + (1f32 / 3f32.sqrt()) * cy;
        let fz = (1f32 / 3f32) * cx - (1f32 / 3f32.sqrt()) * cy;

        let a = (fx - fy).ceil();
        let b = (fy - fz).ceil();
        let c = (fz - fx).ceil();

        let x2 = ((a - c) / 3f32).round();
        let y2 = ((b - a) / 3f32).round();
        let z2 = ((c - b) / 3f32).round();
        let (rx, ry) = self.cube_to_axial(x2, y2, z2);
        return (rx as i16, ry as i16);
    }

    pub fn pixel_to_pointy_hex(&mut self, x: f32, y: f32) -> (i16, i16) {
        let q = (3f32.sqrt() / 3f32 * x as f32 - (1f32 / 3f32) * y as f32) / self.width as f32;
        let r = (2f32 / 3f32 * y as f32) / self.height as f32;
        let (rx, ry) = self.hex_round(q, r);
        return (rx as i16, ry as i16);
    }

    fn hex_round(&mut self, x: f32, y: f32) -> (f32, f32) {
        let (a, b, c) = self.axial_to_cube(x, y);
        let (d, e, f) = self.cube_round(a, b, c);
        return self.cube_to_axial(d, e, f);
    }

    fn cube_round(&mut self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        let mut rx = x.round();
        let mut ry = y.round();
        let mut rz = z.round();

        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        let z_diff = (rz - z).abs();

        if (x_diff > y_diff) & (x_diff > z_diff) {
            rx = -ry - rz
        } else if y_diff > z_diff {
            ry = -rx - rz
        } else {
            rz = -rx - ry
        }
        return (rx, ry, rz);
    }

    fn cube_to_axial(&mut self, x: f32, y: f32, z: f32) -> (f32, f32) {
        let q = x;
        let r = z;
        return (q, r);
    }

    fn axial_to_cube(&mut self, q: f32, r: f32) -> (f32, f32, f32) {
        let x = q;
        let z = r;
        let y = -x - z;
        return (x, y, z);
    }
}
