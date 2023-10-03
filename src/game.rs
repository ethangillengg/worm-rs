use rand::Rng;
use std::{
    collections::HashSet,
    io::{stdin, stdout, Write},
    process::exit,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use termion::{
    color::{self},
    cursor::{Goto, Show},
    event::Key,
    input::TermRead,
};

use crate::entity::{Board, BorderTypes, Entity, Fruit, MoveDirection, Worm};

#[derive(Debug)]
pub enum GameStatus {
    Playing,
    Paused,
    Lost,
    Won,
}

pub struct Game {
    pub fps: u64,
    pub frame_count: u32,
    pub stdin_channel: mpsc::Receiver<Key>,
    pub width: u16,
    pub height: u16,
    pub board: Board,
    pub worm: Worm,
    pub fruits: Vec<Fruit>,
    pub status: GameStatus,
}

impl Default for Game {
    fn default() -> Self {
        let term_size = termion::terminal_size().unwrap();
        let width = term_size.0 - 2;
        let height = term_size.1 - 2;
        // let width: u16 = 60;
        // let height: u16 = 30;

        Game {
            fps: 36,
            frame_count: 0,
            stdin_channel: spawn_stdin_channel(),
            width,
            height,
            board: Board {
                width,
                height,
                border_types: BorderTypes::default(),
            },
            worm: Worm::new(5, 2, 4),
            fruits: Vec::new(),
            status: GameStatus::Playing,
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game::default()
    }

    pub fn start(&mut self) {
        print!("{}", termion::clear::All);
        stdout().flush().unwrap();

        self.status = GameStatus::Playing;
        self.worm = Worm::new(5, 2, 4);
        self.regenerate_fruits(10).unwrap();

        loop {
            match self.status {
                GameStatus::Playing => self.game_running_loop(),
                GameStatus::Won | GameStatus::Lost => self.game_finished_loop(),
                _ => {}
            }
        }
    }

    pub fn quit(&mut self) {
        print!("{}{}", Show, Goto(0, 0));
        stdout().flush().unwrap();
        exit(0);
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

            print!("{} State: {:?} ", Goto(3, 1), self.status);
            print!("{} Elapsed: {} ", Goto(24, 1), elapsed.as_millis());
            print!("{} Slept: {} ", Goto(38, 1), sleep_duration.as_millis());
            stdout().flush().unwrap();

            elapsed = Instant::now() - last_frame_time;
            sleep_duration = Duration::from_millis(1000 / self.fps) - elapsed;

            thread::sleep(sleep_duration);
            self.frame_count += 1;
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
            _ => {}
        }
    }

    fn update_game_state(&mut self) {
        self.worm.move_forward();

        // Check if the worm's head is on top of a fruit
        for i in 0..self.fruits.len() {
            if self.worm.segments[0] == self.fruits[i].pos {
                self.worm.grow();
                // Set the fruit's position to a new random position

                match self.get_random_unoccupied_pos() {
                    Ok(pos) => {
                        self.fruits[i].pos = pos;
                        self.fruits[i].draw();
                        return;
                    }
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

    fn get_random_unoccupied_pos(&self) -> Result<(u16, u16), String> {
        let mut rng = rand::thread_rng();

        let occupied_positions = self.get_occupied_positions();
        let h_range = 2..self.height + 2;
        let w_range = 2..self.width + 2;

        if occupied_positions.len() >= (self.width * self.height).into() {
            return Err(String::from("No free positions available"));
        }

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

    fn regenerate_fruits(&mut self, fruit_count: u16) -> Result<(), String> {
        self.fruits = Vec::<Fruit>::new();

        for _ in 0..fruit_count {
            self.fruits.push(Fruit {
                pos: self.get_random_unoccupied_pos()?,
            });
        }

        return Ok(());
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
