/* get_opts.rs */

use crate::convert_to_vec::*;
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
