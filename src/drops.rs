extern crate ndarray;

use graphics::types::Color;
use graphics::{Context, Graphics};
use rand::Rng;
use ndarray::Array2;


pub struct Drops {
    pub spd: Array2<f64>,
    pub accel: Array2<f64>,
}

impl Drops {
    fn new(dims: [u32; 2]) -> Drops {
        let [xdim, ydim] = dims;
        let mut rng = rand::thread_rng();
        let mut grid = Array2::zeros((xdim, ydim));
        for _ in 0..10 {
            let y: f64 = rng.gen_range(0, xdim);
            let spd: f64 = rng.gen_range(100.0, 128.0);
            grid[[0, y]] = spd;
        }

        RainDrop {
            spd: grid,
            accel: Array2::ones * 9.81,
        }
    }

    fn update(&mut self, t: f64) {

        let new_positions: Array2<f64> = self.spd +
    }
}
