use piston::input::GenericEvent;
use graphics::types::Color;
use graphics::{Context, Graphics};
use rayon::prelude::*;
use crate::drop::RainDrop;
use crate::rand::Rng;


pub struct RainDropController {
    pub drops: Vec<RainDrop>,
    cursor_pos: [f64; 2],
    dims: [f64; 2],
    bg_colour: Color,
    drop_colour: Color,
    accel: f64,
    lateral_acc: f64,
}

impl RainDropController {
    pub fn new(dims: [u32; 2]) -> RainDropController {
        let mut drops: Vec<RainDrop> = Vec::new();
        let dims = [dims[0] as f64, dims[1] as f64];
        let drop_colour = [0.1, 0.2, 0.5, 1.0];
        for _ in 0..10 {
            drops.push(RainDrop::new(dims, drop_colour, 9.81, 0.0));
        }

        RainDropController {
            drops: drops,
            cursor_pos: [0.0; 2],
            dims: dims,
            bg_colour: [0.2, 0.8, 0.7, 0.5],
            drop_colour: drop_colour,
            accel: 9.81,
            lateral_acc: 0.0,
        }
    }

    pub fn draw_background<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::Rectangle;
        let bg_rect = [0.0, 0.0, self.dims[0], self.dims[1]];
        Rectangle::new(self.bg_colour)
            .draw(bg_rect, &c.draw_state, c.transform, g);
    }

    fn cycle_colour(&mut self) {
        let mut rng = rand::thread_rng();
        self.drop_colour = [rng.gen(), rng.gen(), rng.gen(), rng.gen_range(0.4, 1.0)];
        let col = self.drop_colour;
        self.drops.iter_mut()
            .for_each(|x| x.colour = col);
    }

    pub fn event<E: GenericEvent>(&mut self, t: f64, e: &E) {
        use piston::input::{Button, Key}; //, MouseButton};
        let mut rng = rand::thread_rng();

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
            self.lateral_acc = (pos[0] - self.dims[0] / 2.0) * 0.2;
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space => {
                    self.cycle_colour();
                    println!("{:?}", self.cursor_pos);
                },
                Key::Return => {
                    let mut rng = rand::thread_rng();
                    self.bg_colour = [rng.gen(), rng.gen(), rng.gen(), rng.gen_range(0.0, 0.6)];
                },
                Key::Down => {
                    self.accel += 2.5;
                },
                Key::Up => {
                    self.accel -= 2.5;
                },
                Key::Right => {
                    self.lateral_acc += 2.5;
                },
                Key::Left => {
                    self.lateral_acc -= 2.5;
                },
                _ => {},
            }
        }
        self.drops.par_iter_mut()
            .for_each(|x| x.update(t));

        self.drops = self.drops.par_iter()
            .cloned()
            .filter(|x| x.pos.1 < self.dims[1])
            .collect();

        let prob: f32 = rng.gen();
        if prob > 0.95 {
            self.drops.push(RainDrop::new(self.dims,
                                          self.drop_colour,
                                          self.accel,
                                          self.lateral_acc));
        }
    }
}
