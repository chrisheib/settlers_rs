use coffee::graphics::Point;

use super::hextile::*;

pub struct Map {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(h: u16, w: u16) -> Self {
        let mut a = Vec::<Vec<Tile>>::with_capacity(h.into());
        for x in 0..h {
            let mut b = Vec::<Tile>::with_capacity(w.into());
            for y in 0..w {
                let tt: TileType = rand::random();
                let t = Tile::new(tt, x, y);
                b.push(t)
            }
            a.push(b);
        }

        Map {
            width: w,
            height: h,
            tiles: a,
        }
    }

    pub fn get_tile_from_coords(
        &mut self,
        pointa: Point,
        x_offset: i16,
        y_offset: i16,
    ) -> &mut Tile {
        let x = pointa.coords.x;
        let y = pointa.coords.y;
        let width = self.tiles[0][0].width as f32;
        let height = self.tiles[0][0].height as f32;
        let mut schaetz_x = (x - x_offset as f32) / width // breite
            - (((y - y_offset as f32) / (height * 0.75f32)) * (0.5f32 * width)) / width // row-offset
            - 0.5f32; // Schätz-Offset

        if schaetz_x <= -1f32 {
            schaetz_x = -0.99f32; // Vorher irgendwie abfangen...
        }
        if schaetz_x >= self.width as f32 - 0.5f32 {
            schaetz_x = self.width as f32 - 0.51f32;
        }

        let mut schaetz_y = (y - y_offset as f32) / (height * 0.75f32) // höhe
            - 0.5f32; // Schätz-Offset

        if schaetz_y <= -1f32 {
            schaetz_y = -0.99f32; // Vorher irgendwie abfangen...
        }
        if schaetz_y >= self.height as f32 - 0.5f32 {
            schaetz_y = self.height as f32 - 0.51f32;
        }

        println!("Click: {}, {}", pointa.coords.x, pointa.coords.y);
        println!("Schätzwerte: {} {}", schaetz_x, schaetz_y);

        // 4 hexagons:
        let i1 = schaetz_x.floor();
        let j1 = schaetz_y.floor();

        let dist1;
        {
            if self.valid_hex(i1 as i16, j1 as i16) {
                let hex1 = &mut self.tiles[i1 as i16 as usize][j1 as i16 as usize];
                let pointb = hex1.get_center(x_offset, y_offset);
                dist1 = self.distance_between_two_points(&pointa, &pointb);
            } else {
                dist1 = f32::INFINITY;
            }
        }

        let i2 = schaetz_x.floor();
        let j2 = schaetz_y.ceil();
        let dist2;
        {
            if self.valid_hex(i2 as i16, j2 as i16) {
                let hex2 = &mut self.tiles[i2 as i16 as usize][j2 as i16 as usize];
                let pointb = hex2.get_center(x_offset, y_offset);
                dist2 = self.distance_between_two_points(&pointa, &pointb);
            } else {
                dist2 = f32::INFINITY;
            }
        }

        let i3 = schaetz_x.ceil();
        let j3 = schaetz_y.floor();
        let dist3;
        {
            if self.valid_hex(i3 as i16, j3 as i16) {
                let hex3 = &mut self.tiles[i3 as i16 as usize][j3 as i16 as usize];
                let pointb = hex3.get_center(x_offset, y_offset);
                dist3 = self.distance_between_two_points(&pointa, &pointb);
            } else {
                dist3 = f32::INFINITY;
            }
        }

        let i4 = schaetz_x.ceil();
        let j4 = schaetz_y.ceil();
        let dist4;
        {
            if self.valid_hex(i4 as i16, j4 as i16) {
                let hex4 = &mut self.tiles[i4 as i16 as usize][j4 as i16 as usize];
                let pointb = hex4.get_center(x_offset, y_offset);
                dist4 = self.distance_between_two_points(&pointa, &pointb);
            } else {
                dist4 = f32::INFINITY;
            }
        }

        let resulthex;

        if (dist1 <= dist2) & (dist1 <= dist3) & (dist1 <= dist4) {
            resulthex = &mut self.tiles[i1 as i16 as usize][j1 as i16 as usize];
        } else if (dist2 <= dist1) & (dist2 <= dist3) & (dist2 <= dist4) {
            resulthex = &mut self.tiles[i2 as i16 as usize][j2 as i16 as usize];
        } else if (dist3 <= dist1) & (dist3 <= dist2) & (dist3 <= dist4) {
            resulthex = &mut self.tiles[i3 as i16 as usize][j3 as i16 as usize];
        } else {
            resulthex = &mut self.tiles[i4 as i16 as usize][j4 as i16 as usize];
        }

        println!("Distanzen: {}, {}, {}, {}", dist1, dist2, dist3, dist4);
        println!(
            "Center Winner: {}, {}",
            resulthex.get_center(x_offset, y_offset).coords.x,
            resulthex.get_center(x_offset, y_offset).coords.y
        );
        println!();

        return resulthex;
    }

    fn distance_between_two_points(
        &mut self,
        point_a: &coffee::graphics::Point,
        point_b: &coffee::graphics::Point,
    ) -> f32 {
        let y_factor = 30f32 / 20f32;

        let x1 = point_a.coords.x as f32;
        let x2 = point_b.coords.x as f32;
        let part1 = (x1 - x2).powi(2i32);

        let y1 = point_a.coords.y as f32;
        let y2 = point_b.coords.y as f32;
        let part2 = (y1 * y_factor - y2 * y_factor).powi(2i32);

        (part1 + (part2)).sqrt()
    }

    fn valid_hex(&mut self, x: i16, y: i16) -> bool {
        (x < self.width as i16) & (y < self.height as i16) & (x >= 0i16) & (y >= 0i16)
    }
}

impl crate::Drawable for Map {
    fn draw(&mut self, param: &mut crate::DrawParameter) {
        for a in &mut self.tiles {
            for b in a {
                b.draw(param)
            }
        }
    }
}
