use ggez::{ContextBuilder, graphics, event::EventHandler, Context, event, conf};
use rand::Rng;

const GRID_SIZE: Size2i = Size2i::new(16, 16);
const GRID_CELL_SIZE: Size2i = Size2i::new(48, 48);

const WINDOW_SIZE: Size2f = Size2f::new(
    GRID_SIZE.width as f32 * GRID_CELL_SIZE.width as f32,
    GRID_SIZE.height as f32 * GRID_CELL_SIZE.height as f32,
);

trait RandomGen<T> {
    fn gen_range(min: T, max_x: T, max_y: T) -> Self;
}

struct Size2f {
    width: f32,
    height: f32,
}

impl Size2f {
    const fn new(width: f32, height: f32) -> Size2f {
        Size2f { width, height }
    }
}

impl RandomGen<f32> for Size2f {
    fn gen_range(min: f32, max_x: f32, max_y: f32) -> Self {
        let mut rng = rand::thread_rng();
        Size2f {
            width: rng.gen_range(min..max_x),
            height: rng.gen_range(min..max_y),
        }
    }
}

struct Size2i {
    width: i16,
    height: i16,
}

impl RandomGen<i16> for Size2i {
    fn gen_range(min: i16, max_x: i16, max_y: i16) -> Self {
        let mut rng = rand::thread_rng();
        Size2i {
            width: rng.gen_range(min..max_x),
            height: rng.gen_range(min..max_y),
        }
    }
}

impl Size2i {
    const fn new(width: i16, height: i16) -> Size2i {
        Size2i { width, height }
    }
}

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
            let mut count = 0;
            while count < limit {
                let rand_pos = Size2i::gen_range(0, GRID_SIZE.width, GRID_SIZE.height);
                let x = rand_pos.width;
                let y = rand_pos.height;
                let index = (y as usize * GRID_SIZE.width as usize) + x as usize;
                // Check if the cell is already initialized
                if self.grid.cells.get(index).is_none() {
                    self.grid.cells.push(Cell::new(GridPosition::new(x, y)));
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
        let x = (self.x + GRID_SIZE.width) % GRID_SIZE.width;
        let y = (self.y + GRID_SIZE.height) % GRID_SIZE.height;
        GridPosition { x, y }
    }
}

// Implement the `From` trait, to easily convert between
// a `GridPosition` and a ggez `graphics::Rect`, to fill
// the cell's grid.
// To obtain a cell's Rect representation, call `.into()` on
// the `GridPosition` instance.
impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> graphics::Rect {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.width as i32,
            pos.y as i32 * GRID_CELL_SIZE.height as i32,
            GRID_CELL_SIZE.width as i32,
            GRID_CELL_SIZE.height as i32,
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
}

impl Cell {
    fn new(position: GridPosition) -> Cell {
        Cell {
            position,
        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        let rect: graphics::Rect = self.position.into();
        let color = graphics::Color::WHITE;

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
