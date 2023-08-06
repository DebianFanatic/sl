/* convert_to_vec.rs */

pub fn str_to_vecvecstring(incoming_str: &str) -> Vec<Vec<String>> {
    //The first character in the original const, after the 'r"', is a newline; lose it.
    let image_str: &str = &incoming_str[1..];

    // Create new empty vector of vectors of Strings.
    let mut outer_vec: Vec<Vec<String>> = Vec::new();

    // This keeps track of which frame/inner vector we're processing.
    let mut inner_vec_num: usize = 0; // We'll start with the first one.

    // Create a first new inner vector into the outer vector.
    outer_vec.push(Vec::new());

    for mut each_line in image_str.lines() { // Cycle through each line of the image_str image.
        if each_line == "" { // If the line is a blank line ...
            // ... we're starting a new frame, so create a new inner vector...
            outer_vec.push(Vec::new());
            // ... and increase the count of vectors by one.
            inner_vec_num += 1;
        } else { // The line is not blank, so process it as part of the current frame.
            if &each_line[each_line.len() - 1..] == "'" { // If end of line is a single-quote ...
                each_line = &each_line[0..each_line.len() - 1]; // ... remove it.
            }
            // Add an erasing space to the end of each line (converting it to a String from a &str).
            let each_line = format!("{} ", each_line);
            // Now add the non-blank line to the current inner vector.
            outer_vec[inner_vec_num].push(each_line);
        } // Then move on to the next loop/line.
    }
    // Return the finished vector of String vectors.
    outer_vec
} // end of str_to_vecvecstring()
