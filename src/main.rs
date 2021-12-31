use bevy::{core::FixedTimestep, prelude::*};
use lifegame::Field;
mod lifegame;

const WINDOW_TITLE: &str = "Conway's Game of Life";
const WINDOW_SIZE: (f32, f32) = (640.0, 480.0);

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: WINDOW_TITLE.to_string(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_startup_system(setup.system())
        .add_startup_system(Field::setup.system())
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(6.0/60.0))
                .with_system(Field::update.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
