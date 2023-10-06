use rand::Rng;
use std::{
    collections::HashSet,
    io::{stdout, Write},
};

use termion::{
    color,
    cursor::{Down, Goto, Left, Right},
};

use crate::game::Game;

pub trait Entity {
    fn draw(&mut self);

    fn goto_origin(&self) {
        print!("{}", Goto(1, 1));
        stdout().flush().unwrap();
    }
    fn goto_line_start(&self) {
        print!("{}", Left(u16::MAX));
        stdout().flush().unwrap();
    }
    fn goto_next_line_start(&self) {
        print!("{}", Left(u16::MAX));
        print!("{}", Down(1));
        stdout().flush().unwrap();
    }
}

#[derive(Debug)]
pub struct Worm {
    pub segments: Vec<(u16, u16)>,
    pub old_tail: Option<(u16, u16)>,
    current_direction: MoveDirection,
}

#[derive(Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Worm {
    pub fn new(x: u16, y: u16, length: u16) -> Worm {
        let segments: Vec<(u16, u16)> = (0..length).map(|i| (x - i, y)).collect();
        Worm {
            segments,
            current_direction: MoveDirection::Right,
            old_tail: None,
        }
    }

    pub fn length(&mut self) -> usize {
        self.segments.len()
    }

    pub fn head(&mut self) -> &(u16, u16) {
        self.segments.first().unwrap()
    }

    pub fn tail(&mut self) -> &(u16, u16) {
        self.segments.last().unwrap()
    }

    pub fn grow(&mut self) {
        let tail = self.tail();
        let new_seg = (tail.0 - 1, tail.1);

        self.segments.push(new_seg);
    }

    pub fn try_set_direction(&mut self, new_direction: MoveDirection) {
        // Don't allow illegal moves
        match (&self.current_direction, &new_direction) {
            (
                MoveDirection::Up | MoveDirection::Down,
                MoveDirection::Left | MoveDirection::Right,
            ) => {
                self.current_direction = new_direction;
            }
            (
                MoveDirection::Left | MoveDirection::Right,
                MoveDirection::Up | MoveDirection::Down,
            ) => {
                self.current_direction = new_direction;
            }
            _ => {}
        }
    }

    pub fn move_forward(&mut self) {
        self.old_tail = Some(self.tail().clone());
        for i in (1..=self.segments.len() - 1).rev() {
            let next_seg = self.segments[i - 1];
            self.segments[i].0 = next_seg.0;
            self.segments[i].1 = next_seg.1;
        }

        let head = self.segments.first_mut().unwrap();
        match self.current_direction {
            MoveDirection::Up => head.1 -= 1,
            MoveDirection::Down => head.1 += 1,
            MoveDirection::Left => head.0 -= 1,
            MoveDirection::Right => head.0 += 1,
        }
    }
}

impl Entity for Worm {
    fn draw(&mut self) {
        write!(stdout(), "{}", color::Fg(color::Magenta)).unwrap();
        for pos in &mut self.segments {
            print!("{}", Goto(pos.0, pos.1));
            print!("");
            // print!("◉");
        }
        match self.old_tail {
            Some(pos) => {
                print!("{}", Goto(pos.0, pos.1));
                print!(" ");
            }
            None => {}
        }
        print!("{}", color::Fg(color::Reset));
        stdout().flush().unwrap();
    }
}

#[derive(Debug)]
pub struct BorderTypes {
    pub horizontal: String,
    pub vertical: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_left: String,
    pub bottom_right: String,
}

impl Default for BorderTypes {
    fn default() -> Self {
        BorderTypes {
            horizontal: "─".into(),
            vertical: "│".into(),
            top_left: "╭".into(),
            top_right: "╮".into(),
            bottom_left: "╰".into(),
            bottom_right: "╯".into(),
        }
    }
}

pub struct Board {
    pub border_types: BorderTypes,
    pub width: u16,
    pub height: u16,
}

impl Entity for Board {
    fn draw(&mut self) {
        let bt = &self.border_types;

        // Top Border
        self.goto_origin();
        print!(
            "{}{}{}",
            bt.top_left,
            bt.horizontal.repeat((self.width).into()),
            bt.top_right
        );
        self.goto_line_start();
        // self.goto_next_line_start();

        //Body
        for _ in 0..self.height {
            print!(
                "{}{}{}{}{}",
                Left(u16::MAX),
                Down(1),
                bt.vertical,
                Right(self.width),
                bt.vertical,
            );
        }

        // Bottom Border
        print!(
            "{}{}{}",
            bt.bottom_left,
            bt.horizontal.repeat((self.width).into()),
            bt.bottom_right
        );
        stdout().flush().unwrap();
    }
}

pub struct Fruit {
    pub pos: (u16, u16),
}

impl Fruit {
    pub fn new() -> Fruit {
        Fruit { pos: (0, 0) }
    }
}

impl Entity for Fruit {
    fn draw(&mut self) {
        print!("{}", Goto(self.pos.0, self.pos.1));

        write!(stdout(), "{}", color::Fg(color::Green)).unwrap();
        print!("󰉛");
        write!(stdout(), "{}", color::Fg(color::Reset)).unwrap();
        stdout().flush().unwrap();
    }
}
