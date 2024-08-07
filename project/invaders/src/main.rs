use std::{error::Error, io};
use std::io::stdout;
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide,Show};
use rusty_audio::Audio;

fn main() -> Result<(),Box<dyn Error>>{
    //println!("Hello, world!");
    let mut audio=Audio::new();
    audio.add("explode","explode.wav");
    audio.add("lose","lose.wav");
    audio.add("move","move.wav");
    audio.add("pew","pew.wav");
    audio.add("startup","startup.wav");
    audio.add("win","win.wav");
    audio.play("startup");

    //Terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    execute!(stdout,Hide)?;

    //Cleanup
    audio.wait();
    execute!(stdout,Show)?;
    execute!(stdout,LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
