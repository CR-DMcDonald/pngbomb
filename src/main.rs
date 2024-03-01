use image::{ImageBuffer, Rgba};
use std::path::Path;

fn main() {
    //set width and height based off command line arguements
    //read in the first command line arguement
    let width = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    //read in the second command line arguement
    let height = std::env::args().nth(2).unwrap().parse::<u32>().unwrap();    

    let img = ImageBuffer::from_fn(width, height, |_x, _y| Rgba([0u16, 0u16, 0u16, 0u16]));

    // Path where the image will be saved
    let path = Path::new("large_image.png");

    // Save the image to the specified path
    match img.save(path) {
        Ok(_) => println!("Successfully created and saved a large PNG image."),
        Err(e) => println!("Failed to save PNG image. Error: {}", e),
    };
}
