extern crate piston_window;
use piston_window::*;

mod lifegame;

const WINDOW_TITLE: &str = "LifeGame.rs";
const WINDOW_SIZE: Size = Size {
    width: 640.0,
    height: 480.0,
};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .samples(4)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.events.set_max_fps(10);
    window.events.set_ups(10);

    let mut field = lifegame::Field::new((64, 48), (640, 480));
    field.set_random_state_for_all_cels(0.3);

    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, _d| {
                    c.draw_state.blend(draw_state::Blend::Add);
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    field.draw_field(c, g);
                    field.update_state();
                    field.draw_cells(c, g);
                });
            }
            Event::Loop(Loop::Update(_)) => {
                field.set_next_gen_state();
            }
            _ => {}
        }
    }
}
