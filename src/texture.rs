use std::fs::read_dir;

use image::{io::Reader, Rgb};
use once_cell::sync::Lazy;

use crate::Image;

const TEXTURE_SIZE: u32 = 256;

pub static TEXTURES: Lazy<Vec<(Image, String)>> = Lazy::new(||
    read_dir("./textures").unwrap().into_iter().map(|path| {
        let image = Reader::open(path.as_ref().unwrap().path())
        .unwrap()
        .decode()
        .unwrap()
        .as_rgb8()
        .unwrap()
        .clone();

        println!("{:?}", path.as_ref().unwrap().path().file_name().unwrap());

        let mut file_name = path.unwrap().file_name().into_string().unwrap();

        for _ in 0..4 {
            file_name.pop();
        }

        (image, file_name)
    }).collect()
);

pub static AVERAGE_COLOURS: Lazy<Vec<(Rgb<u8>, String)>> = Lazy::new(||
    TEXTURES.iter().map(|texture| {
        let added = texture.0.pixels().fold((0u32, 0u32, 0u32), |acc, pixel|
            (acc.0 + pixel.0[0] as u32, acc.1 + pixel.0[1] as u32, acc.2 + pixel.0[2] as u32)
        );

        // (Colour::new((added.0 / length) as u8, (added.1 / length) as u8, (added.2 / length) as u8), texture.1.clone())
        (Rgb::from([(added.0 / TEXTURE_SIZE) as u8, (added.1 / TEXTURE_SIZE) as u8, (added.2 / TEXTURE_SIZE) as u8]), texture.1.clone())
    }).collect()
);

