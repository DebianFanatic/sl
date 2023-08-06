/* display.rs */

use ncurses::*;

pub fn draw_frame(frame: &Vec<String>, mut row: i32, mut col: i32) {
    for each_line in frame {
        mvaddstr(row, col, &each_line);
        row += 1;
    }
    refresh();
} // end of draw_frame()
