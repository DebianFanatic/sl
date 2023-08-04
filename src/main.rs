/* main.rs */
// After adding Fly
mod convert_to_vec;
mod display;
mod get_options;
mod images;

use convert_to_vec::str_to_vecvecstring;
use display::display_image;
use get_options::get_opts;
use ncurses::*;

fn main() {
    let (model_string, fly, oops, interruptible) = get_opts();
    let model_vecvecstring = str_to_vecvecstring(&model_string);

    // ncurses housekeeping stuff.
    initscr(); // Start ncurses, initializing the screen.
    noecho(); // Turn off keyboard echo to the screen.
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Turn off the display of the cursor.
    let mut screen_height = 0; // How high...
    let mut screen_width = 0; // ... and wide ...
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width); // ... is the screen?

    display_image(model_vecvecstring);

    endwin(); // Exit ncurses and restore screen to normal.
} // end of main()
