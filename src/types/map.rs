use ndarray::prelude::*;

//pub mod self::tile;
use super::tile::Tile;

pub struct Map
{
    pub width: u16,
    pub height: u16,
    data: Vec<Vec<Tile>>,
    //tiles: Array2<tile::Tile>,
}

impl Map {
    pub fn new(h : u16, w: u16) -> Self{
        //let a = Array2::<tile::Tile::new();
        let mut a = Vec::<Vec<Tile>>::with_capacity(h.into());
        for i in 0..h {
            let b = Vec::<Tile>::with_capacity(w.into());
            for j in 0..w {
                //define tiles
            }
            a.push(b);
        }

        //let a = vec!
        Map {
            width : w,
            height : h,
            data: a,
        }
    }
}