mod essential_functions;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use crate::essential_functions::rand_range;

#[derive(PartialEq, Debug)]
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
    burning_level: i8,
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
    fn burn(&mut self){
        if self.is_burning {
            if self.burning_level > 5 {
                self.is_burning = false;
                self.burning_level = 0;
                self.burnt = true;
            } else {
            self.burning_level += 1;
            }
        } else {
            self.is_burning = true;
            self.burning_level += 1;
        }
    }
}

#[derive(Debug)]
struct Grid
{
    size_x: i32,
    size_y: i32,
    grid: Vec<Vec<BurnablePoint>>,
    burning_positions: Vec<Position>
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
            burning_positions: vec![]
        }
    }
    fn random_burn(&mut self){
        let rand_y = rand_range(0, 100);
        let rand_x = rand_range(0, 100);
        self.grid[rand_y][rand_x].burn();
    }
}

fn main() { // Make a Context.
    let d = Grid::grid(100, 100);



    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}