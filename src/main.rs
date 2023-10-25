use components::{math::Size2i, math::Size2f, visuals::Point, math::RandomGen};
use ggez::{ContextBuilder, graphics, event::EventHandler, Context, event, conf};

mod components;
mod data_structures;

const GRID_SIZE: Size2i = Size2i::new(32, 32);
const GRID_CELL_SIZE: Size2i = Size2i::new(48, 48);

const WINDOW_SIZE: Size2f = Size2f::new(
    GRID_SIZE.width as f32 * GRID_CELL_SIZE.width as f32,
    GRID_SIZE.height as f32 * GRID_CELL_SIZE.height as f32,
);



const TARGET_FPS: u32 = 12;
fn main() {
    // Make a context
    let (mut ctx, event_loop) = ContextBuilder::new("game_name", "ISC")
        .window_mode(conf::WindowMode {
            width: WINDOW_SIZE.width,
            height: WINDOW_SIZE.height,
            ..Default::default()
        })
        .build()
        .expect("Error creating ggez context");


    let mut game_state = GameState::new(Grid::new(GRID_SIZE.width as u32, GRID_SIZE.height as u32));
    game_state.randomized(50);  // Randomize 50 cells
    // For wrapping around coordinates
    game_state.grid.pretty_print();
    event::run(ctx, event_loop, game_state);
}


struct GameState {
    grid: Grid,
}

impl GameState {
    fn new(grid: Grid) -> GameState {
        GameState {  grid }
    }
        fn randomized(&mut self, limit: usize) {
            let mut count = 0;
            while count < limit {
                let rand_pos = Size2i::gen_range(0, GRID_SIZE.width, GRID_SIZE.height);
                let x = rand_pos.width;
                let y = rand_pos.height;
                let index = (y as usize * GRID_SIZE.width as usize) + x as usize;
                // Check if the cell is already initialized
                if self.grid.cells.get(index).is_none() {
                    self.grid.cells.push(Cell::new(Point::new(x, y)));
                    count += 1;
                }

            }
        }
    }

    impl EventHandler<ggez::GameError> for GameState {
        fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
            Ok(())
        }

        fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
            self.grid.draw(&mut canvas);
            canvas.finish(ctx)?;
            Ok(())
        }

    }



#[derive(Clone, Copy)]
struct Cell {
    position: Point,
    size: Size2i,
}

impl Cell {
    fn new(position: Point) -> Cell {
        Cell {
            position,
            size: GRID_CELL_SIZE,
        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        println!("Drawing cell at ({}, {})", self.position.x, self.position.y);
        let rect: graphics::Rect = self.clone().into();
        let color = graphics::Color::WHITE;

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
            .dest_rect(rect)
            .color(color),)
    }
}


impl From<Cell> for graphics::Rect {
    fn from(cell: Cell) -> graphics::Rect {
        graphics::Rect::new_i32(
            cell.position.x as i32 * cell.size.width as i32,
            cell.position.y as i32 * cell.size.height as i32,
            cell.size.width as i32,
            cell.size.height as i32,
        )
    }
}
struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Grid {
        Grid {
            width,
            height,
            cells: Vec::new(),
        }
    }

    pub fn draw(&mut self, canvas: &mut graphics::Canvas) {
        for cell in self.cells.iter() {
            cell.draw(canvas);
        }
    }

    pub fn pretty_print(&self) {
        println!("GRID SIZE: ({}, {})", GRID_SIZE.width, GRID_SIZE.height);
        println!("CELL count: {}", self.cells.len());
        for y in 0..GRID_SIZE.height {
            for x in 0..GRID_SIZE.width {
                let index = (y as usize * GRID_SIZE.width as usize) + x as usize;
                if let Some(cell) = self.cells.get(index) {
                    print!("{}", cell.position.x);
                    print!("{}", cell.position.y);
                } else {
                    print!(" "); // " " represents a cell that's out of bounds or not initialized
                }
            }
            println!(); // Move to the next line for the next row
        }
    }




}
