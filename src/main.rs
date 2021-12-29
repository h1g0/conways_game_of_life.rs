use bevy::prelude::*;
//mod lifegame;

const WINDOW_TITLE: &str = "Conway's Game of Life";
const WINDOW_SIZE: (f32,f32) = (640.0,480.0); 

fn main() {
    App::build()
    .insert_resource(WindowDescriptor {
        title: WINDOW_TITLE.to_string(),
        width: WINDOW_SIZE.0,
        height: WINDOW_SIZE.1,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_plugins(DefaultPlugins)
    .run();
}

