use bevy::{core::FixedTimestep, prelude::*};
use wasm_bindgen::prelude::*;

use life::Field;
mod life;

const WINDOW_TITLE: &str = "Conway's Game of Life";
const WINDOW_SIZE: (f32, f32) = (640.0, 480.0);

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.insert_resource(WindowDescriptor {
        title: WINDOW_TITLE.to_string(),
        width: WINDOW_SIZE.0,
        height: WINDOW_SIZE.1,
        resizable: false,
        ..Default::default()
    });
    app.insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)));
    app.add_startup_system(setup.system());
    app.add_startup_system(Field::setup.system());
    app.add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(6.0 / 60.0))
            .with_system(Field::update.system()),
    );
    app.add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.run();
}
fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
