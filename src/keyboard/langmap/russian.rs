use super::langmap::LangMap;
use crossterm::event::*;

pub struct RussianLangMap;

impl LangMap for RussianLangMap {
    fn keymap(&self, event: KeyEvent) -> Option<char> {
        match event.code {
            KeyCode::Char('P') => return    Some('П'),
            KeyCode::Char('p') => return    Some('п'),

            KeyCode::Char('\\') => return   Some('ё'),
            KeyCode::Char('|') => return    Some('Ё'),

            KeyCode::Char('A') => return	Some('А'),
            KeyCode::Char('a') => return	Some('а'),	

            KeyCode::Char('B') => return	Some('Б'),
            KeyCode::Char('b') => return	Some('б'),	

            KeyCode::Char('V') => return	Some('В'),
            KeyCode::Char('v') => return	Some('в'),	

            KeyCode::Char('G') => return	Some('Г'),
            KeyCode::Char('g') => return	Some('г'),

            KeyCode::Char('D') => return	Some('Д'),
            KeyCode::Char('d') => return	Some('д'),	

            KeyCode::Char('E') => return	Some('Е'),	
            KeyCode::Char('e') => return	Some('е'),	

            KeyCode::Char('Ç') => return    Some('Ж'),	
            KeyCode::Char('ç') => return    Some('ж'),	
            
            KeyCode::Char('Z') => return	Some('З'),	
            KeyCode::Char('z') => return	Some('з'),	

            KeyCode::Char('I') => return	Some('И'),	
            KeyCode::Char('i') => return	Some('и'),	

            KeyCode::Char('J') => return	Some('Й'),	
            KeyCode::Char('j') => return	Some('й'),	

            KeyCode::Char('K') => return	Some('К'),	
            KeyCode::Char('k') => return	Some('к'),	

            KeyCode::Char('L') => return	Some('Л'),
            KeyCode::Char('l') => return	Some('л'),	

            KeyCode::Char('M') => return	Some('М'),	
            KeyCode::Char('m') => return	Some('м'),	

            KeyCode::Char('N') => return	Some('Н'),
            KeyCode::Char('n') => return	Some('н'),	

            KeyCode::Char('O') => return	Some('О'),
            KeyCode::Char('o') => return	Some('о'),	

            KeyCode::Char('R') => return	Some('Р'),
            KeyCode::Char('r') => return	Some('р'),	

            KeyCode::Char('S') => return	Some('С'),
            KeyCode::Char('s') => return	Some('с'),	

            KeyCode::Char('T') => return	Some('Т'),
            KeyCode::Char('t') => return	Some('т'),	

            KeyCode::Char('U') => return	Some('У'),
            KeyCode::Char('u') => return    Some('у'),	

            KeyCode::Char('F') => return	Some('Ф'),
            KeyCode::Char('f') => return	Some('ф'),	

            KeyCode::Char('H') => return	Some('Х'),
            KeyCode::Char('h') => return	Some('х'),	

            KeyCode::Char('C') => return	Some('Ц'),
            KeyCode::Char('c') => return	Some('ц'),

            KeyCode::Char('X') => return	Some('Ч'),	
            KeyCode::Char('x') => return	Some('ч'),	

            KeyCode::Char('W') => return	Some('Ш'),	
            KeyCode::Char('w') => return	Some('ш'),	

            KeyCode::Char('?') => return    Some('Ь'),	
            KeyCode::Char('\'') => return    Some('ь'),	
            
            KeyCode::Char('»') => return    Some('Ъ'),	
            KeyCode::Char('«') => return    Some('ъ'),	

            KeyCode::Char('ª') => return	Some('Щ'),	
            KeyCode::Char('º') => return   Some('щ'),	

            KeyCode::Char('Y') => return  	Some('Ы'),	
            KeyCode::Char('y') => return  	Some('ы'),	

            KeyCode::Char('>') => return    Some('Э'),
            KeyCode::Char('<') => return    Some('э'),	

            KeyCode::Char('*') => return    Some('Ю'),
            KeyCode::Char('+') => return    Some('ю'),	

            KeyCode::Char('Q') => return	Some('Я'),
            KeyCode::Char('q') => return	Some('я'),	

            KeyCode::Char(c) => return	Some(c),	
            _ => None,
        }
    }
}


