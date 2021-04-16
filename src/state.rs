use amethyst::{
    assets::{AssetLoaderSystemData},
    core::transform::{Transform},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, light, Material, MaterialDefaults, Mesh, palette, shape},
    renderer::rendy::mesh::{Normal, MeshBuilder, Position, Tangent, TexCoord},
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
    window::ScreenDimensions,
};

pub struct MyGameState {
    pub existing_screen_dimensions: ScreenDimensions,
}

// Maybe useful for animation:
// https://mtigley.dev/posts/sprite-animations-with-amethyst/
//

impl Default for MyGameState {
    fn default() -> Self {
        Self {existing_screen_dimensions: ScreenDimensions::new(0, 0, 1.0)}
    }
}


impl SimpleState for MyGameState {

    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {

        // Determine actual screen size so initial camera uses the same aspect ratio as the screen.
        // Clone so we don't perform an immutable borrow, as that will stop us passing the world mutably to init_*
        // functions later.
        let screen_dimensions: ScreenDimensions = (*state_data.world.read_resource::<ScreenDimensions>()).clone();
        self.existing_screen_dimensions = screen_dimensions.clone();

        init_camera(state_data.world, screen_dimensions);
        init_sphere(state_data.world);
        init_cube(state_data.world);
        initialize_light(state_data.world);
    }


    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
/*        let screen_dimensions = (*state_data.world.read_resource::<ScreenDimensions>()).clone();

        if (screen_dimensions.width(), screen_dimensions.height()) !=
            (self.existing_screen_dimensions.width(), self.existing_screen_dimensions.height()) {

            self.existing_screen_dimensions = screen_dimensions;
            println!("User has altered screen dimensions to {} x {}", self.existing_screen_dimensions.width(), self.existing_screen_dimensions.height());
            //init_camera(state_data.world, screen_dimensions);
           // println!("{:#?}", *state_data.world.read_resource::<Camera>());
        }*/

        Trans::None
    }


    fn handle_event(&mut self, mut _state_data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        amethyst::start_logger(Default::default());

        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                println!("Received window close event");
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::Escape) {
                println!("Escape key pressed");
                return Trans::Quit;
            }

            if let Some(event) = get_key(&event) {
                println!("Received key event: {:?}", event);
            }
        }

        Trans::None
    }

}

/// Create a camera entity in the world.
fn init_camera(world: &mut World, screen_dimensions: ScreenDimensions) {
    let (screen_width, screen_height) = (screen_dimensions.width(), screen_dimensions.height());

    let mut transform = Transform::default();
    transform.set_translation_xyz(screen_width / 2.0, screen_height / 2.0, 100.0);

    world
        .create_entity()
        .with(Camera::standard_3d(screen_width, screen_height))
        .with(CameraOrtho::new(
            CameraNormalizeMode::Contain,
            CameraOrthoWorldCoordinates {
                left: -screen_width / 2.0,
                right: screen_width / 2.0,
                bottom: -screen_height / 2.0,
                top: screen_height / 2.0,
                far: 2000.0,
                near: 0.125,
            }))
        .with(transform)
        .build();
}

/// Create a sphere entity and add it to the world.
fn init_sphere(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(3.0, 3.0, 0.0);

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            shape::Shape::Sphere(100, 100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
            loader.load_from_data(
                Material {
                    ..material_defaults
                },
                (),
            )
        },
    );

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .build();
}


/// Create a cube entity and add it to the world.
fn init_cube(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(-3.0, -2.0, 0.0);

    //let p = Position::from([0.1, 0.1, 0.1]);

    let cube = shape::Shape::Cube;
    let cube_meshbuilder: MeshBuilder = cube.generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None);
    let cube_meshdata = cube_meshbuilder.into();

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            cube_meshdata,
            (),
        )
    });

    /* // Compact version of above
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            shape::Shape::Cube
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });
    */

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
            loader.load_from_data(
                Material {
                    ..material_defaults
                },
                (),
            )
        },
    );

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .build();
}



fn initialize_light(world: &mut World) {
    let light: light::Light = light::PointLight {
        intensity: 10.0,
        color: palette::rgb::Rgb::new(1.0, 1.0, 1.0),
        ..light::PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world
        .create_entity()
        .with(light)
        .with(transform)
        .build();
}
