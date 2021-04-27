#[allow(unused_imports)]
use amethyst::{
    core::{SystemDesc},
    core::transform::{Transform, TransformBundle},
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


#[derive(SystemDesc)]
pub struct MyMoveSystem;

impl<'a> System<'a> for MyMoveSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        Read<'a, InputHandler<StringBindings>>,
    );

//     fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
    fn run(&mut self, (mut transforms, camera, input): Self::SystemData) {

        for (camera, transform) in (&camera, &mut transforms).join() {

            let i: &InputHandler<StringBindings> = &*input;

            if let Some(v) = i.axis_value("vertical") {
                if v != 0.0 {
                    println!("Movement detected on vertical axis, value is {}", v);
                }
            }

            if let Some(v) = i.axis_value("horizontal") {
                if v != 0.0 {
                    println!("Movement detected on horizontal axis, value is {}", v);
                }
            }
        }
    }
}
