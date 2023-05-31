extern crate image;

use image::{GenericImage, GenericImageView, ImageBuffer};
use std::path::PathBuf;

/// Creates a quadrant of the original image starting from `start_x` and `start_y` up to `end_x` and `end_y`.
///
/// # Arguments
///
/// * `img` - A reference to the original image.
/// * `start_x` - The starting x-coordinate.
/// * `start_y` - The starting y-coordinate.
/// * `end_x` - The ending x-coordinate.
/// * `end_y` - The ending y-coordinate.
///
/// # Returns
///
/// * `ImageBuffer` - An ImageBuffer object representing the quadrant.
///
fn create_and_copy_quadrant(
    img: &image::DynamicImage,
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let mut quadrant = ImageBuffer::new(end_x - start_x, end_y - start_y);
    for x in start_x..end_x {
        for y in start_y..end_y {
            quadrant.put_pixel(x - start_x, y - start_y, img.get_pixel(x, y));
        }
    }
    quadrant
}

/// Main function that reads an image file, splits the image into four quadrants,
/// and saves each quadrant as a separate image file.
///
/// The image file is provided as an argument when running the program.
///
/// Each quadrant image file is saved with the original filename appended with a "-01", "-02", "-03", or "-04".
///
fn main() {
    let (file, ext) = match std::env::args().count() {
        2 => (std::env::args().nth(1).unwrap(), String::from("jpg")),
        3 => (
            std::env::args().nth(1).unwrap(),
            std::env::args().nth(2).unwrap(),
        ),
        _ => {
            panic!("usage: <file> [ext]");
        }
    };

    let path = PathBuf::from(&file);

    let img = image::open(&path).unwrap_or_else(|_| {
        panic!("file not found: {:?}", &path);
    });

    let (width, height) = img.dimensions();
    let half_width = width / 2;
    let half_height = height / 2;

    let top_left = create_and_copy_quadrant(&img, 0, 0, half_width, half_height);
    let top_right = create_and_copy_quadrant(&img, half_width, 0, width, half_height);
    let bottom_left = create_and_copy_quadrant(&img, 0, half_height, half_width, height);
    let bottom_right = create_and_copy_quadrant(&img, half_width, half_height, width, height);

    let base_name: &str = path.file_stem().unwrap().to_str().unwrap();
    top_left.save(format!("{}-01.{}", base_name, ext)).unwrap();
    top_right.save(format!("{}-02.{}", base_name, ext)).unwrap();
    bottom_left
        .save(format!("{}-03.{}", base_name, ext))
        .unwrap();
    bottom_right
        .save(format!("{}-04.{}", base_name, ext))
        .unwrap();
}
