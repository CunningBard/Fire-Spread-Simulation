mod essential_functions;

use crate::essential_functions::{rand_item_index, rand_prob, rand_prob_, rand_range, switch_bool};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::input::{mouse, keyboard};
use ggez::event::{self, EventHandler, KeyCode, MouseButton};


const BURN_SURROUNDING_PROBABILITY: i32 = 30;
const BURN_LIFETIME: u8 = 5;

fn surrounding_position(pos: &(i32, i32), max_x: i32, min_x: i32, max_y: i32, min_y: i32) -> Vec<(i32, i32)>{
    let minus_positions = vec![
    vec![-1, -1], vec![ 0, -1], vec![ 1, -1],
    vec![-1,  0],               vec![ 1,  0],
    vec![-1,  1], vec![ 0,  1], vec![ 1,  1]
];

    let mut positions = vec![];
    for position in &minus_positions {
        let x = pos.0 + position[0];
        let y = pos.1 + position[1];
        if x >= min_x && max_x >= x && y >= min_y && max_y >= y {
            positions.push((x, y))
        }
    }
    positions
}


#[derive(PartialEq, Debug, Clone, Copy)]
struct BurnablePoint
{
    is_burning: bool,
    burnt: bool,
    burning_level: u8,
    position: (i32, i32),
}

impl BurnablePoint
{
    fn new(position: (i32, i32)) -> Self {
        Self {
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
    check_burning_positions: Vec<Vec<bool>>,
    burning_positions: Vec<(i32, i32)>,
}
impl Grid {
    fn new(size_x: i32, size_y: i32) -> Self {
        let mut grid = vec![];
        for y in 0..size_y {
            let mut y_axis = vec![];
            for x in 0..size_x {
                y_axis.push(BurnablePoint::new((x, y)))
            }
            grid.push(y_axis);
        }

        let mut check_burning_positions = vec![];
        for _y in 0..size_y {
            let mut y_axis = vec![];
            for _x in 0..size_x {
                y_axis.push(false)
            }
            check_burning_positions.push(y_axis);
        }

        Self {
            grid,
            size_x,
            size_y,
            check_burning_positions,
            burning_positions: vec![],
        }
    }

    fn random_burn(&mut self){
        let rand_y = rand_range(0, self.size_y);
        let rand_x = rand_range(0, self.size_x);
        self.grid[rand_y as usize][rand_x as usize].burn();
        self.burning_positions.push((rand_x, rand_y));
        self.check_burning_positions[rand_y as usize].remove(rand_x as usize);
        self.check_burning_positions[rand_y as usize].insert(rand_x as usize, true);
    }

    fn handle(&mut self){
        let mut to_burn: Vec<(i32, i32)> = vec![];
        let mut remove = vec![];
        let mut remove_pos = vec![];
        let mut ind = -1;
        for pos in &self.burning_positions {
            ind += 1;
            let bp: &mut BurnablePoint = &mut self.grid[pos.1 as usize][pos.0 as usize];
            if bp.is_burning{
                if bp.burn(){
                    to_burn.push(*pos);
                }
            } else {
                remove.push(ind as usize);
                remove_pos.push(*pos);
            }
        }
        remove.reverse();
        for to_remove in remove {
            self.burning_positions.remove(to_remove);
        }
        // add remove thing from burning pos
        for burn_now in &to_burn {
            let mut points = surrounding_position(burn_now, self.size_x - 1, 0, self.size_y - 1, 0);
            let mut ind = -1;
            let mut to_remove = vec![];
            for point in &points {
                ind += 1;
                if self.check_burning_positions[point.1 as usize][point.0 as usize]{
                    to_remove.push(ind);
                }
            }
            to_remove.reverse();
            for remove_point in to_remove {
                points.remove(remove_point as usize);
            }
            if !points.is_empty(){
                let burn_point = points[rand_item_index(points.clone())];
                self.grid[burn_point.1 as usize][burn_point.0 as usize].burn();
                self.burning_positions.push(burn_point);
                self.check_burning_positions[burn_point.1 as usize].remove(burn_point.0 as usize);
                self.check_burning_positions[burn_point.1 as usize].insert(burn_point.0 as usize, true);

            }
        }
    }
}


fn main() {
    for _ in 0..10{
        println!("PRESS 1 TO RUN");
    }

    let (mut ctx, event_loop) = ContextBuilder::new("fireSpreadSim", "CunningBard")
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx, 800, 600, 1);
    event::run(ctx, event_loop, my_game);
}


struct MyGame {
    grid: Grid,
    to_handle: bool
}

impl MyGame {
    pub fn new(_ctx: &mut Context, size_x: i32, size_y: i32,  random_burns: i32) -> Self {
        let mut grid = Grid::new(size_x, size_y);
        for _ in 0..random_burns {
            grid.random_burn()
        }
        
        Self {
            grid,
            to_handle: false
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(_ctx, KeyCode::Key1){
            self.to_handle = switch_bool(self.to_handle);
        }

        if self.to_handle {
            self.grid.handle();
        } else {
            if mouse::button_pressed(_ctx, MouseButton::Left){
                let m_pos = mouse::position(_ctx);
                if m_pos.x >= 0.0 && m_pos.x <= self.grid.size_x as f32 && m_pos.y >= 0.0 && m_pos.y <= self.grid.size_y as f32 {
                let mut gg = &mut self.grid.grid[m_pos.y as i32 as usize][m_pos.x as i32  as usize];
                    if !gg.burnt {
                        gg.is_burning = false;
                        gg.burning_level = 0;
                        gg.burnt = true;
                        self.grid.check_burning_positions[gg.position.1 as usize].remove(gg.position.0 as usize);
                        self.grid.check_burning_positions[gg.position.1 as usize].insert(gg.position.0 as usize, true);
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from((236, 236, 236, 255)));
        let mut g = vec![];
        for y in &self.grid.grid {
            for x in y{
                if x.is_burning{
                    g.push(255);
                    g.push(0);
                    g.push(0);
                    g.push(255);
                } else if x.burnt{
                    g.push(0);
                    g.push(0);
                    g.push(0);
                    g.push(255);
                } else {
                    g.push(0);
                    g.push(255);
                    g.push(0);
                    g.push(255);
                }
            }
        }
        let res = graphics::Image::from_rgba8(ctx, self.grid.size_x as u16,
                                              self.grid.size_y as u16, &g)?;

        graphics::draw(ctx, &res, (glam::vec2(0.0, 0.0), 0.0, Color::WHITE))?;
        // println!("{:?}", now.elapsed());
        graphics::present(ctx)
    }
}
