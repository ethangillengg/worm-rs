use std::{thread, time::Duration};

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
    pub fn draw(&self, width: usize, height: usize) {
        let horizontal_border = &self.border_types.horizontal;
        let vertical_border = &self.border_types.vertical;
        let top_left = &self.border_types.top_left;
        let top_right = &self.border_types.top_right;
        let bottom_left = &self.border_types.bottom_left;
        let bottom_right = &self.border_types.bottom_right;

        println!("{}", termion::clear::All);

        // Draw top border
        print!("{}", top_left);
        print!("{}", horizontal_border.repeat(width));
        println!("{}", top_right);

        for _ in 0..height {
            // Draw left border
            print!("{}", vertical_border);

            // Draw empty space inside the board
            print!("{}", " ".repeat(width));

            // Draw right border
            println!("{}", vertical_border);
        }

        // Draw bottom border
        print!("{}", bottom_left);
        print!("{}", horizontal_border.repeat(width));
        println!("{}", bottom_right);

        println!("{}", termion::cursor::Goto(1, 1));
    }
}

fn main() {
    let board = GameBoard::default();
    let mut term_size = termion::terminal_size().unwrap();
    term_size.0 -= 2;
    term_size.1 -= 4;

    board.draw(term_size.0.into(), term_size.1.into());
    thread::sleep(Duration::from_millis(4000));
}
