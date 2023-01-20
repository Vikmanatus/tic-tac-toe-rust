use std::{fmt::Error, io::stdout};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

mod game_utils;

fn main() -> Result<(), Box<Error>> {
    let hello_user = "John doe";
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();

    println!("Hello, world! {}", hello_user);
    stdout.execute(Show).unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();

    Ok(())
}
