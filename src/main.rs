use std::{fmt::Error, io::stdout, time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, event::{poll, Event, read, KeyCode},
};

mod game_utils;

fn main() -> Result<(), Box<Error>> {
    let hello_user = "John doe";
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();

    'gameloop: loop {
        while poll( Duration::default()).unwrap() {
            if let Event::Key(key_code) = read().unwrap() {
                match key_code.code {
                    KeyCode::Esc => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }
    println!("Hello, world! {}", hello_user);
    stdout.execute(Show).unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();

    Ok(())
}
