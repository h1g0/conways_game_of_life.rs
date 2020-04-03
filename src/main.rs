extern crate piston_window;
use piston_window::*;

mod lifegame;

const WINDOW_TITLE: &str = "LifeGame.rs";
const WINDOW_SIZE: Size = Size{width: 640.0, height: 480.0};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .samples(4)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    while let Some(e) = window.next() {
        match e{
            Event::Loop(Loop::Render(_))=>{
                //レンダリング処理
                window.draw_2d(&e, |_c, g, _d| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    _c.draw_state.blend(draw_state::Blend::Add);
                    //ここにゲームの処理何か書く
                });
            }
            Event::Loop(Loop::Update(_))=>{
                //アップデート処理
            }
            _=>{}
        }

    }
}