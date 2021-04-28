use amethyst::ecs::{Component, DenseVecStorage};


/// Each entity with this component will be spun (rotated around its center), by the `SpinEntitySystem` system.
// TODO Would prefer to use NullStorage, but results in errors about the `Default` trait not being satisfied.
pub struct SpinMe;

impl Component for SpinMe {
    type Storage = DenseVecStorage<Self>;
}
