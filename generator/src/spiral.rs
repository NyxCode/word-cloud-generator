use rusttype::{Point, point};

pub struct Spiral {
    pub step: f32,
    pub index: f32,
}

impl Spiral {
    pub fn new(step: f32) -> Self {
        Spiral { step, index: 0.0 }
    }

    pub fn next_point(&mut self) -> Point<i32> {
        let x = self.index;
        self.index += self.step;
        point((x * x.cos()) as i32,
              (x * x.sin()) as i32)
    }
}