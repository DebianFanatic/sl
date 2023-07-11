/* display.rs */

pub fn display_image(image_vecvec: Vec<Vec<String>>) {
    let mut frame_num = 0;
    for each_frame in image_vecvec {
        println!("Frame {}:", frame_num);
        for each_line in each_frame {
            println!("{}", each_line);
        }
        frame_num += 1;
    }
} // end of display_image()
