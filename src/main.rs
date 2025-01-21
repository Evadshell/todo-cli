#![allow(unused)]
extern crate ncurses;
use ncurses::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut todos = vec!["make a todo", "learn rust", "muheuehueh"];
    let mut quit = false;
    let mut todo_curr: usize = 1;
    let mut done = Vec::<&str>::new();
    while !quit {
        addstr("TODO :");
        for (index, todo) in todos.iter().enumerate() {
            let pair = {
                if todo_curr == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };
            attron(COLOR_PAIR(pair));
            mv(index as i32, 1);
            addstr(*todo).unwrap();
            attroff(COLOR_PAIR(pair));
        }
        addstr("DONE");
        for (index, don) in done.iter().enumerate() {
            mv(index as i32, 30);
            addstr(*don);
        }
        let key = getch();
        refresh();
        match key as u8 as char {
            'q' => quit = true,
            // _ => {addstr(key as u8 as ch).unwrap()}
            'w' => {
                if todo_curr > 0 {
                    todo_curr = todo_curr - 1
                } else if todo_curr == 0 {
                    todo_curr = todos.len() - 1;
                }
            }
            's' => {
                if todo_curr < todos.len() - 1 {
                    todo_curr = todo_curr + 1
                } else if todo_curr == todos.len() - 1 {
                    todo_curr = 0;
                }
            }
            'd' => {
                done.push(todos[todo_curr]);
                todos.remove(todo_curr);
            }
            _ => {}
        }
    }
    getch();
    endwin();
}
