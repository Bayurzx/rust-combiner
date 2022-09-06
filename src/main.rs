mod args;

// imports our args from args.rs file

use args::Args;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat, ImageError,
};
// use std::{fs::File, io::BufReader};

#[derive(Debug)]
enum ImageDataErr {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError),
    UnableToFormatImage(String)
}

struct FloatingImg {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImg {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4; // 3_655_744;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());

        FloatingImg {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErr> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErr::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErr> {
    let args = Args::new();
    let (img_1, img_format_1) = find_image_from_path(args.img_1)?;
    let (img_2, img_format_2) = find_image_from_path(args.img_2)?;

    if img_format_1 != img_format_2 {
        println!("Image format are not the same");
        return Err(ImageDataErr::DifferentImageFormats);
    }

    let (img_1, img_2) = standardize(img_1, img_2);
    let mut output = FloatingImg::new(img_1.width(), img_1.height(), args.output);

    let combined_data = combine_images(img_1, img_2);
    output.set_data(combined_data)?;

    if let Err(e) = image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        img_format_1,
    ) {
        Err(ImageDataErr::UnableToSaveImage(e))
    } else {

        Ok(())
    }


}

fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErr> {
    // let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    match Reader::open(&path) {
        Ok(image_reader) => {
            // let image_format: ImageFormat = image_reader.format().unwrap();
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErr::UnableToDecodeImage(e))
                }
            } else {
                return Err(ImageDataErr::UnableToFormatImage(path))
            }
            // (image, image_format)
        },
        Err(e) => {
            Err(ImageDataErr::UnableToReadImageFromPath(e))
        },
    }
}

fn get_smallest_dimension(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;

    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

// resize img after getting smallest dimensions
fn standardize(img_1: DynamicImage, img_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(img_1.dimensions(), img_2.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if img_2.dimensions() == (width, height) {
        (img_1.resize_exact(width, height, Triangle), img_2)
    } else {
        (img_1, img_2.resize_exact(width, height, Triangle))
    }
}

fn combine_images(img_1: DynamicImage, img_2: DynamicImage) -> Vec<u8> {
    let vec_1 = img_1.to_rgba8().into_vec();
    let vec_2 = img_2.to_rgba8().into_vec();

    print!("Succesfully wedded your pics\n");
    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combine_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combine_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combine_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }
    return combine_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();

    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("index is out of bounds"),
        };
        rgba.push(val);
    }

    return rgba
}
