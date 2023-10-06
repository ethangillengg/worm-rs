use clap::Parser;

pub mod entity;
pub mod game;

/// Terminal worm game in rust!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GameSettings {
    /// Number of fruit
    #[arg(short, long, default_value_t = 5)]
    fruit_count: u64,

    /// Starting length of the worm
    #[arg(short, long, default_value_t = 4)]
    worm_length: u16,

    /// Print stats about render speed on exit
    #[arg(short, long, default_value = "false")]
    stats: bool,
}

fn main() {
    let game_settings = GameSettings::parse();
    let mut game = game::Game::new(game_settings);

    game.start();
}
