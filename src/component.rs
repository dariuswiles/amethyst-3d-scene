use amethyst::ecs::{Component, DenseVecStorage};


/// Each entity with this component will be spun (rotated around its center), by the `SpinEntitySystem` system.
/// The spin_rate is expressed in radians per second, i.e., a value of 2π is one complete rotation per second.
pub struct SpinMe {
    pub spin_rate: f32,
}

impl Component for SpinMe {
    type Storage = DenseVecStorage<Self>;
}
