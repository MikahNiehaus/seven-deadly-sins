use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::error::Error;
use rusty_audio::Audio;

fn main()  {
    // let mut audio = Audio::new();
    // audio.add("beep2", "beep2.wav");
    // audio.play("beep2");
    // audio.wait();




    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1, 1), termion::clear::All)
            .unwrap();
    stdout.flush().unwrap();

    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Char('h') => println!("termion is cool"),
            Key::Char('q') => break,
            Key::Char('t') => println!("termion is cool"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
}