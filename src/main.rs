use image::{io::Reader as ImageReader, GenericImageView, ImageResult, DynamicImage};
use std::io::Cursor;
use std::env::args;

extern crate image;


fn has_transparency(img: DynamicImage) -> Result<bool, Box<dyn std::error::Error>> {
    let mut is_transparent: bool = false;
    
    for pixel in img.pixels() {
        let pixel = pixel.2;
        if pixel[3] == 0 {
            is_transparent = true;
        }
    }

    Ok(is_transparent)
}


fn get_image_from_url(url: &str) -> ImageResult<DynamicImage> {
    let resp_wrapped = reqwest::blocking::get(url);

    if resp_wrapped.is_err() {
        return Err(image::ImageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get image from url")));
    }

    let mut resp = resp_wrapped.unwrap();

    let mut buf = Vec::new();
    resp.copy_to(&mut buf).unwrap();
    let image = ImageReader::new(Cursor::new(buf)).with_guessed_format().unwrap().decode().unwrap();
    Ok(image)
}


fn main() {
    let argument = args().nth(1).unwrap().replace("'", "").replace("\"", "").replace(" ", "");

    if argument == "" {
        println!("No URL provided");
        return;
    }

    let img_wrapped = get_image_from_url(&argument);

    if img_wrapped.is_err() {
        println!("Cannot decode image");
        return;
    }

    let img = img_wrapped.unwrap();

    if has_transparency(img).unwrap() {
        println!("Image has transparency");
    } else {
        println!("Image does not have transparency");
    }
}
