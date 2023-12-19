use rand::Rng;
use std::{
    collections::HashSet,
    io::{stdin, stdout, StdoutLock, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use termion::{
    color::{self},
    cursor::{Goto, Show},
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen},
};

use crate::{
    entity::{Board, BorderTypes, Entity, Fruit, MoveDirection, Worm},
    GameSettings,
};

#[derive(Debug, Eq, PartialEq)]
pub enum GameStatus {
    Playing,
    Paused,
    Lost,
    Won,
    ShowingStats,
    Exiting,
}
pub struct RenderStats {
    pub frame_count: u32,
    pub total_render_duration: Duration,
}
impl RenderStats {
    pub fn new() -> RenderStats {
        RenderStats {
            frame_count: 0,
            total_render_duration: Duration::from_millis(0),
        }
    }

    pub fn avg_frame_time(&self) -> Duration {
        self.total_render_duration / self.frame_count
    }

    pub fn print_stats(&self) {
        println!("{}", termion::clear::All);
        println!("{}", Goto(1, 1));
        println!("Total Frames Rendered: {}", self.frame_count);
        println!("{}", Goto(1, 2));
        println!(
            "Average Frame Time: {}μs",
            self.avg_frame_time().as_micros()
        );
        println!("{}", Goto(1, 3));
        thread::sleep(Duration::from_millis(1000));
    }
}

pub struct Game<'a> {
    pub fps: u64,
    pub width: u16,
    pub height: u16,
    pub board: Board,
    pub worm: Worm,
    pub fruits: Vec<Fruit>,
    pub status: GameStatus,
    pub settings: GameSettings,
    pub render_stats: RenderStats,

    stdin_channel: mpsc::Receiver<Key>,
    // need this for the lifetime, the lock lasts until it goes out of scope
    //TODO better solution??
    stdout_lock: AlternateScreen<RawTerminal<StdoutLock<'a>>>,
}

impl Game<'_> {
    pub fn new<'a>(settings: GameSettings) -> Game<'a> {
        let term_size = termion::terminal_size().unwrap();
        let width = term_size.0 - 2;
        let height = term_size.1 - 2;

        let mut game = Game {
            fps: 30,
            width,
            height,
            board: Board {
                width,
                height,
                border_types: BorderTypes::default(),
            },
            worm: Worm::new(5, 20, settings.worm_length),
            fruits: Vec::new(),
            status: GameStatus::Playing,
            settings,

            render_stats: RenderStats::new(),
            stdin_channel: spawn_stdin_channel(),
            stdout_lock: get_stdout_raw_lock(),
        };

        if ((height * width - game.settings.worm_length) as u64) < game.settings.fruit_count {
            game.quit();
        }
        return game;
    }

    pub fn start(&mut self) {
        println!("{}{}", termion::clear::All, termion::cursor::Hide);

        self.status = GameStatus::Playing;
        self.worm = Worm::new(5, 20, self.settings.worm_length);
        self.regenerate_fruits().unwrap();

        loop {
            match self.status {
                GameStatus::Playing => self.game_running_loop(),
                GameStatus::Won | GameStatus::Lost => self.game_finished_loop(),
                GameStatus::Exiting => break,
                _ => {}
            }
        }
    }

    pub fn quit(&mut self) {
        if self.settings.stats {
            self.status = GameStatus::ShowingStats;
            self.render_stats.print_stats();
            while self.status != GameStatus::Exiting {
                self.handle_input();
                thread::sleep(Duration::from_millis(1000 / self.fps));
            }
        }

        print!("{}{}", Show, Goto(0, 0));
        stdout().flush().unwrap();
        self.status = GameStatus::Exiting;
    }

    pub fn game_finished_loop(&mut self) {
        let msg: &str;
        let icon: String;
        match self.status {
            GameStatus::Won => {
                msg = "You Won!!";
                icon = format!("{}󰆥{}", color::Fg(color::Yellow), color::Fg(color::Reset));
            }
            _ => {
                icon = format!(
                    "{}󰚌{}",
                    color::Fg(color::LightWhite),
                    color::Fg(color::Reset)
                );
                msg = "You Died...";
            }
        }

        let w_offset: u16 = (self.width - u16::try_from(msg.len()).unwrap_or(0)) / 2;
        let mut h_offset = (self.height) / 2;

        print!("{}{}", termion::clear::All, Goto(w_offset, h_offset));
        print!("{} {}", icon, msg);
        h_offset += 3;

        print!("{}press q to quit", Goto(w_offset, h_offset));
        h_offset += 2;

        print!("{}press r to retry", Goto(w_offset, h_offset));
        stdout().flush().unwrap();

        while let GameStatus::Won | GameStatus::Lost = self.status {
            self.handle_input();
            thread::sleep(Duration::from_millis(1000 / self.fps));
        }
    }

    pub fn game_running_loop(&mut self) {
        let mut last_frame_time: Instant;
        let mut elapsed: Duration = Duration::from_millis(0);
        let mut sleep_duration: Duration = Duration::from_millis(0);

        // Draw all the entities initially
        self.board.draw();
        for fruit in &mut self.fruits {
            fruit.draw();
        }

        //Render loop
        while let GameStatus::Playing = self.status {
            last_frame_time = Instant::now();

            self.draw();
            self.handle_input();
            self.update_game_state();

            print!("{} Length: {} ", Goto(3, 1), self.worm.length());
            stdout().flush().unwrap();

            elapsed = Instant::now() - last_frame_time;
            sleep_duration = Duration::from_millis(1000 / self.fps) - elapsed;

            thread::sleep(sleep_duration);
            self.render_stats.frame_count += 1;
            self.render_stats.total_render_duration += elapsed;
        }
    }

    fn draw(&mut self) {
        self.worm.draw();
    }

    fn handle_input(&mut self) {
        // Get the most recently pressed key
        let key = self.stdin_channel.try_iter().last().unwrap_or(Key::Null);
        match self.status {
            GameStatus::Playing => match key {
                Key::Char('q') => self.quit(),
                Key::Char('w') => self.worm.try_set_direction(MoveDirection::Up),
                Key::Char('a') => self.worm.try_set_direction(MoveDirection::Left),
                Key::Char('s') => self.worm.try_set_direction(MoveDirection::Down),
                Key::Char('d') => self.worm.try_set_direction(MoveDirection::Right),
                _ => {}
            },
            GameStatus::Won | GameStatus::Lost => match key {
                Key::Char('q') => self.quit(),
                Key::Char('r') => self.start(),
                _ => {}
            },
            GameStatus::ShowingStats => match key {
                Key::Char('q') => self.status = GameStatus::Exiting,
                _ => {}
            },
            _ => {}
        }
    }

    fn update_game_state(&mut self) {
        self.worm.move_forward();

        // Check if the worm's head is on top of a fruit
        for i in 0..self.fruits.len() {
            if self.worm.segments[0] == self.fruits[i].pos {
                self.worm.grow();

                let o = self.get_occupied_positions();

                // Set the fruit's position to a new random position
                match self.get_random_unoccupied_pos() {
                    Ok(pos) => {
                        self.fruits[i].pos = pos;
                        self.fruits[i].draw();
                        return;
                    }
                    // Board was full
                    Err(_) => {
                        self.status = GameStatus::Won;
                        return;
                    }
                }
            }
        }
        // Check if the worm has collided with the border
        if self.worm.segments[0].0 < 2
            || self.worm.segments[0].0 > self.width + 1
            || self.worm.segments[0].1 < 2
            || self.worm.segments[0].1 > self.height + 1
        {
            self.status = GameStatus::Lost;
            return;
        }

        // Check if the worm has collided with itself
        for i in 1..self.worm.segments.len() {
            if self.worm.segments[0] == self.worm.segments[i] {
                self.status = GameStatus::Lost;
                return;
            }
        }
    }

    fn get_occupied_positions(&self) -> HashSet<(u16, u16)> {
        self.fruits
            .iter()
            .map(|f| f.pos)
            .chain(self.worm.segments.iter().cloned())
            .collect()
    }

    fn regenerate_fruits(&mut self) -> Result<(), String> {
        // clear out existing fruits
        self.fruits = Vec::<Fruit>::new();

        // //OLD SLOW ALGORITHM
        // for _ in 0..self.settings.fruit_count {
        //     // make a new fruit at a random position that is not occupied
        //     self.fruits.push(Fruit {
        //         pos: self.get_random_unoccupied_pos()?,
        //     });
        // }
        // return Ok(());

        // 1. iterate through all positions on the board
        // 2. give each pos a chance to spawn a fruit based on (# fruit remaining to place/# of empty positions)
        let mut index = 0; // represent curent postion in 1d, iterate col then row
        let occupied_positions = self.get_occupied_positions();

        // offset range by one to avoid division by 0 in the gen_bool call
        for fruit_remaining in (1..self.settings.fruit_count + 1).rev() {
            let mut rng = rand::thread_rng();

            loop {
                index += 1;
                let pos = (index % self.width, index / self.width);

                // chance to spawn a fruit based on num remaining, and number of positions left
                if rng.gen_bool(fruit_remaining as f64 / (self.width * self.height - index) as f64)
                {
                    if occupied_positions.contains(&pos) {
                        continue;
                    } else {
                        self.fruits.push(Fruit {
                            pos: (pos.0 + 2, pos.1 + 2),
                        });
                        break;
                    }
                }
            }
        }
        return Ok(());
    }

    fn get_random_unoccupied_pos(&self) -> Result<(u16, u16), String> {
        let mut rng = rand::thread_rng();

        let occupied_positions = self.get_occupied_positions();
        let h_range = 2..self.height + 2;
        let w_range = 2..self.width + 2;

        // error if board is full already
        if occupied_positions.len() >= (self.width * self.height).into() {
            return Err(String::from("Board is full"));
        }

        // loop through rng until we get a position that is free
        loop {
            let pos = (
                rng.gen_range(w_range.clone()),
                rng.gen_range(h_range.clone()),
            );

            if !&occupied_positions.contains(&pos) {
                return Ok(pos);
            }
        }
    }
}

fn spawn_stdin_channel() -> mpsc::Receiver<Key> {
    let (tx, rx) = mpsc::channel::<Key>();
    let stdin = stdin();
    let mut keys = stdin.keys();

    thread::spawn(move || loop {
        match keys.next() {
            Some(key) => tx.send(key.unwrap()).unwrap(),
            None => {}
        }
    });
    rx
}

fn get_stdout_raw_lock<'a>() -> AlternateScreen<RawTerminal<StdoutLock<'a>>> {
    // Set terminal to raw mode to allow reading stdin one key at a time
    stdout()
        .lock()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap()
}
