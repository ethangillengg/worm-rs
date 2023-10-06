use clap::Parser;

use crate::game::GameSettings;

pub mod entity;
pub mod game;

/// Terminal worm game in rust!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of fruit
    #[arg(short, long)]
    fruit: u16,

    /// Starting length of the worm
    #[arg(short, long, default_value_t = 4)]
    length: u16,
}

fn main() {
    let args = Args::parse();

    let game_settings = GameSettings {
        fruit_count: args.fruit,
        worm_length: args.length,
    };

    let mut game = game::Game::new(game_settings);

    game.start();
}
