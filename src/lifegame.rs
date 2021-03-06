extern crate piston_window;
use piston_window::*;

extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
struct Cell {
    id: usize,
    alive: bool,
    alive_in_next_gen: bool,
}
enum Neighbor {
    UpperLeft,
    UpperMiddle,
    UpperRight,
    Left,
    Right,
    LowerLeft,
    LowerMiddle,
    LowerRight,
}

pub struct Field {
    width: usize,
    height: usize,
    x_num: usize,
    y_num: usize,
    cell: Vec<Cell>,
    cell_size: Size,
}

impl Field {
    pub fn new(xy_num: (usize, usize), wh: (usize, usize)) -> Field {
        let mut result: Field = Field {
            width: wh.0,
            height: wh.1,
            x_num: xy_num.0,
            y_num: xy_num.1,
            cell: vec![
                Cell {
                    id: 0,
                    alive: false,
                    alive_in_next_gen: false,
                };
                xy_num.0 * xy_num.1
            ],
            cell_size: Size {
                width: (wh.0 / xy_num.0) as f64,
                height: (wh.1 / xy_num.1) as f64,
            },
        };
        for i in 0..result.cell.len() {
            result.cell[i].id = i;
        }
        return result;
    }

    pub fn draw_field(&self, c: Context, g: &mut G2d) {
        for i in 0..=self.x_num {
            line(
                [0.0, 0.3, 0.0, 0.5],
                0.5,
                [
                    self.cell_size.width * i as f64,
                    0.0,
                    self.cell_size.width * i as f64,
                    self.height as f64,
                ],
                c.transform,
                g,
            );
        }
        for i in 0..=self.y_num {
            line(
                [0.0, 0.3, 0.0, 0.5],
                0.5,
                [
                    0.0,
                    self.cell_size.height * i as f64,
                    self.width as f64,
                    self.cell_size.width * i as f64,
                ],
                c.transform,
                g,
            );
        }
    }

    pub fn update_state(&mut self) {
        for c in &mut self.cell {
            c.alive = c.alive_in_next_gen;
        }
    }

    pub fn set_state(&mut self, id: usize, alive: bool) {
        self.cell[id].alive = alive;
    }
    pub fn get_state(&self, id: usize) -> bool {
        if id >= self.cell.len() {
            panic!("The cell[{}] is not exit!", id);
        }
        return self.cell[id].alive;
    }

    fn get_neighbor_id(&self, id: usize, neighbor: Neighbor) -> Option<usize> {
        if id >= self.cell.len() {
            return None;
        }
        let left = if id % self.x_num == 0 { false } else { true };
        let upper = if (id as isize) - (self.x_num as isize) < 0 {
            false
        } else {
            true
        };
        let right = if id % self.x_num == self.x_num - 1 {
            false
        } else {
            true
        };
        let lower = if id + self.x_num >= self.cell.len() {
            false
        } else {
            true
        };
        match neighbor {
            Neighbor::UpperLeft => {
                if upper && left {
                    Some(id - self.x_num - 1)
                } else {
                    None
                }
            }
            Neighbor::UpperMiddle => {
                if upper {
                    Some(id - self.x_num)
                } else {
                    None
                }
            }
            Neighbor::UpperRight => {
                if upper && right {
                    Some(id - self.x_num + 1)
                } else {
                    None
                }
            }
            Neighbor::Left => {
                if left {
                    Some(id - 1)
                } else {
                    None
                }
            }
            Neighbor::Right => {
                if right {
                    Some(id + 1)
                } else {
                    None
                }
            }
            Neighbor::LowerLeft => {
                if lower && left {
                    Some(id + self.x_num - 1)
                } else {
                    None
                }
            }
            Neighbor::LowerMiddle => {
                if lower {
                    Some(id + self.x_num)
                } else {
                    None
                }
            }
            Neighbor::LowerRight => {
                if lower && right {
                    Some(id + self.x_num + 1)
                } else {
                    None
                }
            }
        }
    }
    fn get_neighbor_state(&self, id: usize, neighbor: Neighbor) -> bool {
        if let Some(neighbor_id) = self.get_neighbor_id(id, neighbor) {
            return self.get_state(neighbor_id);
        } else {
            return false;
        }
    }

    #[allow(dead_code)]
    pub fn set_state_for_all_cells(&mut self, alive: Vec<bool>) {
        for i in 0..self.cell.len() {
            if i < alive.len() {
                self.cell[i].alive = alive[i];
            } else {
                self.cell[i].alive = false;
            }
        }
        self.set_next_gen_state();
    }

    pub fn set_random_state_for_all_cels(&mut self, alive_probability: f64) {
        let mut rng = thread_rng();
        for i in 0..self.cell.len() {
            self.set_state(i, rng.gen_bool(alive_probability));
        }
        self.set_next_gen_state();
    }

    pub fn draw_cells(&self, c: Context, g: &mut G2d) {
        for i in 0..self.cell.len() {
            if self.get_state(i) {
                let x = i % self.x_num;
                let y = (i - x) / self.x_num;
                let square = rectangle::square(
                    self.cell_size.width * x as f64,
                    self.cell_size.height * y as f64,
                    self.cell_size.width,
                );
                let color = [0.1, 0.6, 0.0, 1.5];
                rectangle(color, square, c.transform, g);
            }
        }
    }

    pub fn set_next_gen_state(&mut self) {
        for i in 0..self.cell.len() {
            self.cell[i].alive_in_next_gen = self.get_next_gen_state_for_one_cell(i);
        }
    }

    fn get_next_gen_state_for_one_cell(&self, id: usize) -> bool {
        let alive_neighbor =
            0 + if self.get_neighbor_state(id, Neighbor::UpperLeft) {
                1
            } else {
                0
            } + if self.get_neighbor_state(id, Neighbor::UpperMiddle) {
                1
            } else {
                0
            } + if self.get_neighbor_state(id, Neighbor::UpperRight) {
                1
            } else {
                0
            } + if self.get_neighbor_state(id, Neighbor::Left) {
                1
            } else {
                0
            } + if self.get_neighbor_state(id, Neighbor::Right) {
                1
            } else {
                0
            } + if self.get_neighbor_state(id, Neighbor::LowerLeft) {
                1
            } else {
                0
            } + if self.get_neighbor_state(id, Neighbor::LowerMiddle) {
                1
            } else {
                0
            } + if self.get_neighbor_state(id, Neighbor::LowerRight) {
                1
            } else {
                0
            };

        if self.get_state(id) {
            match alive_neighbor {
                2 | 3 => true,
                _ => false,
            }
        } else {
            match alive_neighbor {
                3 => true,
                _ => false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbor_id_test() {
        let field = Field::new((3, 3), (8, 8));
        //012
        //345
        //678
        assert_eq!(field.get_neighbor_id(0, Neighbor::UpperLeft), None);
        assert_eq!(field.get_neighbor_id(0, Neighbor::UpperMiddle), None);
        assert_eq!(field.get_neighbor_id(0, Neighbor::UpperRight), None);
        assert_eq!(field.get_neighbor_id(0, Neighbor::Left), None);
        assert_eq!(field.get_neighbor_id(0, Neighbor::Right), Some(1));
        assert_eq!(field.get_neighbor_id(0, Neighbor::LowerLeft), None);
        assert_eq!(field.get_neighbor_id(0, Neighbor::LowerMiddle), Some(3));
        assert_eq!(field.get_neighbor_id(0, Neighbor::LowerRight), Some(4));

        assert_eq!(field.get_neighbor_id(1, Neighbor::UpperLeft), None);
        assert_eq!(field.get_neighbor_id(1, Neighbor::UpperMiddle), None);
        assert_eq!(field.get_neighbor_id(1, Neighbor::UpperRight), None);
        assert_eq!(field.get_neighbor_id(1, Neighbor::Left), Some(0));
        assert_eq!(field.get_neighbor_id(1, Neighbor::Right), Some(2));
        assert_eq!(field.get_neighbor_id(1, Neighbor::LowerLeft), Some(3));
        assert_eq!(field.get_neighbor_id(1, Neighbor::LowerMiddle), Some(4));
        assert_eq!(field.get_neighbor_id(1, Neighbor::LowerRight), Some(5));

        assert_eq!(field.get_neighbor_id(2, Neighbor::UpperLeft), None);
        assert_eq!(field.get_neighbor_id(2, Neighbor::UpperMiddle), None);
        assert_eq!(field.get_neighbor_id(2, Neighbor::UpperRight), None);
        assert_eq!(field.get_neighbor_id(2, Neighbor::Left), Some(1));
        assert_eq!(field.get_neighbor_id(2, Neighbor::Right), None);
        assert_eq!(field.get_neighbor_id(2, Neighbor::LowerLeft), Some(4));
        assert_eq!(field.get_neighbor_id(2, Neighbor::LowerMiddle), Some(5));
        assert_eq!(field.get_neighbor_id(2, Neighbor::LowerRight), None);

        assert_eq!(field.get_neighbor_id(3, Neighbor::UpperLeft), None);
        assert_eq!(field.get_neighbor_id(3, Neighbor::UpperMiddle), Some(0));
        assert_eq!(field.get_neighbor_id(3, Neighbor::UpperRight), Some(1));
        assert_eq!(field.get_neighbor_id(3, Neighbor::Left), None);
        assert_eq!(field.get_neighbor_id(3, Neighbor::Right), Some(4));
        assert_eq!(field.get_neighbor_id(3, Neighbor::LowerLeft), None);
        assert_eq!(field.get_neighbor_id(3, Neighbor::LowerMiddle), Some(6));
        assert_eq!(field.get_neighbor_id(3, Neighbor::LowerRight), Some(7));

        assert_eq!(field.get_neighbor_id(4, Neighbor::UpperLeft), Some(0));
        assert_eq!(field.get_neighbor_id(4, Neighbor::UpperMiddle), Some(1));
        assert_eq!(field.get_neighbor_id(4, Neighbor::UpperRight), Some(2));
        assert_eq!(field.get_neighbor_id(4, Neighbor::Left), Some(3));
        assert_eq!(field.get_neighbor_id(4, Neighbor::Right), Some(5));
        assert_eq!(field.get_neighbor_id(4, Neighbor::LowerLeft), Some(6));
        assert_eq!(field.get_neighbor_id(4, Neighbor::LowerMiddle), Some(7));
        assert_eq!(field.get_neighbor_id(4, Neighbor::LowerRight), Some(8));

        assert_eq!(field.get_neighbor_id(5, Neighbor::UpperLeft), Some(1));
        assert_eq!(field.get_neighbor_id(5, Neighbor::UpperMiddle), Some(2));
        assert_eq!(field.get_neighbor_id(5, Neighbor::UpperRight), None);
        assert_eq!(field.get_neighbor_id(5, Neighbor::Left), Some(4));
        assert_eq!(field.get_neighbor_id(5, Neighbor::Right), None);
        assert_eq!(field.get_neighbor_id(5, Neighbor::LowerLeft), Some(7));
        assert_eq!(field.get_neighbor_id(5, Neighbor::LowerMiddle), Some(8));
        assert_eq!(field.get_neighbor_id(5, Neighbor::LowerRight), None);

        assert_eq!(field.get_neighbor_id(6, Neighbor::UpperLeft), None);
        assert_eq!(field.get_neighbor_id(6, Neighbor::UpperMiddle), Some(3));
        assert_eq!(field.get_neighbor_id(6, Neighbor::UpperRight), Some(4));
        assert_eq!(field.get_neighbor_id(6, Neighbor::Left), None);
        assert_eq!(field.get_neighbor_id(6, Neighbor::Right), Some(7));
        assert_eq!(field.get_neighbor_id(6, Neighbor::LowerLeft), None);
        assert_eq!(field.get_neighbor_id(6, Neighbor::LowerMiddle), None);
        assert_eq!(field.get_neighbor_id(6, Neighbor::LowerRight), None);

        assert_eq!(field.get_neighbor_id(7, Neighbor::UpperLeft), Some(3));
        assert_eq!(field.get_neighbor_id(7, Neighbor::UpperMiddle), Some(4));
        assert_eq!(field.get_neighbor_id(7, Neighbor::UpperRight), Some(5));
        assert_eq!(field.get_neighbor_id(7, Neighbor::Left), Some(6));
        assert_eq!(field.get_neighbor_id(7, Neighbor::Right), Some(8));
        assert_eq!(field.get_neighbor_id(7, Neighbor::LowerLeft), None);
        assert_eq!(field.get_neighbor_id(7, Neighbor::LowerMiddle), None);
        assert_eq!(field.get_neighbor_id(7, Neighbor::LowerRight), None);

        assert_eq!(field.get_neighbor_id(8, Neighbor::UpperLeft), Some(4));
        assert_eq!(field.get_neighbor_id(8, Neighbor::UpperMiddle), Some(5));
        assert_eq!(field.get_neighbor_id(8, Neighbor::UpperRight), None);
        assert_eq!(field.get_neighbor_id(8, Neighbor::Left), Some(7));
        assert_eq!(field.get_neighbor_id(8, Neighbor::Right), None);
        assert_eq!(field.get_neighbor_id(8, Neighbor::LowerLeft), None);
        assert_eq!(field.get_neighbor_id(8, Neighbor::LowerMiddle), None);
        assert_eq!(field.get_neighbor_id(8, Neighbor::LowerRight), None);
    }

    #[test]
    fn cell_dies_for_no_neighbor() {
        let mut field = Field::new((3, 3), (8, 8));
        let mut result;
        let live = vec![false, false, false, false, true, false, false, false, false];
        field.set_state_for_all_cells(live);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, false);

        let dead = vec![
            false, false, false, false, false, false, false, false, false,
        ];
        field.set_state_for_all_cells(dead);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, false);
    }
    #[test]
    fn cell_dies_for_1_neighbor() {
        let mut field = Field::new((3, 3), (8, 8));
        let mut result;
        let live = vec![false, true, false, false, true, false, false, false, false];
        field.set_state_for_all_cells(live);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, false);

        let dead = vec![false, true, false, false, false, false, false, false, false];
        field.set_state_for_all_cells(dead);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, false);
    }
    #[test]
    fn cell_lives_for_2_neighbors() {
        let mut field = Field::new((3, 3), (8, 8));
        let mut result;
        let live = vec![false, true, true, false, true, false, false, false, false];
        field.set_state_for_all_cells(live);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, true);
        let dead = vec![false, true, true, false, false, false, false, false, false];
        field.set_state_for_all_cells(dead);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, false);
    }
    #[test]
    fn cell_lives_borns_for_3_neighbors() {
        let mut field = Field::new((3, 3), (8, 8));
        let mut result;
        let live = vec![false, true, true, false, true, false, false, true, false];
        field.set_state_for_all_cells(live);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, true);
        let dead = vec![false, true, true, false, false, false, false, true, false];
        field.set_state_for_all_cells(dead);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, true);
    }
    #[test]
    fn cell_dies_for_4_or_more_neighbors() {
        let mut field = Field::new((3, 3), (8, 8));
        let mut result;
        let live = vec![false, true, true, false, true, false, false, true, true];
        field.set_state_for_all_cells(live);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, false);
        let dead = vec![false, true, true, false, false, false, false, true, true];
        field.set_state_for_all_cells(dead);
        result = field.get_next_gen_state_for_one_cell(4);
        assert_eq!(result, false);
    }
}
