pub fn colorize_string(value: &str, input: String) -> String{
    let mod_string = value.to_uppercase().trim();
    let result = match value {
        "BLACK"=> format!("{}{}",  termion::color::Fg(termion::color::Black), input),
        "BLUE"=>  format!("{}{}",  termion::color::Fg(termion::color::Blue), input),
        "CYAN"=>  format!("{}{}",  termion::color::Fg(termion::color::Cyan), input),
        "GREEN"=> format!("{}{}",  termion::color::Fg(termion::color::Green), input),
        "LIGHTBLACK"=> format!("{}{}",  termion::color::Fg(termion::color::LightBlack), input),
        "LIGHTBLUE"=>  format!("{}{}",  termion::color::Fg(termion::color::LightBlue), input),
        "LIGHTCYAN"=>  format!("{}{}",  termion::color::Fg(termion::color::LightCyan), input),
        "LIGHTGREEN"=> format!("{}{}",  termion::color::Fg(termion::color::LightGreen), input),
        "LIGHTMAGENTA"=>  format!("{}{}",  termion::color::Fg(termion::color::LightMagenta), input),
        "LIGHTRED"=>  format!("{}{}",  termion::color::Fg(termion::color::LightRed), input),
        "LIGHTWHITE"=>  format!("{}{}",  termion::color::Fg(termion::color::LightWhite), input),
        "LIGHTYELLOW"=>  format!("{}{}",  termion::color::Fg(termion::color::LightYellow), input),
        "MAGENTA"=>  format!("{}{}",  termion::color::Fg(termion::color::Magenta), input),
        "RED"=>  format!("{}{}",  termion::color::Fg(termion::color::Red), input),
        "WHITE"=>  format!("{}{}",  termion::color::Fg(termion::color::White), input),
        "YELLOW"=>  format!("{}{}",  termion::color::Fg(termion::color::Yellow), input)
    };
    return result;
}