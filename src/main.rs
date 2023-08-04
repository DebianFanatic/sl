/* main.rs */
// After adding Fly
mod convert_to_vec;
mod display;
mod get_options;
mod images;

use convert_to_vec::str_to_vecvecstring;
use display::display_image;
use get_options::get_opts;

fn main() {
    let (model_string, fly, oops, interruptible) = get_opts();
    let model_vecvecstring = str_to_vecvecstring(&model_string);
    display_image(model_vecvecstring);
    println!("The option to fly = {}.", fly); // Temporary test of what we get back in "fly".
    println!("The Accident (\"oops\") option = {}.", oops); // Temporary test of what we get back in "oops";
    println!("The \"interruptible\") option = {}.", interruptible); // Temporary test of what we get back in "interruptible";
} // end of main()
