use termion::cursor::{Down, Goto, Left};

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

#[derive(Debug, Default)]
pub struct GameBoard {
    pub border_types: BorderTypes,
}

impl GameBoard {
    pub fn draw(&self) {
        let term_size = termion::terminal_size().unwrap();
        let width: usize = (term_size.0 - 2).into();
        let height: usize = (term_size.1 - 1).into();
        let bt = &self.border_types;

        let next_line = format_args!("{}{}", Left(term_size.0), Down(1)).to_string();

        // Top Border
        print!("{}", Goto(1, 1));
        println!(
            "{}{}{}",
            bt.top_left,
            bt.horizontal.repeat(width),
            bt.top_right
        );
        print!("{}", Left(term_size.0));

        //Body
        for _ in 0..height {
            print!("{}", bt.vertical);
            print!("{}", " ".repeat(width));
            print!("{}", bt.vertical);
            print!("{}", next_line);
        }

        // Bottom Border
        print!(
            "{}{}{}",
            bt.bottom_left,
            bt.horizontal.repeat(width),
            bt.bottom_right
        );

        println!("{}", Goto(1, 1));
    }
}
