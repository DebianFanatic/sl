/* convert_to_vec.rs */

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
