use std::{
    io::{stdin, stdout, Write},
    process::exit,
    sync::mpsc,
    thread, time,
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

        Game {
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
            worm: Worm::new(width / 2, height / 2, 4),
            fruits: vec![Fruit { pos: (50, 14) }],
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game::default()
    }

    pub fn start(&mut self) {
        loop {
            // Read input (if any)
            self.draw();
            self.handle_input();
            self.update_game_state();
            // If a key was pressed
            thread::sleep(time::Duration::from_millis(1000 / self.fps));
            self.frame_count += 1;
        }
    }

    pub fn draw(&mut self) {
        self.board.draw();
        self.worm.draw();
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
            Key::Char('w') => self.worm.current_direction = MoveDirection::Up,
            Key::Char('a') => self.worm.current_direction = MoveDirection::Left,
            Key::Char('s') => self.worm.current_direction = MoveDirection::Down,
            Key::Char('d') => self.worm.current_direction = MoveDirection::Right,
            _ => {}
        }
    }

    fn update_game_state(&mut self) {
        self.worm.move_forward();
        // Check if the worm's head is on top of a fruit
        if self.worm.segments[0] == self.fruits[0].pos {
            self.worm.grow();
            self.fruits[0].randomize_pos(self.width, self.height);
        }
    }
}

fn spawn_stdin_channel() -> mpsc::Receiver<Key> {
    let (tx, rx) = mpsc::channel::<Key>();
    // let stdin = stdin().lock();
    let stdin = stdin();
    let mut keys = stdin.keys();

    thread::spawn(move || loop {
        match keys.next() {
            Some(key) => {
                tx.send(key.unwrap()).unwrap();

                // let mut buffer = String::new();
                // stdin.read_to_string(&mut buffer).unwrap();
            }
            None => {}
        }
    });
    rx
}
