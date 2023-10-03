use rand::Rng;
use std::{
    collections::HashSet,
    io::{stdin, stdout, Write},
    process::exit,
    sync::mpsc,
    thread,
    time::{self, Duration, Instant},
};
use termion::{
    cursor::{Goto, Show},
    event::Key,
    input::TermRead,
};

use crate::entity::{Board, BorderTypes, Entity, Fruit, MoveDirection, Worm};

pub struct Game {
    pub fps: u64,
    pub frame_count: u32,
    pub stdin_channel: mpsc::Receiver<Key>,
    pub width: u16,
    pub height: u16,
    pub board: Board,
    pub worm: Worm,
    pub fruits: Vec<Fruit>,
}

impl Default for Game {
    fn default() -> Self {
        let term_size = termion::terminal_size().unwrap();
        let width = term_size.0 - 2;
        let height = term_size.1 - 2;
        // let width: u16 = 6;
        // let height: u16 = 6;
        let fruit_count: u16 = 32;

        let mut game = Game {
            fps: 20,
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
        };

        game.regenerate_fruits(fruit_count).unwrap();

        return game;
    }
}

impl Game {
    pub fn new() -> Game {
        Game::default()
    }

    pub fn start(&mut self) {
        //Render loop
        let mut last_frame_time = Instant::now();
        let mut elapsed: Duration = Duration::from_millis(0);
        let mut sleep_duration: Duration = Duration::from_millis(0);
        loop {
            last_frame_time = Instant::now();

            self.draw();
            self.handle_input();
            self.update_game_state();
            print!("{} Elapsed: {} ", Goto(24, 1), elapsed.as_millis());
            stdout().flush().unwrap();
            print!("{} Slept: {} ", Goto(38, 1), sleep_duration.as_millis());
            stdout().flush().unwrap();
            self.sleep_inconsistent_random();

            // thread::sleep(Duration::from_millis(1000 / self.fps));
            // Measure elapsed time since last frame
            elapsed = Instant::now() - last_frame_time;
            sleep_duration = Duration::from_millis(1000 / self.fps) - elapsed;

            thread::sleep(sleep_duration);
            self.frame_count += 1;
        }
    }
    pub fn sleep_inconsistent_random(&self) {
        let ms_to_sleep = rand::thread_rng().gen_range(0..(900 / self.fps));
        print!("{} Random Sleep: {} ", Goto(3, 1), ms_to_sleep);
        stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(ms_to_sleep));
    }

    pub fn draw(&mut self) {
        self.board.draw();
        self.worm.draw();

        // print!("{} Length: {} ", Goto(3, 1), self.worm.length());

        for fruit in &mut self.fruits {
            fruit.draw();
        }
    }

    fn handle_input(&mut self) {
        // Get the most recently pressed key
        let key = self.stdin_channel.try_iter().last().unwrap_or(Key::Null);
        match key {
            Key::Char('q') => {
                print!("{}{}", Show, Goto(0, 0));
                stdout().flush().unwrap();
                exit(0);
            }
            Key::Char('w') => self.worm.try_set_direction(MoveDirection::Up),
            Key::Char('a') => self.worm.try_set_direction(MoveDirection::Left),
            Key::Char('s') => self.worm.try_set_direction(MoveDirection::Down),
            Key::Char('d') => self.worm.try_set_direction(MoveDirection::Right),
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
                self.fruits[i].pos = self.get_random_unoccupied_pos().unwrap();
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

    fn regenerate_fruits(&mut self, fruit_count: u16) -> Result<(), String> {
        self.fruits = Vec::<Fruit>::new();
        for _ in 0..fruit_count {
            self.fruits.push(Fruit {
                pos: self.get_random_unoccupied_pos()?,
            });
        }
        return Ok(());
    }

    pub fn get_random_unoccupied_pos(&self) -> Result<(u16, u16), String> {
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
