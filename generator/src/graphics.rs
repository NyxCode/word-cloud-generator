use image::{RgbaImage, DynamicImage, Rgba, Rgb};
use std::f32::consts::PI;
use imageproc::affine::Interpolation;
use crate::log_time;
use crate::cli::CLIOptions;

pub fn rotate(settings: &CLIOptions, image: RgbaImage) -> RgbaImage {
    let _log = log_time("rotating");

    let angle = settings.post_processing_rotation * PI / 180.0;

    // calculate the size of the image
    let (new_width, new_height) = {
        let (width, height) = (image.width() as f32, image.height() as f32);
        (
            (width * angle.cos().abs() + height * angle.sin().abs()) as u32,
            (height * angle.cos().abs() + width * angle.sin().abs()) as u32,
        )
    };

    let mut new_image = DynamicImage::new_rgba8(new_width, new_height).to_rgba();
    let (offset_x, offset_y) = (new_width - image.width(), new_height - image.height());

    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            if pixel.data[3] == 0 {
                continue;
            }
            let (target_x, target_y) = (x + offset_x / 2, y + offset_y / 2);
            new_image.put_pixel(target_x, target_y, *pixel);
        }
    }

    imageproc::affine::rotate_with_default(&new_image,
                                           (new_width as f32 / 2.0, new_height as f32 / 2.0),
                                           angle,
                                           Rgba([0, 0, 0, 0]),
                                           Interpolation::Bilinear)
}

pub fn set_background(image: &mut RgbaImage, color: &Rgb<u8>) {
    let _log = log_time("setting background");

    let (bg_r, bg_g, bg_b, bg_a) = (color[0] as f32 / 255.0,
                                    color[1] as f32 / 255.0,
                                    color[2] as f32 / 255.0,
                                    255);

    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            let (r, g, b, a) = (
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0,
                pixel[3] as f32 / 255.0,
            );
            let (res_r, res_g, res_b) = (a * r + (1.0 - a) * bg_r,
                                         a * g + (1.0 - a) * bg_g,
                                         a * b + (1.0 - a) * bg_b);
            image.put_pixel(x, y, Rgba([
                (res_r * 255.0) as u8,
                (res_g * 255.0) as u8,
                (res_b * 255.0) as u8,
                bg_a
            ]));
        }
    }
}