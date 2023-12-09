use super::langmap::LangMap;
use crossterm::event::*;

pub struct DefaultLangMap;

impl LangMap for DefaultLangMap {
    fn keymap(&self, event: KeyEvent) -> Option<char> {
        match event.code {
            KeyCode::Char(c) =>  return Some(c),
            _ => None,
        }
    }
}

