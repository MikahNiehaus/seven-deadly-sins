
pub fn startPage() {

    println!("{}███████╗  ██████╗░███████╗░█████╗░██████╗░██╗░░░░░██╗░░░██╗  ░██████╗██╗███╗░░██╗░██████╗", termion::cursor::Goto(1, 1));
    println!("{}█▀▀▄ █▀▀ ", termion::cursor::Goto(1, 2));
    println!("{}╚════██║  ██╔══██╗██╔════╝██╔══██╗██╔══██╗██║░░░░░╚██╗░██╔╝  ██╔════╝██║████╗░██║██╔════╝", termion::cursor::Goto(1, 2));
    println!("{}░░░░██╔╝  ██║░░██║█████╗░░███████║██║░░██║██║░░░░░░╚████╔╝░  ╚█████╗░██║██╔██╗██║╚█████╗░", termion::cursor::Goto(1, 3));
    println!("{}░░░██╔╝░  ██║░░██║██╔══╝░░██╔══██║██║░░██║██║░░░░░░░╚██╔╝░░  ░╚═══██╗██║██║╚████║░╚═══██╗", termion::cursor::Goto(1, 4));
    println!("{}░░██╔╝░░  ██████╔╝███████╗██║░░██║██████╔╝███████╗░░░██║░░░  ██████╔╝██║██║░╚███║██████╔╝", termion::cursor::Goto(1, 5));
    println!("{}░░╚═╝░░░  ╚═════╝░╚══════╝╚═╝░░╚═╝╚═════╝░╚══════╝░░░╚═╝░░░  ╚═════╝░╚═╝╚═╝░░╚══╝╚═════╝░", termion::cursor::Goto(1, 6));
    println!("{}press \"q\" to exit at any time or press \"s\" to start, full screen recomended", termion::cursor::Goto(1, 8), );

 
}

pub fn chooseCharacter(){
    println!("{}Choose a character by pressing a number", termion::cursor::Goto(1, 1), );
    println!("{}1 - Lust", termion::cursor::Goto(1, 3), );
    println!("{}2 - Gluttony", termion::cursor::Goto(1, 4), );
    println!("{}3 - Geed", termion::cursor::Goto(1, 5), );
    println!("{}4 - Sloth", termion::cursor::Goto(1, 6), );
    println!("{}5 - Wrath", termion::cursor::Goto(1, 7), );
    println!("{}6 - Envy", termion::cursor::Goto(1, 8), );
    println!("{}7 - Pride", termion::cursor::Goto(1, 9), );

}
