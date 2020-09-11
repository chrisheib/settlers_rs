use super::tile::*;

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
