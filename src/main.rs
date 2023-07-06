/* main.rs */
/* Kent West, July 2023 */

mod images;
mod housekeeping;

use images::*;
use housekeeping::*;

fn main() {

    // let kind = get_options();
    let opts: Options = get_options();

    let image_vecvec = string_to_vecvecstrings(opts.image_string);

    display_image(image_vecvec);
    
} // end of main()

fn get_options() -> Options {
    use clap::Parser; // Use c[ommand] l[ine] a[rgument] p[arser] to get command-line arguments.

    #[derive(Parser)]
    struct Arguments {
      kind: String,
    }
  
    // The struct variable must be initialized; we're mostly using dummy values.
    let mut cli_opts = Options {
        kind: String::from("D51"),
        image_string: String::from(""),
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
      "P" | "PLANE"                 =>  PLANE,
      "D" | "D51"                   =>  D51,
      "C" | "C51"                   =>  C51,
      "L" | "LITTLE"                =>  LITTLE,
      "B" | "BOAT"                  =>  BOAT,
      "T" | "TWIN" | "TWINENGINE"   =>  TWINENGINE,
      "M" | "MOTOR" | "MOTORCYCLE"  =>  MOTORCYCLE,        
      "J" | "JACK"                  =>  JACK,
      "F" | "FERRIS"                =>  FERRIS,
      _                             =>  D51,
    };
    cli_opts.image_string = image_string.to_string();

    return cli_opts;
} // end of get_options()

fn string_to_vecvecstrings(image_string: String) -> Vec<Vec<String>> {

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


fn display_image(image_vecvec: Vec<Vec<String>>) {
    let mut frame_num = 0;
    for each_frame in image_vecvec {
      println!("Frame {}:", frame_num);
      for each_line in each_frame {
        println!("{}", each_line);
      }
    frame_num += 1;
    }
} // end of display_image()
