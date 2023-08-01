/* get_options.rs */

pub fn get_opts() -> (String, bool) {
    use crate::images::*;
    use clap::Parser; // Use c[ommand] l[ine] a[rgument] p[arser] to get command-line arguments.

    // Set up program-argument possibilities.
    #[derive(Parser)]
    struct Arguments {
        /// Which model should be used?
        #[arg(long,short, default_value_t = String::from("D51"))]
        model: String,

        /// Should the model fly?
        #[arg(long, short, default_value_t = false)]
        fly: bool,
    } // end of Arguments struct definition

    let args: Arguments = Arguments::parse();

    let image_str = match args.model.to_uppercase().as_str() {
        "P" | "PLANE" => PLANE,
        "D" | "D51" => D51,
        "C" | "C51" => C51,
        "L" | "LITTLE" => LITTLE,
        "B" | "BOAT" => BOAT,
        "T" | "TWIN" | "TWINENGINE" => TWINENGINE,
        "M" | "MOTOR" | "MOTORCYCLE" | "CYCLE" => MOTORCYCLE,
        "J" | "JACK" => JACK,
        "F" | "FERRIS" | "MASCOT" => FERRIS,
        _ => D51,
    };

    let mut fly: bool = false;
    if args.fly {
        fly = true;
    } else {
        fly = false;
    }

    return (image_str.to_string(), fly);
} // end of get_options()
