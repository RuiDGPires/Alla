use crossterm::event::*;

pub trait LangMap {
    fn keymap(&self, event: KeyEvent) -> Option<char>;
}
