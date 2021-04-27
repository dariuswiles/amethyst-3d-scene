#[allow(unused_imports)]
use amethyst::{
    core::{SystemDesc},
    core::transform::{Transform, TransformBundle},
    core::math::Vector3,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputBundle, InputHandler, StringBindings},
    prelude::*,
    renderer::{
        Camera,
        plugins::{RenderToWindow, RenderPbr3D},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    window::DisplayConfig,
};


pub struct MyMoveSystem;

impl<'a> System<'a> for MyMoveSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, camera, input): Self::SystemData) {

        for (camera, transform) in (&camera, &mut transforms).join() {

//             let i: &InputHandler<StringBindings> = &*input;

            let mut x = 0.0;
            let mut z = 0.0;


            if let Some(v) = input.axis_value("vertical") {
                if v != 0.0 {
                    println!("Movement detected on vertical axis, value is {}", v);
                    z = v;
                }
            }

            if let Some(h) = input.axis_value("horizontal") {
                if h != 0.0 {
                    println!("Movement detected on horizontal axis, value is {}", h);
                    x = h;
                }
            }

            transform.append_translation(Vector3::new(x, 0.0, z));

        }
    }
}
