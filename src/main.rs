extern crate piston_window;
use piston_window::*;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("LifeGame.rs", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    let mut x = 0.0;
    let mut y = 0.0;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            x += 1.0;
            x %= 640.0;
            y += 1.0;
            y %= 480.0;
            rectangle([0.5, 1.0, 0.5, 0.5],
                [x, y, 10.0, 10.0],
                _c.transform, g);
        });
    }
}
