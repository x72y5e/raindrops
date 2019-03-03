use graphics::types::Color;
use graphics::{Context, Graphics};
use rand::Rng;


#[derive(Clone, Copy)]
pub struct RainDrop {
    pub pos: (f64, f64),
    spd: f64,
    h_spd: f64,
    pub accel: f64,
    pub h_accel: f64,
    pub colour: Color,
}

impl RainDrop {
    pub fn new(dims: [f64; 2], colour: Color, accel: f64, lateral: f64) -> RainDrop {
        let [xdim, _ydim] = dims;
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(0.0, xdim);
        let spd: f64 = rng.gen_range(150.0, 250.0);

        RainDrop {
            pos: (x, 0.0),
            spd,
            h_spd: 0.0,
            accel,
            h_accel: lateral,
            colour,
        }
    }

    pub fn update(&mut self, t: f64) {
        let (x, y) = self.pos;
        let next_x = x + t * self.h_spd;
        let next_y = y + t * self.spd;
        self.spd += t * self.accel;
        self.h_spd += t * self.h_accel;
        self.pos = (next_x, next_y);
    }

    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::Rectangle;
        let (x, y) = self.pos;
        let drop_rect = [x, y, 2.0, 2.0];
        Rectangle::new(self.colour)
            .draw(drop_rect, &c.draw_state, c.transform, g);
    }
}
