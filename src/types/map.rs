use super::tile::*;
use coffee::graphics::{Mesh, Frame};

pub struct Map
{
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(h : u16, w: u16) -> Self{
        let mut a = Vec::<Vec<Tile>>::with_capacity(h.into());
        for x in 0..h {
            let mut b = Vec::<Tile>::with_capacity(w.into());
            for y in 0..w {
                //let t = Tile::new(TileType::TtDirt, x, y);
                let tt: TileType = rand::random();
                let t = Tile::new(tt, x, y);
                b.push(t)
            }
            a.push(b);
        }

        Map {
            width : w,
            height : h,
            tiles: a,
        }
    }

    pub fn draw_map(&mut self, frame: &mut Frame, _offset_x :&i16, _offset_y : &i16) {
        let mut mesh = Mesh::new();
        for a in &mut self.tiles {
            for b in a{
                b.draw_tile(&mut mesh, _offset_x, _offset_y)
            }
        }
        mesh.draw(&mut frame.as_target());
    }
}