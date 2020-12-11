use crate::startPage::{startPage, chooseCharacter, chooseLust};

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::characters::{lust,wrath};
mod characters;
mod startPage;
// use std::error::Error;
// use rusty_audio::Audio;

struct State {
    start:  bool,
} 
trait Start {
    fn start(self: &mut Self);
}

fn main()  {
 
let mut _state = State {
        start: false,
    };

    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();
    startPage();

    


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
            Key::Char('q') => break,
            Key::Char('s') => chooseCharacter(),
            Key::Char('1') => lust(),
            Key::Char('2') => {lust();wrath();},
            Key::Char('3') => chooseLust(),
            Key::Char('4') => chooseLust(),
            Key::Char('5') => wrath(),
            Key::Char('6') => chooseLust(),
            Key::Char('7') => chooseLust(),

            _ => (),
        }

        stdout.flush().unwrap();
    }
}

