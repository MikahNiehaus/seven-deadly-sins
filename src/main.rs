use crate::startPage::{startPage, chooseCharacter};

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::characters::{lust,wrath};
mod characters;
mod startPage;


struct State {
    character:  String,
} 



fn main()  {

 let mut state = "";

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
            Key::Char('y') => {yHit(state.to_string());},
            Key::Char('y') => break,
            Key::Char('s') => {chooseCharacter()},
            Key::Char('1') => { keyOne(); state = "CHOSE_LUST";},
            Key::Char('2') => {lust();wrath();},
            // Key::Char('3') => chooseLust(),
            // Key::Char('4') => chooseLust(),
            // Key::Char('5') => wrath(),
            // Key::Char('6') => chooseLust(),
            // Key::Char('7') => chooseLust(),

            _ => (),
        }

        stdout.flush().unwrap();
    }
}

fn keyOne(){
    lust();
    println!("{}She had a husband who loved her and three kids. Life was perfect until she cheated on her husband and destroyed her family", termion::cursor::Goto(1, 53), );
    println!("{}Choose this character", termion::cursor::Goto(1, 54), );
    println!("{}y/n", termion::cursor::Goto(1, 55), );
    
}

fn yHit(state: std::string::String) {
    if state == "CHOSE_LUST"{
    println!("{}Blubdsfi;oalsdfjas", termion::cursor::Goto(1, 1), );
    }
}

