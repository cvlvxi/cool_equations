#![feature(generic_const_exprs)]
mod game;
mod utils;
mod types;

use speedy2d::{Graphics2D, Window};


const WIDTH: u32 = 2048;
const HEIGHT: u32 =  1024;
const HEIGHT_MAP_SIZE: usize = 256;

use game::Game;


#[tokio::main]
async fn main() {

    let game = Game::new(HEIGHT_MAP_SIZE, WIDTH, HEIGHT);
    let window = Window::new_centered("Title", (WIDTH, HEIGHT)).unwrap();
    window.run_loop(game);
}
