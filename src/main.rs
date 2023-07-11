/* main.rs */

mod display;
mod get_opts;
mod housekeeping;
mod images;

use display::*;
use get_opts::*;
use housekeeping::*;

fn main() {
    let opts: Options = get_options();

    display_image(opts.image_vecvec);
} // end of main()
