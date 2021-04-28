use amethyst::{
    core::transform::{TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderToWindow, RenderPbr3D},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    window::DisplayConfig,
};

pub mod state;
pub mod system;
pub mod component;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let key_binding_path = assets_dir.join("key_binding.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(key_binding_path)?;


    let mut display_config = DisplayConfig::default();
    display_config.title = "My first Amethyst 3D program".to_string();
    display_config.dimensions = Some((1000, 800));


    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(system::CameraMoveSystem, "camera_move_system", &["input_system"])
        .with(system::SpinEntitySystem, "spin_entity_system", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.3, 0.7, 0.2, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderPbr3D::default()),
        )?;

    let mut game = Application::new(assets_dir, state::MyGameState::default(), game_data)?;
    game.run();

    Ok(())
}
