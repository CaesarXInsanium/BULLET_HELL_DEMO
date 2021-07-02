use amethyst::{Application, core::TransformBundle, renderer::{RenderFlat2D, RenderToWindow}, ui::RenderUi, utils::application_root_dir};
pub use amethyst::{
    renderer::{RenderingBundle, types::{DefaultBackend}},
    input::{InputBundle, StringBindings},
    utils::app_root_dir,    
};
pub mod state;
use state::GameplayState;

fn run() -> amethyst::Result<String>{
    amethyst::start_logger(Default::default());
    let root_dir = application_root_dir()?;
    let display_config = root_dir.join("config/display_config.ron");
    let assets_path = root_dir.join("assets");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(root_dir.join("config/bindings.ron"))?;

    //start of the show
    let game_data = amethyst::GameDataBuilder::default();

    let mut game =
        Application::new(assets_path, GameplayState::default(), game_data)?;

    game.run();
    Ok("Sucess".to_string())
    
}
fn main() -> amethyst::Result<()> {
    //running game
    amethyst::start_logger(Default::default());
    let root_dir = application_root_dir()?;
    let display_config = root_dir.join("config/display_config.ron");
    let assets_path = root_dir.join("assets");
    //let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(root_dir.join("config/bindings.ron"))?;

    //star of the show
    let game_data = amethyst::GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config)?
                    .with_clear([0.0, 0.0, 0.0, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
        )?;

    let mut game =
        Application::new(assets_path, GameplayState::default(), game_data)?;

    game.run();
    Ok(())
}
