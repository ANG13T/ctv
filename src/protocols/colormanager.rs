pub fn get_color_for_string(value: &str) {
    let mod_string = value.to_uppercase().trim();
    let result = match value {
        "BLACK"=> termion::color::Fg(termion::color::Black),
        "BLUE"=>  termion::color::Fg(termion::color::Blue),
        "CYAN"=>  termion::color::Fg(termion::color::Cyan),
        "GREEN"=> termion::color::Fg(termion::color::Green),
        "LIGHTBLACK"=> termion::color::Fg(termion::color::LightBlack),
        "LIGHTBLUE"=>  termion::color::Fg(termion::color::LightBlue),
        "LIGHTCYAN"=>  termion::color::Fg(termion::color::LightCyan),
        "LIGHTGREEN"=> termion::color::Fg(termion::color::LightGreen),
        "LIGHTMAGENTA"=>  termion::color::Fg(termion::color::LightMagenta),
        "LIGHTRED"=>  termion::color::Fg(termion::color::LightRed),
        "LIGHTWHITE"=>  termion::color::Fg(termion::color::LightWhite),
        "LIGHTYELLOW"=>  termion::color::Fg(termion::color::LightYellow),
        "MAGENTA"=>  termion::color::Fg(termion::color::Magenta),
        "RED"=>  termion::color::Fg(termion::color::Red),
        "WHITE"=>  termion::color::Fg(termion::color::White),
        "YELLOW"=>  termion::color::Fg(termion::color::Yellow)
    };
    return result;
}