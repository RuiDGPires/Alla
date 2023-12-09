use crossterm::{terminal, terminal::ClearType, execute, cursor, style};
use std::io::{stdout, Write};
use crossterm::event:: {
    KeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
    PopKeyboardEnhancementFlags,
};

const LINE_OFF: usize = 5;

#[derive(Default)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Begin,
    End,
}

impl Pos {
    pub fn up(&mut self) {
        self.y -= 1;
    }

    pub fn down(&mut self) {
        self.y += 1;
    }

    pub fn right(&mut self) {
        self.x += 1;
    }

    pub fn left(&mut self) {
        self.x -= 1;
    }
}

pub struct Screen {
    win_size: (u16, u16),
    pub contents: Vec<String>,
    pub cursor: Pos,
    page: usize,
}

impl Drop for Screen {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");

        execute!(stdout(), cursor::Show).expect("Unable to show cursor");
        let _ = Screen::clear();
    }
}

impl Screen {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Could not enable raw mode");
        execute!(stdout(), cursor::Hide).expect("Unable to hide cursor");
        execute!(stdout(),PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)).expect("KeyboardEnhancement");

        let win_size = terminal::size()
            .map(|(x, y)| (x, y))
            .unwrap();
        let cursor = Pos{x: 0, y: 0};
        Self { win_size: win_size, contents: vec![], cursor: cursor, page: 0}
    }

    pub fn from(lines: Vec<&str>) -> Self {
        let mut _self = Self::new();
        _self.contents = lines.iter().map(|x| x.to_string()).collect();
        _self
    }

    pub fn clear() -> Result<(), std::io::Error> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn update(&mut self) -> Result<(), std::io::Error> {
        let screen_rows = self.win_size.1;
        Self::clear()?;

        for i in 0..screen_rows {
            let _ = execute!(stdout(), style::SetBackgroundColor(style::Color::Black));
            let _ = execute!(stdout(), style::SetForegroundColor(style::Color::DarkGrey));
            print!("~");
            let _ = execute!(stdout(), style::SetForegroundColor(style::Color::Reset));
            let _ = execute!(stdout(), style::SetBackgroundColor(style::Color::Reset));
            if i < screen_rows - 1 {
                println!("\r")
            }
        }

        if self.contents.is_empty() {
            self.contents.push("".to_owned());
        }

        let mut row_offset = 0;
        for (y, row) in self.contents.iter().enumerate()
            .filter(|(i, _)| *i >= self.page && *i < self.page + self.win_size.1 as usize) {
            let _ = execute!(stdout(), style::SetBackgroundColor(style::Color::Black));
            let _ = execute!(stdout(), style::SetForegroundColor(style::Color::DarkGrey));
            execute!(stdout(), cursor::MoveTo(0, (y - self.page + row_offset) as u16))?;
            print!("{:3} ", y + 1);
            let _ = execute!(stdout(), style::SetForegroundColor(style::Color::Reset));
            let _ = execute!(stdout(), style::SetBackgroundColor(style::Color::Reset));

            row_offset += row.chars().count() / (self.win_size.0 as usize - LINE_OFF - 1); 
            for (x, c) in row.chars().enumerate() {
                if x == self.cursor.x && y == self.cursor.y {
                    let _ = execute!(stdout(), style::SetBackgroundColor(style::Color::White));
                    let _ = execute!(stdout(), style::SetForegroundColor(style::Color::Black));
                    print!("{}", c);
                    let _ = execute!(stdout(), style::SetBackgroundColor(style::Color::Reset));
                    let _ = execute!(stdout(), style::SetForegroundColor(style::Color::Reset));
                } else {
                    print!("{}", c);
                }
            }

            if y == self.cursor.y  && self.cursor.x >= row.chars().count() {
                let _ = execute!(stdout(), style::SetBackgroundColor(style::Color::White));
                let _ = execute!(stdout(), style::SetForegroundColor(style::Color::Black));
                print!(" ");
                let _ = execute!(stdout(), style::SetBackgroundColor(style::Color::Reset));
                let _ = execute!(stdout(), style::SetForegroundColor(style::Color::Reset));
            }
        }

        let _ = stdout().flush();

        Ok(())
    }
    
    pub fn cursor_next_word(&mut self, dir: Direction) -> () {
        match dir {
            _ => ()
        }
    }

    pub fn move_page(&mut self, dir: Direction) -> () {
        match dir {
            Direction::Up | Direction::Down => {
                for _ in 0..self.win_size.1 {
                    self.cursor_move(dir.clone());
                }
            },

            _ => () 
        }
    }

    pub fn cursor_move(&mut self, dir: Direction) -> () {
        match dir {
            Direction::Up => {
                if self.cursor.y > 0 {
                    self.cursor.up();
                }
                self.cursor.x = usize::min(self.cursor.x, self.contents[self.cursor.y].chars().count());

                if self.cursor.y < self.page { // out of page bounds
                    self.page -= 1; 
                }
            },
            Direction::Down => {
                if self.cursor.y < self.contents.len() - 1{
                    if self.cursor.y >= self.page + self.win_size.1 as usize - 1{ // out of page bounds
                        self.page += 1;
                    }
                    self.cursor.down();
                }
                self.cursor.x = usize::min(self.cursor.x, self.contents[self.cursor.y].chars().count());

            },
            Direction::Left => {
                if self.cursor.x > 0 {
                    self.cursor.left();
                }
            },
            Direction::Right => {
                if self.cursor.x < self.contents[self.cursor.y].chars().count() {
                    self.cursor.right();
                }
            },
            Direction::Begin => {
                self.cursor.x = 0;
            },
            Direction::End => {
                self.cursor.x = self.contents[self.cursor.y].chars().count();
            }

        }
    }
}
