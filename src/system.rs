use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::math::Vector3,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

// Value to multiply movement amounts by so camera moves at the desired rate.
pub const VELOCITY: f32 = 6.0;

pub struct MyMoveSystem;

impl<'a> System<'a> for MyMoveSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut transforms, camera, input, time): Self::SystemData) {

        for (_camera, transform) in (&camera, &mut transforms).join() {

            let mut x = 0.0;
            let mut z = 0.0;

            if let Some(v) = input.axis_value("vertical") {
                if v != 0.0 {
                    // Reducing "z" moves us forward, so negate value obtained from input axis
                    z = -v * VELOCITY * time.delta_seconds();
                }
            }

            if let Some(h) = input.axis_value("horizontal") {
                if h != 0.0 {
                    x = h * VELOCITY * time.delta_seconds();
                }
            }

            transform.append_translation(Vector3::new(x, 0.0, z));

        }
    }
}
