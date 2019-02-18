use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Gravity;

impl Component for Gravity {
    type Storage = DenseVecStorage<Self>;
}
