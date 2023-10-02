use std::io::stdout;

use termion::cursor::Hide;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use termion::{self, clear};

pub mod entity;
pub mod game;

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    println!("{}{}", clear::All, Hide);
    let mut stdout = stdout()
        .lock()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let mut game = game::Game::new();
    game.start();
}
