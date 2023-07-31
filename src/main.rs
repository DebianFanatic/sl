/* main.rs */

mod convert_to_vec;
mod display;
mod get_options;
mod images;

use convert_to_vec::str_to_vecvecstring;
use display::display_image;
use get_options::get_opts;

fn main() {
    let model_string = get_opts();
    let model_vecvecstring = str_to_vecvecstring(&model_string);
    display_image(model_vecvecstring);
} // end of main()
