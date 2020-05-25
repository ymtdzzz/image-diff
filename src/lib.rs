use anyhow::Error;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::cmp::max;

pub type Result<T> = std::result::Result<T, Error>;

pub fn diff(before: &DynamicImage, after: &DynamicImage) -> Result<DynamicImage> {
    let (after_width, after_height) = after.dimensions();
    let (before_width, before_height) = before.dimensions();
    let width = max(after_width, before_width);
    let height = max(after_height, before_height);
    let mut result = DynamicImage::new_rgba16(width, height);

    for y in 0..height {
        for x in 0..width {
            let new_color: [u8; 4];
            let pixel: Rgba<u8>;
            if x >= before_width
                || y >= before_height
                || x >= after_width
                || y >= after_height
            {
                new_color = [255, 0, 0, 255];
                pixel = Rgba(new_color);
            } else {
                let before_pixel: Rgba<u8> = before.get_pixel(x, y);
                let after_pixel: Rgba<u8> = after.get_pixel(x, y);
                let alpha = before_pixel[3];

                let is_diff = before_pixel[0] != after_pixel[0]
                    || before_pixel[1] != after_pixel[1]
                    || before_pixel[2] != after_pixel[2];

                let mut new_red = after_pixel[0];
                let mut new_green = after_pixel[1];
                let mut new_blue = after_pixel[2];
                if is_diff {
                    new_red = 255;
                    new_green = 0;
                    new_blue = 0;
                }

                new_color = [new_red, new_green, new_blue, alpha];
                pixel = Rgba(new_color);

            }
            result.put_pixel(x, y, pixel);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        let before = image::open("test/before.png");
        let after = image::open("test/after.png").unwrap();
        let dif = diff(&before.unwrap(), &after);
        assert_eq!(true, dif.is_ok());
        let before = image::open("test/before.png");
        let after = image::open("test/before_big.png");
        let dif = diff(&before.unwrap(), &after.unwrap());
        assert_eq!(true, dif.is_ok());
        let before = image::open("test/before_small.png");
        let after = image::open("test/before.png");
        let dif = diff(&before.unwrap(), &after.unwrap());
        assert_eq!(true, dif.is_ok());
    }
}
