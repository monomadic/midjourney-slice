extern crate image;

use image::{GenericImage, GenericImageView, ImageBuffer};
use std::path::PathBuf;

fn main() {
    let file = if std::env::args().count() == 2 {
        std::env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let path = PathBuf::from(&file);

    let img = image::open(&path).unwrap();

    let (width, height) = img.dimensions();
    let half_width = width / 2;
    let half_height = height / 2;

    let mut top_left = ImageBuffer::new(half_width, half_height);
    let mut top_right = ImageBuffer::new(half_width, half_height);
    let mut bottom_left = ImageBuffer::new(half_width, half_height);
    let mut bottom_right = ImageBuffer::new(half_width, half_height);

    for x in 0..half_width {
        for y in 0..half_height {
            let pixel = img.get_pixel(x, y);
            top_left.put_pixel(x, y, pixel);
            top_right.put_pixel(x, y, img.get_pixel(x + half_width, y));
            bottom_left.put_pixel(x, y, img.get_pixel(x, y + half_height));
            bottom_right.put_pixel(x, y, img.get_pixel(x + half_width, y + half_height));
        }
    }

    let base_name: &str = path.file_stem().unwrap().to_str().unwrap();
    top_left.save(format!("{}-01.jpg", base_name)).unwrap();
    top_right.save(format!("{}-02.jpg", base_name)).unwrap();
    bottom_left.save(format!("{}-03.jpg", base_name)).unwrap();
    bottom_right.save(format!("{}-04.jpg", base_name)).unwrap();
}
