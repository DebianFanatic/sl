#![allow(unused)]

/* main.rs */

mod convert_to_vec;
mod display;
mod get_options;
mod images;

use convert_to_vec::str_to_vecvecstring;
use display::draw_frame;
use get_options::get_opts;
use ncurses::*;
use std::{thread::sleep, time::Duration}; // For a sleep/delay between image frames.

fn main() { 
    // Get image data; put it in "model_vecvecstring".
    let (model_string, fly, oops, interruptible) = get_opts();
    let model_vecvecstring = str_to_vecvecstring(&model_string);

    // ncurses housekeeping stuff.
    initscr(); // Start ncurses, initializing the screen.
    noecho(); // Turn off keyboard echo to the screen.
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Turn off the display of the cursor.
    let mut screen_height = 0; // How high...
    let mut screen_width = 0; // ... and wide ...
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width); // ... is the screen?

    // This will be our delay, in milliseconds, between frame images.
    let delay = 100;
    let delay_between_frames = Duration::from_millis(delay);

    // How many frames in the image?
    let num_of_frames = model_vecvecstring.len() as i32;

    // How tall is the image? (How many lines are in the first inner vector?)
    let image_height = model_vecvecstring[0].len() as i32; // Normally type usize, but we need i32 for the "let row..." below.
        
    // How wide is the image (in characters/screen columns)?
    let image_width = model_vecvecstring[0][0].len() as i32;

    let row: i32 = screen_height - image_height;
    for col in (-image_width..screen_width).rev() { // Distance to travel is width of screen plus the width of the image.
        let frame_num = (col % num_of_frames) as usize; // The remainder of this math tells us which frame to display, in descending order.
        let frame = &model_vecvecstring[frame_num]; // For a 6-frame image, the result will be somewhere in the range of 0 to 5.
        draw_frame(frame, row, col); // ... and draw it.
        sleep(delay_between_frames);
    }
    endwin(); // Exit ncurses and restore screen to normal.
} // end of main()

