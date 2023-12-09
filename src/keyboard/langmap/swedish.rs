use super::langmap::LangMap;
use crossterm::event::*;

pub struct SwedishLangMap;

impl LangMap for SwedishLangMap {
    fn keymap(&self, event: KeyEvent) -> Option<char> {
        match event.code {
            KeyCode::Char('ª') =>  return Some('Å'),
            KeyCode::Char('º') =>  return Some('å'),

            KeyCode::Char(';') =>  return Some('Ä'),
            KeyCode::Char(',') =>  return Some('ä'),

            KeyCode::Char(':') =>  return Some('Ö'),
            KeyCode::Char('.') =>  return Some('ö'),

            KeyCode::Char(c) =>  return Some(c),
            _ => None,
        }
    }
}


