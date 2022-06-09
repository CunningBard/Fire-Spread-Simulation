mod essential_functions;

use macroquad::prelude::*;
use crate::essential_functions::{rand_item_index, rand_prob, rand_prob_, rand_range};

const BURN_SURROUNDING_PROBABILITY: i32 = 30;
const BURN_LIFETIME: u8 = 5;

fn surrounding_position(pos: &Position, max_x: i32, min_x: i32, max_y: i32, min_y: i32) -> Vec<Position>{
    let minus_positions = vec![
    vec![-1, -1], vec![ 0, -1], vec![ 1, -1],
    vec![-1,  0],               vec![ 1,  0],
    vec![-1,  1], vec![ 0,  1], vec![ 1,  1]
];

    let mut positions = vec![];
    for position in &minus_positions {
        let x = pos.x + position[0];
        let y = pos.y + position[1];
        if x >= min_x && max_x >= x && y >= min_y && max_y >= y {
            positions.push(Position{x, y})
        }
    }
    positions
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Position
{
    x: i32,
    y: i32
}

impl Position
{
    fn position(x: i32, y: i32) -> Position{
        Position{
            x,
            y
        }
    }
}

#[derive(PartialEq, Debug)]
struct BurnablePoint
{
    is_burning: bool,
    burnt: bool,
    burning_level: u8,
    position: Position
}

impl BurnablePoint
{
    fn burnable_point(position: Position )-> BurnablePoint{
        BurnablePoint {
            is_burning: false,
            burnt: false,
            burning_level: 0,
            position
        }
    }
    fn burn(&mut self) -> bool {
        if self.is_burning {
            if self.burning_level > BURN_LIFETIME {
                self.is_burning = false;
                self.burning_level = 0;
                self.burnt = true;
            } else {
                self.burning_level += 1;
                return rand_prob(BURN_SURROUNDING_PROBABILITY);
            }
        } else {
            self.is_burning = true;
            self.burning_level += 1;
        }
        false
    }
}

#[derive(Debug)]
struct Grid
{
    size_x: i32,
    size_y: i32,
    grid: Vec<Vec<BurnablePoint>>,
    burning_positions: Vec<Position>,
    burnt_positions: Vec<Position>
}

impl Grid {
    fn grid(size_x: i32, size_y: i32) -> Grid{
        let mut grid = vec![];
        for y in 0..size_y {
            let mut y_axis = vec![];
            for x in 0..size_x {
                y_axis.push(BurnablePoint::burnable_point(Position::position(x, y)))
            }
            grid.push(y_axis);
        }
        Grid {
            grid,
            size_x,
            size_y,
            burning_positions: vec![],
            burnt_positions: vec![]
        }
    }
    fn random_burn(&mut self){
        let rand_y = rand_range(0, 100);
        let rand_x = rand_range(0, 100);
        self.grid[rand_y as usize][rand_x as usize].burn();
        self.burning_positions.push(Position{ x: rand_x, y: rand_y })
    }

    fn handle(&mut self){
        let mut to_burn: Vec<Position> = vec![];
        let mut remove = vec![];
        let mut remove_pos = vec![];
        let mut ind = -1;
        for pos in &self.burning_positions {
            ind += 1;
            let bp: &mut BurnablePoint = &mut self.grid[pos.y as usize][pos.x as usize];
            if bp.is_burning{
                if bp.burn(){
                    to_burn.push(pos.clone());
                }
            } else {
                remove.push(ind as usize);
                remove_pos.push(pos.clone());
            }
        }
        remove.reverse();
        for to_remove in remove {
            self.burning_positions.remove(to_remove);
        }

        for pos in remove_pos{
            self.burnt_positions.push(pos);
        }
        // add remove thing from burning pos
        for burn_now in &to_burn {
            let mut points = surrounding_position(burn_now, self.size_x - 1, 0, self.size_y - 1, 0);
            let mut ind = -1;
            let mut to_remove = vec![];
            for point in &points {
                ind += 1;
                for point_ in &self.burning_positions {
                    if point == point_{
                        to_remove.push(ind);
                    }
                }
                for point_ in &self.burnt_positions {
                    if point == point_{
                        to_remove.push(ind);
                    }
                }
            }
            to_remove.reverse();
            for remove_point in to_remove {
                &points.remove(remove_point as usize);
            }
            if points.len() > 0 {
                let burn_point = points[rand_item_index(points.clone())];
                self.grid[burn_point.y as usize][burn_point.x as usize].burn();
                self.burning_positions.push(burn_point);
            }
        }
    }
}


#[macroquad::main("BasicShapes")]
async fn main() {
    let mut g = Grid::grid(100, 100);
    g.random_burn();
    loop {
        clear_background(WHITE);
        g.handle();
        for y in &g.grid {
            for point in y {
                if point.is_burning {
                    draw_rectangle(point.position.x as f32, point.position.y as f32, 1.0, 1.0, RED);
                } else if point.burnt{
                    draw_rectangle(point.position.x as f32, point.position.y as f32, 1.0, 1.0, BLACK);
                } else {
                    draw_rectangle(point.position.x as f32, point.position.y as f32, 1.0, 1.0, GREEN);
                }
            }
        }

        next_frame().await
    }
}