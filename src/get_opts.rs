/* get_opts.rs */

use crate::housekeeping::*;
use crate::images::*;

pub fn get_options() -> Options {
    use clap::Parser; // Use c[ommand] l[ine] a[rgument] p[arser] to get command-line arguments.

    // Set up program-arguments possibilities.
    #[derive(Parser)]
    struct Arguments {
        /// Which image to animate? D[51] | C[51] | L[ittle] | B[oat] | T[win[engine]] | P[lane] | M[otor[cycle]] | J[ack] | F[erris]
        #[arg(long, short, default_value_t = String::from("D51"))]
        kind: String,

        /// Fly? If this switch is provided, the image will "fly".
        #[arg(short, long, default_value_t = false)]
        fly: bool,

        /// Ctrl-c?
        #[arg(short, long, default_value_t = false)]
        breakable: bool,

        /// Accident (only for some images)?
        #[arg(short, long, default_value_t = false)]
        accident: bool,

        /// No trailer (only for D51 & C51 trains)?
        #[arg(short, long, default_value_t = false)]
        no_trailer: bool,
    } // end of Arguments struct

    // The Options struct variable must be initialized; we're mostly using dummy values.
    let mut cli_opts = Options {
        kind: String::from("D51"),
        image_vecvec: vec![vec![String::from("")]],
        speed: 100,
        accident: false,
        fly: false,
        screen_height: 100,
        screen_width: 100,
        craft_height: 100,
        craft_length: 100,
    };

    // Get command-line arguments, if any.
    let args: Arguments = Arguments::parse();

    let image_string = match args.kind.to_uppercase().as_str() {
        "P" | "PLANE" => PLANE,
        "D" | "D51" => D51,
        "C" | "C51" => C51,
        "L" | "LITTLE" => LITTLE,
        "B" | "BOAT" => BOAT,
        "T" | "TWIN" | "TWINENGINE" => TWINENGINE,
        "M" | "MOTOR" | "MOTORCYCLE" => MOTORCYCLE,
        "J" | "JACK" => JACK,
        "F" | "FERRIS" | "MASCOT" => FERRIS,
        _ => D51,
    };
    cli_opts.image_vecvec = string_to_vecvecstrings(image_string.to_string());

    return cli_opts;
} // end of get_options()

pub fn string_to_vecvecstrings(image_string: String) -> Vec<Vec<String>> {
    //The first character in the original const, after the 'r"', is a newline; lose it.
    let image_string = &image_string[1..image_string.len()];

    // Create new blank vector of vector of Strings.
    let mut outer_vec: Vec<Vec<String>> = Vec::new();

    // This keeps track of which frame/inner vector we're processing.
    let mut inner_vec_num: usize = 0;

    // Create first inner vector in the outer vector.
    outer_vec.push(Vec::new());

    for mut each_line in image_string.lines() {
        if each_line != "" {
            // If a blank line is not found ...
            // ... remove single-quote mark at end of each line,
            if &each_line[each_line.len() - 1..] == "'" {
                each_line = &each_line[0..each_line.len() - 1];
            }
            // ... and then add the string to the current inner vector.
            outer_vec[inner_vec_num].push(each_line.to_string());
        } else {
            // If a blank line is found...
            // ... we're done with the current vector, so create a new inner vector...
            outer_vec.push(Vec::new());
            // ... and increase the count of vectors by one.
            inner_vec_num += 1;
        }
    }
    // Return the finished vector of String vectors.
    outer_vec
} // end of string_to_vecvecstrings()
