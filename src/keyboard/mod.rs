mod langmap;
use crossterm::event::{*, self};
use std::time::Duration;
pub use langmap::*;

pub struct Keyboard {
    langmap: Box<dyn LangMap>
}

pub enum Arrow {
    Up,
    Down,
    Right,
    Left
}

pub enum Action {
    Char(char),
    KeyboardChange(Box<dyn LangMap>),
    Enter,
    Arrow(Arrow, bool), // bool -> shift
    Backspace,
    Delete,
    Exit,

    None,
}

impl Keyboard {
    pub fn new() -> Self {
        Self{langmap: Box::new(DefaultLangMap)} 
    }

    fn key_to_char(&self, event: KeyEvent) -> Option<char> {
        self.langmap.keymap(event)
    }

    pub fn change_map(&mut self, langmap: Box<dyn LangMap>) -> () {
        self.langmap = langmap;
    }

    pub fn poll_key(&self) -> Action {
        if let Some(event) = Self::read_key() {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => return Action::Exit,
                KeyEvent {
                    code: KeyCode::F(8),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => return Action::KeyboardChange(Box::new(RussianLangMap)),
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => return Action::Enter,
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: x,
                    kind: _,
                    state: _,
                } => return Action::Arrow(Arrow::Up, x == KeyModifiers::SHIFT),
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: x,
                    kind: _,
                    state: _,
                } => return Action::Arrow(Arrow::Down, x == KeyModifiers::SHIFT),
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: x,
                    kind: _,
                    state: _,
                } => return Action::Arrow(Arrow::Left, x == KeyModifiers::SHIFT),
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: x,
                    kind: _,
                    state: _,
                } => return Action::Arrow(Arrow::Right, x == KeyModifiers::SHIFT),
                KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => return Action::Backspace,
                KeyEvent {
                    code: KeyCode::Delete,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => return Action::Delete,
                _ => {
                    if let Some(c) = self.key_to_char(event) {
                        return Action::Char(c);
                    }

                    return Action::None;
                }
            }
        }

        Action::None
    }

    fn read_key() -> Option<KeyEvent> {
        if event::poll(Duration::from_millis(500)).ok()? {
            if let Event::Key(event) = event::read().ok()? { 
                return Some(event);
            }
        }
         
        None 
    }
}



