use rand::Rng;
use std::io::{stdout, Write};

use termion::{
    color,
    cursor::{Down, Goto, Left},
};

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
    fn goto_next_line(&self) {
        print!("{}", Left(u16::MAX));
        print!("{}", Down(1));
        stdout().flush().unwrap();
    }
}

#[derive(Debug)]
pub struct Worm {
    pub segments: Vec<(u16, u16)>,
    pub current_direction: MoveDirection,
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

    pub fn move_forward(&mut self) {
        let new_segs = self.segments.clone();
        let head = self.segments.first_mut().unwrap();

        match self.current_direction {
            MoveDirection::Up => head.1 -= 1,
            MoveDirection::Down => head.1 += 1,
            MoveDirection::Left => head.0 -= 1,
            MoveDirection::Right => head.0 += 1,
        }

        for i in 1..self.segments.len() {
            self.segments[i] = new_segs[i - 1];
        }
    }
}

impl Entity for Worm {
    fn draw(&mut self) {
        write!(stdout(), "{}", color::Fg(color::LightMagenta)).unwrap();
        for pos in &mut self.segments {
            print!("{}", Goto(pos.0, pos.1));
            print!("◉");
        }
        write!(stdout(), "{}", color::Fg(color::Reset)).unwrap();

        print!("{}Length:{}", Goto(2, 2), self.length());
        self.goto_origin();
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
            bt.horizontal.repeat(self.width.into()),
            bt.top_right
        );
        self.goto_line_start();
        self.goto_next_line();

        //Body
        for _ in 0..self.height.into() {
            print!("{}", bt.vertical);
            print!("{}", " ".repeat(self.width.into()));
            print!("{}", bt.vertical);
            self.goto_next_line();
        }

        // Bottom Border
        print!(
            "{}{}{}",
            bt.bottom_left,
            bt.horizontal.repeat(self.width.into()),
            bt.bottom_right
        );
        stdout().flush().unwrap();
    }
}

pub struct Fruit {
    pub pos: (u16, u16),
}
impl Fruit {
    pub fn randomize_pos(&mut self, width: u16, height: u16) {
        self.pos = (
            rand::thread_rng().gen_range(1..width),
            rand::thread_rng().gen_range(1..height),
        );
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
