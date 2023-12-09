use std::env;
mod screen;
mod keyboard;

use keyboard::Keyboard;

fn arrow_to_dir(arrow: keyboard::Arrow) -> screen::Direction {
    match arrow {
        keyboard::Arrow::Up => screen::Direction::Up,
        keyboard::Arrow::Down => screen::Direction::Down,
        keyboard::Arrow::Right => screen::Direction::Right,
        keyboard::Arrow::Left => screen::Direction::Left,
    }
}

fn string_insert(s: &String, index: usize, character: char) -> String {
    if s == "" {
        return character.to_string();
    }

    if index >= s.chars().count() {
        return format!("{}{}", s, character);
    }


    let mut new_string = "".to_owned();

    for (i, c) in s.chars().enumerate() {
        if i == index {
            new_string.push(character);
        }
        new_string.push(c);
    }

    new_string
}

fn string_delete(s: &String, index: usize) -> String {
    if s == "" {
        return "".to_owned();
    }

    let mut new_string = "".to_owned();

    for (i, c) in s.chars().enumerate() {
        if i != index {
            new_string.push(c);
        }
    }

    new_string
}

fn string_split(s: &String, index: usize) -> (String, String) {
    if s == "" {
        return ("".to_owned(), "".to_owned());
    }

    let mut left = "".to_owned();
    let mut right = "".to_owned();

    for (i, c) in s.chars().enumerate() {
        if i < index {
            left.push(c);
        } else {
            right.push(c);
        }
    }

    (left, right)
}

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();

    let mut screen = {
        if let Some(file) = argv.iter().nth(1) {
            if let Ok(file) = std::fs::read_to_string(file) {
                screen::Screen::from(file.lines().collect())
            } else {
                screen::Screen::new()
            }
        } else {
            screen::Screen::new()
        }
    };


    let mut keyboard = Keyboard::new();

    loop {
        screen.update()?;
        match keyboard.poll_key() {
            keyboard::Action::Exit => break,
            keyboard::Action::KeyboardChange(k) => {
                keyboard.change_map(k);
            },
            keyboard::Action::Char(c) => {
                screen.contents[screen.cursor.y] = string_insert(&screen.contents[screen.cursor.y], screen.cursor.x, c);
                screen.cursor_move(screen::Direction::Right);
            },
            keyboard::Action::Enter => {
                let (left, right) = string_split(&screen.contents[screen.cursor.y], screen.cursor.x);
                screen.contents[screen.cursor.y ] = left;
                screen.contents.insert(screen.cursor.y  + 1, right);
                screen.cursor_move(screen::Direction::Down);
                screen.cursor_move(screen::Direction::Begin);
            },
            keyboard::Action::Arrow(a, shift) => {
                if !shift {
                    screen.cursor_move(arrow_to_dir(a));
                } else {
                    match a {
                        keyboard::Arrow::Up | keyboard::Arrow::Down => {
                            screen.move_page(arrow_to_dir(a));
                        },
                        keyboard::Arrow::Right | keyboard::Arrow::Left => {
                            screen.cursor_next_word(arrow_to_dir(a));
                        }
                    }
                }
            },
            keyboard::Action::Backspace => {
                if screen.cursor.x != 0 {
                    screen.contents[screen.cursor.y ] = string_delete(&screen.contents[screen.cursor.y ], screen.cursor.x - 1);
                    screen.cursor_move(screen::Direction::Left);
                } else {
                    if screen.cursor.y > 0 {
                        let prev = screen.contents[screen.cursor.y  - 1].clone();
                        let prev_y = screen.cursor.y;
                        screen.cursor_move(screen::Direction::Up);
                        screen.cursor_move(screen::Direction::End);
                        screen.contents[prev_y  - 1] = prev + &screen.contents[prev_y ];
                        screen.contents.remove(prev_y );
                    }
                }
            },
            keyboard::Action::Delete => {
                if screen.cursor.x < screen.contents[screen.cursor.y ].chars().count() {
                    screen.contents[screen.cursor.y ] = string_delete(&screen.contents[screen.cursor.y ], screen.cursor.x);
                } else {
                    if screen.cursor.y < screen.contents.len() - 1{
                        let prev = screen.contents[screen.cursor.y ].clone();
                        screen.contents[screen.cursor.y ] = prev + &screen.contents[screen.cursor.y  + 1];
                        screen.contents.remove(screen.cursor.y  + 1);
                    }
                }
            },

            _ => continue,
        }
    }

    Ok(())
}
