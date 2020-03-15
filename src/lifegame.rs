extern crate piston_window;
use piston_window::*;

#[derive(Clone,Copy)]
struct Cell{
    id: usize,
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

impl Field{
    pub fn new(xy_num: (usize,usize), wh: (usize,usize)) -> Field{
        let mut result:Field =
        Field{
            width: wh.0,
            height: wh.1,
            x_num: xy_num.0,
            y_num: xy_num.1,
            cell: vec![
                    Cell{
                        id: 0,
                        alive: false,
                        alive_in_next_gen: false,
                    };
                    xy_num.0 * xy_num.1
                ],
            cell_size: Size{
                width: (wh.0 / xy_num.0) as f64,
                height: (wh.1 / xy_num.1) as f64,
            }
        };
        for i in 0..result.cell.len(){
            result.cell[i].id = i;
        }
        return result;
    }

    pub fn draw_field(&self, window: &mut PistonWindow, args: &RenderArgs){
        unimplemented!();
    }

    pub fn update_state(&mut self){
        for c in &mut self.cell {
            c.alive = c.alive_in_next_gen;
        }
    }

    pub fn set_state(&mut self, id: usize, alive:bool){
        self.cell[id].alive = alive;
    }

    pub fn set_state_for_all_cells(&mut self, alive:Vec<bool>){
        // TODO: ここから
        for i in 0..self.cell.len(){
            if i < alive.len(){
                self.cell[i].alive = alive[i];
            }else{
                self.cell[i].alive = false;
            }
        }
    }

    pub fn draw_cells(){
        unimplemented!();
    }

    pub fn set_next_gen_state(){
        unimplemented!();
    }

    fn get_next_gen_state_for_one_cell(&self, id: usize)->bool{
        unimplemented!();
    }

}

#[cfg(test)]
mod tests {
    mod cell{
        #[test]
        fn cell_dies_for_no_neighbor() {
            let mut result;
            let live = vec![
                false,false,false,
                false,true,false,
                false,false,false,
            ];
            result = false;
            assert_eq!(result,false);

            let dead = vec![
                false,false,false,
                false,false,false,
                false,false,false,
            ];
            result = false;
            assert_eq!(result,false);
        }
        #[test]
        fn cell_dies_for_1_neighbor() {
            let mut result;
            let live = vec![
                false,true,false,
                false,true,false,
                false,false,false,
            ];
            result = false;
            assert_eq!(result,false);

            let dead = vec![
                false,true,false,
                false,false,false,
                false,false,false,
            ];
            result = false;
            assert_eq!(result,false);
        }
        #[test]
        fn cell_lives_for_2_neighbors() {
            let mut result;
            let live = vec![
                false,true,true,
                false,true,false,
                false,false,false,
            ];
            result = true;
            assert_eq!(result,true);
            let dead = vec![
                false,true,true,
                false,false,false,
                false,false,false,
            ];
            result = false;
            assert_eq!(result,false);
        }
        #[test]
        fn cell_lives_borns_for_3_neighbors() {
            let mut result;
            let live = vec![
                false,true,true,
                false,true,false,
                false,true,false,
            ];
            result = true;
            assert_eq!(result,true);
            let dead = vec![
                false,true,true,
                false,false,false,
                false,true,false,
            ];
            result = true;
            assert_eq!(result,true);
        }
        #[test]
        fn cell_dies_for_4_or_more_neighbors(){
            let mut result;
            let live = vec![
                false,true,true,
                false,true,false,
                false,true,true,
            ];
            result = false;
            assert_eq!(result,false);
            let dead = vec![
                false,true,true,
                false,false,false,
                false,true,true,
            ];
            result = false;
            assert_eq!(result,false);
        }
    }
}