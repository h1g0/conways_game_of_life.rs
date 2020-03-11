extern crate piston_window;
use piston_window::*;


const WINDOW_TITLE: &str = "LifeGame.rs";
const WINDOW_SIZE: Size = Size{width: 640.0, height: 480.0};

#[derive(Clone)]
struct Cell{
    alive: bool,
    alive_in_next_gen: bool,
}

struct Field{
    width: usize,
    height: usize,
    x_num: usize,
    y_num: usize,
    cell: Vec<Cell>,
    cell_size: Size,
}

impl Cell{
    fn set_next_gen_state(cell: &Vec<Cell>){
        unimplemented!();
    }
}

impl Field{
    fn init() -> Field{
        let xy_num = (64,48);
        let wh = (WINDOW_SIZE.width as usize, WINDOW_SIZE.height as usize);
        Field{
            width: wh.0,
            height: wh.1,
            x_num: xy_num.0,
            y_num: xy_num.1,
            cell: vec![
                    Cell{
                        alive: false,
                        alive_in_next_gen: false,
                    };
                    xy_num.0 * xy_num.1
                ],
            cell_size: Size{
                width: (wh.0 / xy_num.0) as f64,
                height: (wh.1 / xy_num.1) as f64,
            }
        }
    }

    fn draw_field(&self, window: &mut PistonWindow, args: &RenderArgs){
        unimplemented!();
    }

    fn apply_current_gen_state(){
        unimplemented!();
    }

    fn draw_cells(){
        unimplemented!();
    }

    fn set_next_gen_state(){
        unimplemented!();
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .samples(4)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            _c.draw_state.blend(draw_state::Blend::Add);
            //ここにゲームの処理何か書く
        });
    }
}
