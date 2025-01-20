#![allow(dead_code, unused)]
use termion::{event::Key, input::TermRead, raw::IntoRawMode, terminal_size};

use std::{
    fmt::write,
    io::{Write, stdin, stdout},
};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}q to exit , Type stuff and all. {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .unwrap();
    stdout.flush().unwrap();
    for k in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();
        match k.as_ref().unwrap() {
            Key::Char('q') => break,
            _ => println!("{:?}", k.unwrap()),
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
