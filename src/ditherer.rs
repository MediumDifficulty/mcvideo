use image::Rgb;

use crate::{Image, texture::AVERAGE_COLOURS};

type ColourTuple = (f64, f64, f64);

pub fn dither(image: &Image, enabled: bool) -> Vec<&'static String> {
    let mut new_image: Vec<ColourTuple> = image.chunks(3)
        .map(|pixel| (pixel[0] as f64, pixel[1] as f64, pixel[2] as f64))
        .collect();

    if enabled {
        let width = image.width() as usize;

        for y in (0..image.height() as usize).rev() {
            for x in 0..image.width() as usize {
                let old_pixel = rgb_to_tuple(image.get_pixel(x as u32, y as u32));
                let new_pixel = find_closest(old_pixel);
                let quant_error = subtract_tuple(old_pixel, new_pixel);


                add_pixel(&mut new_image, width, x, y, multiply_tuple(quant_error, 7.0 / 16.0));
                add_pixel(&mut new_image, width, x, y, multiply_tuple(quant_error, 3.0 / 16.0));
                add_pixel(&mut new_image, width, x, y, multiply_tuple(quant_error, 5.0 / 16.0));
                add_pixel(&mut new_image, width, x, y, multiply_tuple(quant_error, 1.0 / 16.0));
            }
        }
    }

    new_image.iter().map(|pixel| find_closest_texture(*pixel)).collect()
}

fn add_pixel(image: &mut Vec<ColourTuple>, width: usize, x: usize, y: usize, value: ColourTuple) {
    let index = index(width, x, y);
    let old_pixel = image[index];

    set_pixel(image, width, x, y, add_tuple(old_pixel, value))
}

fn set_pixel(image: &mut Vec<ColourTuple>, width: usize, x: usize, y: usize, value: ColourTuple) {
    let index = index(width, x, y);
    if index < image.len() {
        image[index].0 = value.0;
        image[index].1 = value.1;
        image[index].2 = value.2;
    }
}

fn index(width: usize, x: usize, y: usize) -> usize {
    width * y + x
}

fn subtract_tuple(l: ColourTuple, r: ColourTuple) -> ColourTuple {
    (
        l.0 - r.0,
        l.1 - r.1,
        l.2 - r.2,
    )
}

fn multiply_tuple(l: ColourTuple, r: f64) -> ColourTuple {
    (
        l.0 * r,
        l.1 * r,
        l.2 * r,
    )
}

fn add_tuple(l: ColourTuple, r: ColourTuple) -> ColourTuple {
    (
        l.0 + r.0,
        l.1 + r.1,
        l.2 + r.2,
    )
}

fn find_closest(colour: ColourTuple) -> ColourTuple {
    let mut best_colour = rgb_to_tuple(&AVERAGE_COLOURS[0].0);
    let mut best_dist = f64::INFINITY;

    AVERAGE_COLOURS.iter().for_each(|test_colour| {
        let dist = dist_sq(colour, rgb_to_tuple(&test_colour.0));
        if dist < best_dist {
            best_dist = dist;
            best_colour = rgb_to_tuple(&test_colour.0);
        }
    });

    (best_colour.0, best_colour.0, best_colour.0)
}

fn find_closest_texture(colour: ColourTuple) -> &'static String {
    let mut best_dist = f64::INFINITY;
    let mut best_colour = rgb_to_tuple(&AVERAGE_COLOURS[0].0);
    let mut best_texture = &AVERAGE_COLOURS[0].1;

    AVERAGE_COLOURS.iter().for_each(|test_colour| {
        let dist = dist_sq(colour, rgb_to_tuple(&test_colour.0));
        if dist < best_dist {
            best_dist = dist;
            best_colour = rgb_to_tuple(&test_colour.0);
            best_texture = &test_colour.1;
        }
    });

    best_texture
}

fn dist_sq(colour: ColourTuple, other: ColourTuple) -> f64 {
    let a = (
        other.0 - colour.0,
        other.1 - colour.1,
        other.2 - colour.2
    );
        
    let r = a.0;
    let g = a.1;
    let b = a.2;

    r*r + g*g + b*b
}

fn rgb_to_tuple(rgb: &Rgb<u8>) -> ColourTuple {
    (rgb.0[0] as f64, rgb.0[1] as f64, rgb.0[2] as f64)
}