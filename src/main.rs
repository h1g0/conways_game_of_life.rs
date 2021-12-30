use bevy::prelude::*;
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
        .add_startup_system(setup.system())
        .add_startup_system(Field::setup.system())
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(Field::update_field.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(
            SpriteBundle {
                material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                sprite: Sprite::new(Vec2::new(WINDOW_SIZE.0, WINDOW_SIZE.1)),
                ..Default::default()
            }
        )
        .insert(lifegame::Field::new((64, 48), (640, 480)));
}
