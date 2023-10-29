
use components::{math::Size2f, math::Size2i, visuals::Point};
use data_structures::quadtree::Quadtree;
use ggez::{conf, event, event::EventHandler, graphics, Context, ContextBuilder};
use crate::{data_structures::quadtree::Region, encoding::{encoder::Encoder, decoder::Decoder}};

mod components;
mod data_structures;
mod encoding;
mod io;

const GRID_SIZE: Size2i = Size2i::new(128, 64);
const GRID_CELL_SIZE: Size2i = Size2i::new(8, 8);

const WINDOW_SIZE: Size2f = Size2f::new(
    GRID_SIZE.width as f32 * GRID_CELL_SIZE.width as f32,
    GRID_SIZE.height as f32 * GRID_CELL_SIZE.height as f32,
);

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

    let mut game_state = GameState::new(&Quadtree::new(Point::new(0, 0), GRID_SIZE));
    game_state.randomized(1024);
    let encoder = Encoder::new(GRID_SIZE, game_state.point_quadtree.clone());
    let encoded_file = match encoder.to_file(&"test.bin".to_string()) {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    let mut decoder = Decoder::new(encoded_file.bytes());
    let decoded_data = decoder.decode().unwrap();
    println!("Read {} points from file", decoded_data.len());

    // event::run(ctx, event_loop, game_state);
}

struct GameState {
    point_quadtree: Quadtree,
}

impl GameState {
    fn new(point_quadtree: &Quadtree) -> GameState {
        GameState { point_quadtree: point_quadtree.clone() }
    }

    fn encode(&self) -> Vec<u8> {
        Encoder::new(GRID_SIZE, self.point_quadtree.clone()).encode()
    }

    fn randomized(&mut self, limit: usize) {
        let mut count = 0;
        while count < limit {
            let rand_point = Point::rand(GRID_SIZE.width as u32, GRID_SIZE.height as u32);
            self.point_quadtree.insert_point(rand_point);
            count += 1;
        }
        println!(
            "Randomized {} points",
            self.point_quadtree
                .query_region(&Region {
                    top_left: Point::new(0, 0),
                    size: GRID_SIZE
                })
                .len()
        );
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let points = self
            .point_quadtree
            .query_region(&self.point_quadtree.region);
        points.iter().for_each(|point| {
            let cell = Cell::new(*point);
            cell.draw(&mut canvas);
        });
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
            graphics::DrawParam::new().dest_rect(rect).color(color),
        )
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
