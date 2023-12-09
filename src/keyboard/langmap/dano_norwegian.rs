use super::langmap::LangMap;
use crossterm::event::*;

pub struct DanoNorwegianLangMap;

impl LangMap for DanoNorwegianLangMap {
    fn keymap(&self, event: KeyEvent) -> Option<char> {
        match event.code {
            KeyCode::Char('ª') =>  return Some('Å'),
            KeyCode::Char('º') =>  return Some('å'),

            KeyCode::Char(';') =>  return Some('Æ'),
            KeyCode::Char(',') =>  return Some('æ'),

            KeyCode::Char(':') =>  return Some('Ø'),
            KeyCode::Char('.') =>  return Some('ø'),

            KeyCode::Char(c) =>  return Some(c),
            _ => None,
        }
    }
}


