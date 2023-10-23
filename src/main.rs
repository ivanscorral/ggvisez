use ggez::{ContextBuilder, graphics, event::EventHandler, Context, event};
use rand::Rng;

const GRID_SIZE: (i16, i16) = (10, 10);
const GRID_CELL_SIZE: (i16, i16) = (12, 12);

const WINDOW_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const TARGET_FPS: u32 = 12;
fn main() {
    // Make a context
    let (mut ctx, event_loop) = ContextBuilder::new("game_name", "ISC")
        .build()
        .expect("Error creating ggez context");


    // Event handler instance
    // Inside your main or game loop
    let mut game_state = GameState::new(Grid::new(GRID_SIZE.0 as u32, GRID_SIZE.1 as u32));
    game_state.randomized(50);  // Randomize 50 cells
    // For wrapping around coordinates
    let pos = GridPosition::new(15, 15);
    let wrapped_pos = pos.wrap_around();
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
            let mut rng = rand::thread_rng();

            for _ in 0..limit {
                let x = rng.gen_range(0..GRID_SIZE.0);
                let y = rng.gen_range(0..GRID_SIZE.1);
                let index = (y as usize * GRID_SIZE.0 as usize) + x as usize;

                // Check if the cell is already initialized
                if self.grid.cells.get(index).is_none() {
                    self.grid.cells.push(Cell::new(GridPosition::new(x, y)));
                } else {
                    // If the cell is already initialized, check if it's active, and if not, make it active
                    if !self.grid.cells[index].active {
                        self.grid.cells[index].active = true;
                    }
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


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    fn new(x: i16, y: i16) -> GridPosition {
        GridPosition { x, y }
    }

    fn wrap_around(&self) -> GridPosition {
        let x = (self.x + GRID_SIZE.0) % GRID_SIZE.0;
        let y = (self.y + GRID_SIZE.1) % GRID_SIZE.1;
        GridPosition { x, y }
    }
    // TODO: Make this work
}

// Implement the `From` trait, to easily convert between
// a `GridPosition` and a ggez `graphics::Rect`, to fill
// the cell's grid.
// To obtain a cell's Rect representation, call `.into()` on
// the `GridPosition` instance.
impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> graphics::Rect {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

// Implement the `From` trait to easily convert (i16, i16) to `GridPosition`.
impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

struct Cell {
    position: GridPosition,
    active: bool,
}

impl Cell {
    fn new(position: GridPosition) -> Cell {
        Cell {
            position,
            active: true,
        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        let rect: graphics::Rect = self.position.into();
        let color = if self.active {
            graphics::Color::WHITE
        } else {
            graphics::Color::BLACK
        };
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
            .dest_rect(rect)
            .color(color),)
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

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        for cell in self.cells.iter() {
            cell.draw(canvas);
        }
    }

    pub fn pretty_print(&self) {
        println!("GRID SIZE: ({}, {})", GRID_SIZE.0, GRID_SIZE.1);
        println!("CELL count: {}", self.cells.len());
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                let index = (y as usize * GRID_SIZE.0 as usize) + x as usize;
                if let Some(cell) = self.cells.get(index) {
                    if cell.active {
                        print!("O"); // "O" represents an active cell
                    } else {
                        print!("."); // "." represents an inactive cell
                    }
                } else {
                    print!(" "); // " " represents a cell that's out of bounds or not initialized
                }
            }
            println!(); // Move to the next line for the next row
        }
    }




}
