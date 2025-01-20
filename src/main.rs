extern crate ncurses;
use ncurses::*;
fn main() {
    initscr();
    let mut todos = vec!["make a todo", "learn rust"];
    let mut quit = false;
    while !quit {
        // addstr("test");
        for (row, todo) in todos.iter().enumerate() {
            mv(row as i32, 1);
            addstr(*todo);
        }
        let key = getch();
        refresh();
        match key as u8 as char {
            'q' => quit = true,
            // _ => {addstr(key as &str).unwrap()}
            _ => {}
        }
    }
    getch();
    endwin();
}
