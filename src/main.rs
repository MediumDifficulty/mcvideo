pub mod ditherer;
pub mod texture;

use std::{process::Command, fs::{create_dir_all, read_dir, File, DirEntry}, io::Write, env};

use datapack::builder::{DataPackBuilder, structure::Structure, mcfunction::MCFunction};
use flate2::{Compression, write::GzEncoder};
use image::io::Reader;
use image::ImageBuffer;
use image::Rgb;
use nbt::{CompoundTag, encode::write_compound_tag, Tag};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

// const pos: (u32, u32, u32) = (0, 128, 0);
// const name: &str = "video";
// const dithered: bool = true;
// const dimensions: &str = "320x180";

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let input = &args[1];
    let name = &args[2];
    let dithered: bool = args[3].parse().unwrap();
    let dimensions = &args[4];
    let pos: (usize, usize, usize) = (
        args[5].parse().unwrap(),
        args[6].parse().unwrap(),
        args[7].parse().unwrap()
    );

    create_dir_all("frames").unwrap();
    
    println!("Extracting frames...");
    Command::new("ffmpeg")
        .args(["-i", input.as_str(), "-vf", format!("fps=fps=20, scale={}", dimensions).as_str(), "frames\\%04d.png"])
        .output()
        .unwrap()
        .stderr
        .iter()
        .map(|ele| *ele as char)
        .for_each(|character| print!("{}", character));


    let mut frame_path_reader: Vec<DirEntry> = read_dir("./frames").unwrap()
        .map(|p| p.unwrap())
        .collect();
    
    frame_path_reader.sort_by_key(|dir| dir.path());

    let mut frames: Vec<Image> = Vec::new();

    for path in frame_path_reader {
        frames.push(
            Reader::open(path.path())
                .unwrap()
                .decode()
                .unwrap()
                .as_rgb8()
                .unwrap()
                .clone()
            )
        }
        
        let width = frames[0].width() as usize;
    
        println!("Dithering...");
        let dithered: Vec<Vec<&String>> = frames.par_iter()
        .map(|frame| ditherer::dither(&frame, dithered) )
        .collect();
        
        
    println!("Creating structures...");
    let structures: Vec<Vec<u8>> = dithered.par_iter()
        .enumerate()
        .map(|(i, frame)| {
            let last_frame = if i > 0 {
                Some(&dithered[i - 1])
            } else {
                None
            };
            
            let delta_frame: Vec<Option<&String>> = frame.iter().enumerate().map(|(i, pixel)| {
                if last_frame.is_none() || last_frame.unwrap()[i] != *pixel {
                    Some(*pixel)
                } else {
                    None
                }
            }).collect();

            let mut palette = Vec::new();
            for texture in &delta_frame {
                if texture.is_some() && !palette.contains(texture.unwrap()) {
                    palette.push(texture.unwrap().to_string())
                }
            }
            
            let mut blocks: Vec<CompoundTag> = Vec::new();
            delta_frame.iter().enumerate().for_each(|(i, pixel)| {
                if pixel.is_some() {
                    let mut tag = CompoundTag::new();

                    tag.insert("pos", Tag::List(vec![Tag::Int((i % width) as i32), Tag::Int(0), Tag::Int((i / width) as i32)]));
                    tag.insert_i32("state", palette.iter().position(|texture| texture == pixel.unwrap()).unwrap() as i32);
    
                    blocks.push(tag)
                }
            });

            let palette_tag: Vec<CompoundTag> = palette.iter().map(|ele| {
                let mut tag = CompoundTag::new();
                tag.insert_str("Name", format!("minecraft:{}", ele).as_str());
                
                tag
            }).collect();

            
            let mut structure_tag = CompoundTag::new();
            structure_tag.insert_i32("DataVersion", 3105);
            structure_tag.insert_compound_tag_vec("entities", Vec::new());
            structure_tag.insert("size", Tag::List(vec![Tag::Int(10), Tag::Int(10), Tag::Int(10)]));
            structure_tag.insert_compound_tag_vec("blocks", blocks);
            structure_tag.insert_compound_tag_vec("palette", palette_tag);


            let mut buf = Vec::new();
            write_compound_tag(&mut buf, &structure_tag).unwrap();

            let mut gzipped = GzEncoder::new(Vec::new(), Compression::default());
            gzipped.write_all(&buf.as_slice()).unwrap();

            gzipped.finish().unwrap()

        })
        .collect();
    

    let mut datapack = DataPackBuilder::new();
    datapack.set_name(name.to_string());
    
    println!("Adding elements to datapack...");
    structures.iter().enumerate().for_each(|(i, structure)| {
        let mut function_contents = format!(
"setblock {} {} {} structure_block{{\"name\": \"{}:frame_{}\", \"mode\": \"LOAD\", \"posX\": 1}}
setblock {} {} {} redstone_block
fill {} {} {} {} {} {} air",
            pos.0, pos.1, pos.2, name, i,
            pos.0, pos.1 + 1, pos.2,
            pos.0, pos.1 + 1, pos.2, pos.0, pos.1, pos.2
);
        if i < structures.len() - 1 {
            function_contents.push_str(format!("\nschedule function {}:frame_{} 1t", name, i + 1).as_str())
        }

        datapack.add_structure(Structure::new(&structure, &format!("frame_{}", i)))
            .add_mc_function(MCFunction::new(&function_contents, &format!("frame_{}", i), false, false));
    });


    let pack_file = File::create(format!("{}.zip", name)).unwrap();

    println!("Building datapack...");
    datapack.build(&pack_file);


    ()
}
