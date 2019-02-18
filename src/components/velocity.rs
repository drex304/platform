use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
