use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

// TODO Value to multiply movement amounts by so camera moves at the desired rate. Temporary until movement speed is
//      based on time between frames.
pub const MOVE_RATIO: f32 = 0.5;

pub struct MyMoveSystem;

impl<'a> System<'a> for MyMoveSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, camera, input): Self::SystemData) {

        for (_camera, transform) in (&camera, &mut transforms).join() {

            let mut x = 0.0;
            let mut z = 0.0;

            if let Some(v) = input.axis_value("vertical") {
                if v != 0.0 {
//                     println!("Movement detected on vertical axis, value is {}", v);
                    z = -v * MOVE_RATIO;     // Reducing "z" moves us forward, so negate value obtained from input axis
                }
            }

            if let Some(h) = input.axis_value("horizontal") {
                if h != 0.0 {
//                     println!("Movement detected on horizontal axis, value is {}", h);
                    x = h * MOVE_RATIO;
                }
            }

            transform.append_translation(Vector3::new(x, 0.0, z));

        }
    }
}
