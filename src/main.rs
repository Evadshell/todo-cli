extern crate ncurses;
use ncurses::*;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::{env, process};
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
        addstr(text).unwrap();
        attroff(COLOR_PAIR(pair));
        self.row += 1;

        // todo!()
    }
    fn begin_list(&mut self, id: usize) {
        assert!(self.list_curr.is_none(), "Nested list are not allowed");
        self.list_curr = Some(id);
        // todo!()
    }
    fn list_element(&mut self, label: &str, id: Id) -> bool {
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
        return false;
    }
    fn end_list(&mut self) {
        self.list_curr = None;
    }
}
#[derive(Debug)]
enum Focus {
    Todo,
    Done,
}
impl Focus {
    fn toggle(&self) -> Self {
        match self {
            Focus::Done => Focus::Todo,
            Focus::Todo => Focus::Done,
        }
    }
}
fn parse_item(line: &str) -> Option<(Focus, &str)> {
    let todo_prefix = "TODO: ";
    let done_prefix = "TODO: ";
    if line.starts_with(todo_prefix) {
        return Some((Focus::Todo, &line[todo_prefix.len()..]));
    }
    if line.starts_with(done_prefix) {
        return Some((Focus::Done, &line[done_prefix.len()..]));
    } else {
        None
    }
}
fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            eprintln!("ERROR: file path not provided");
            process::exit(1)
        }
    };
    let mut focus = Focus::Todo;

    let mut todos = Vec::<String>::new();

    let mut quit = false;
    let mut todo_curr: usize = 0;
    let mut dones = Vec::<String>::new();
    let mut ui = Ui::default();
    let mut done_curr: usize = 0;
    {
        let file = File::open(file_path.clone()).unwrap();
        for (index, line) in io::BufReader::new(file).lines().enumerate() {
            match parse_item(&line.unwrap()) {
                Some((Focus::Todo, title)) => {
                    todos.push(title.to_string());
                }

                Some((Focus::Done, title)) => {
                    dones.push(title.to_string());
                }
                None => {
                    eprintln!("{} {} ill formed line item ", file_path, index + 1);
                    process::exit(1)
                }
            }
        }
    }
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    while !quit {
        // addstr("TODO :");
        erase();
        ui.begin(0, 0);
        {
            match focus {
                Focus::Todo => {
                    ui.label("TODO ::", REGULAR_PAIR);
                    ui.begin_list(todo_curr);
                    for (index, todo) in todos.iter().enumerate() {
                        ui.list_element(&format!("- [ ] {}", todo), index);
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
                }
                Focus::Done => {
                    ui.label("DONE ::", REGULAR_PAIR);
                    ui.begin_list(done_curr);
                    for (index, done) in dones.iter().enumerate() {
                        ui.list_element(&format!("- [x] {}", done), index);
                    }
                    ui.end_list();
                }
            }
        }
        ui.end();
        refresh();
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            // _ => {addstr(key as u8 as ch).unwrap()}
            'w' => {
                match focus {
                    Focus::Done => {
                        if done_curr > 0 {
                            done_curr = done_curr - 1
                        }
                    }
                    Focus::Todo => {
                        if todo_curr > 0 {
                            todo_curr = todo_curr - 1
                        }
                    }
                }
                // else if todo_curr == 0 {
                //     todo_curr = todos.len() - 1;
                // }
            }
            // 'e' => {
            //     let mut file = File::create("TODO").unwrap();
            //     for todo in todos.iter() {
            //         writeln!(file, "TODO : {}", todo);
            //     }
            //     for done in dones.iter() {
            //         writeln!(file, "DONE : {}", done);
            //     }
            // }
            's' => {
                match focus {
                    Focus::Todo => {
                        if todo_curr + 1 < todos.len() {
                            todo_curr = todo_curr + 1
                        }
                    }
                    Focus::Done => {
                        if done_curr + 1 < dones.len() {
                            done_curr = done_curr + 1
                        }
                    }
                }
                // else if todo_curr == todos.len() - 1 {
                //     todo_curr = 0;
                // }
            }
            '\n' => match focus {
                Focus::Todo => {
                    if todo_curr < todos.len() {
                        dones.push(todos.remove(todo_curr));
                        if todo_curr >= todos.len() && todos.len() > 0 {
                            todo_curr = todos.len() - 1;
                        }
                    }
                }
                Focus::Done => {
                    if done_curr < dones.len() {
                        todos.push(dones.remove(done_curr));
                        if done_curr >= dones.len() && dones.len() > 0 {
                            done_curr = dones.len() - 1;
                        }
                    }
                }
            },
            '\t' => focus = focus.toggle(),
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
