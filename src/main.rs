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

mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    let mut display_config = DisplayConfig::default();
    display_config.title = "My first Amethyst 3D program".to_string();
    display_config.dimensions = Some((1000, 800));


    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.3, 0.7, 0.2, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderPbr3D::default()),
        )?;

    let mut game = Application::new(assets_dir, state::MyGameState, game_data)?;
    game.run();

    Ok(())
}
