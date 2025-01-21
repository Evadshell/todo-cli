#![allow(unused)]
extern crate ncurses;
use ncurses::*;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;
type Id = usize;
#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}
impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
        // todo!()
    }
    fn end(&mut self) {
        // todo!()
    }
    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;

        // todo!()
    }
    fn begin_list(&mut self, id: usize) {
        assert!(self.list_curr.is_none(), "Nested list are not allowed");
        self.list_curr = Some(id);
        // todo!()
    }
    fn list_element(&mut self, label: &str, id: Id) {
        let id_curr = self
            .list_curr
            .expect("Not allowed to create list element outside the list area");

        self.label(label, {
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });
        // attron(COLOR_PAIR(pair));
        // mv(index as i32, 1);
        // addstr(*todo).unwrap();
        // attroff(COLOR_PAIR(pair));
    }
    fn end_list(&mut self) {
        self.list_curr = None;
    }
}
fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut todos = vec![
        "make a todo".to_string(),
        "learn rust".to_string(),
        "muheuehueh".to_string(),
    ];
    let mut quit = false;
    let mut todo_curr: usize = 1;
    let mut dones = Vec::<String>::new();
    let mut ui = Ui::default();
    let mut done_curr: usize = 0;
    while !quit {
        // addstr("TODO :");
        ui.begin(0, 0);
        {
            ui.begin_list(todo_curr);
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(todo, index);
                // let pair = {
                //     if todo_curr == index {
                //         HIGHLIGHT_PAIR
                //     } else {
                //         REGULAR_PAIR
                //     }
                // };
                // attron(COLOR_PAIR(pair));
                // mv(index as i32, 1);
                // addstr(*todo).unwrap();
                // attroff(COLOR_PAIR(pair));
            }
            ui.end_list();

            // ui.label("----------------------------------",REGULAR_PAIR);
            // ui.begin_list(done_curr);
            // for (index, done) in dones.iter().enumerate() {
            //     ui.list_element(done, index);
            // }
            // ui.end_list();
        }
        ui.end();
        refresh();
        let key = getch();
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
            // 'd' => {
            //     done.push(todos[todo_curr]);
            //     todos.remove(todo_curr);
            // }
            _ => {}
        }
    }
    getch();
    endwin();
}
