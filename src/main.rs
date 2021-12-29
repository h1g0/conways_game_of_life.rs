use bevy::prelude::*;
//mod lifegame;

const WINDOW_TITLE: &str = "Conway's Game of Life";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0,480.0); 

fn main() {
    App::build()
        .add_system(hello_world.system())
        .run();
}

fn hello_world() {
    println!("hello world!");
}
