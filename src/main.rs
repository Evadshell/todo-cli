use termion::{raw::IntoRawMode, terminal_size};

use std::io::{Write, stdout};

fn main() {
  
    println!("{:?}", terminal_size());
  
    // println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    //     let mut stdout = stdout().into_raw_mode()?;
    //   write!(stdout, "Hey there .")?;

    //   Ok(())
}
