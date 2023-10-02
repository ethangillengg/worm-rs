use std::{
    io::{stdin, stdout, Write},
    process::exit,
    sync::mpsc,
    thread, time,
};
use termion::{cursor::Show, event::Key, input::TermRead};

use crate::entity::{Board, BorderTypes, Entity, MoveDirection, Worm};

pub struct Game {
    pub fps: u64,
    pub frame_count: u32,
    pub stdin_channel: mpsc::Receiver<Key>,
    pub width: u16,
    pub height: u16,
    pub board: Board,
    pub worm: Worm,
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
            worm: Worm::default(),
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
            // If a key was pressed
            thread::sleep(time::Duration::from_millis(1000 / self.fps));
            self.frame_count += 1;
        }
    }

    pub fn draw(&mut self) {
        self.board.draw();
        self.worm.draw();
    }

    fn handle_input(&mut self) {
        let key = self.stdin_channel.try_recv().unwrap_or(Key::Null); //No input found
        match key {
            // Exit if 'q' is pressed
            Key::Char('q') => {
                print!("{}", Show);
                stdout().flush().unwrap();
                exit(0);
            }
            Key::Char('w') => self.worm.move_forward(MoveDirection::Up),
            Key::Char('a') => self.worm.move_forward(MoveDirection::Left),
            Key::Char('s') => self.worm.move_forward(MoveDirection::Down),
            Key::Char('d') => self.worm.move_forward(MoveDirection::Right),

            // Else print the pressed key
            _ => {}
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
            }
            None => {}
        }
    });
    rx
}
