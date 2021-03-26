use amethyst::{
    assets::{AssetLoaderSystemData},
    core::transform::{Transform},
    prelude::*,
    renderer::{Camera, light, Material, MaterialDefaults, Mesh, palette, shape,
        rendy::mesh::{Normal, MeshBuilder, Position, Tangent, TexCoord, },
    },
    window::ScreenDimensions,
};


/// A dummy game state that shows 3 sprites.
pub struct MyGameState;

impl SimpleState for MyGameState {

    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {

        // Determine actual screen size so initial camera uses the same aspect ratio as the screen.
        // Clone so we don't perform an immutable borrow, as that will stop us passing the world mutably to init_*
        // functions later.
        let screen_dimensions: ScreenDimensions = (*state_data.world.read_resource::<ScreenDimensions>()).clone();

        init_camera(state_data.world, screen_dimensions);
        init_sphere(state_data.world);
        init_cube(state_data.world);
        initialize_light(state_data.world);
    }
}

/// Creates a camera entity in the `world`.
fn init_camera(world: &mut World, screen_dimensions: ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world
        .create_entity()
        .with(Camera::standard_3d(screen_dimensions.width(), screen_dimensions.height()))  // width, height must be dimensions of window to avoid distortion
        .with(transform)
        .build();
}

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
