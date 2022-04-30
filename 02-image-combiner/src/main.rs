use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageError,
    ImageFormat,
};
use itertools::concat;

mod args;

#[derive(Debug)]
enum ImageDataError {
    BufferTooSmall,
    DifferentImageFormats,
    UnableToDecodeImage(ImageError),
    UnableToFormatImage(String),
    UnableToReadImageFromPath(String),
    UnableToSaveImage(ImageError),
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = width * height * 4;
        let data = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        Self {
            width,
            height,
            data,
            name,
        }
    }
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataError> {
        if data.len() != self.data.capacity() {
            return Err(ImageDataError::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataError> {
    let args = args::Args::new();
    let (image1, image_format1) = find_image_from_path(args.image1)?;
    let (image2, image_format2) = find_image_from_path(args.image2)?;

    if image_format1 != image_format2 {
        return Err(ImageDataError::DifferentImageFormats);
    }

    let (image1, image2) = standardize_sizes(image1, image2);
    let mut output_image = FloatingImage::new(image1.width(), image1.height(), args.output);
    let combined_data = combine_images(image1, image2);

    output_image.set_data(combined_data)?;
    save_image(&output_image, image_format1)
}

fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataError> {
    match Reader::open(&path) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(error) => Err(ImageDataError::UnableToDecodeImage(error)),
                }
            } else {
                Err(ImageDataError::UnableToFormatImage(path))
            }
        }
        Err(_) => Err(ImageDataError::UnableToReadImageFromPath(path)),
    }
}

fn get_smallest_dimensions(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let (width1, height1) = dim1;
    let (width2, height2) = dim2;

    if width1 < width2 && height1 < height2 {
        (width1, height1)
    } else if width1 < width2 && height1 > height2 {
        (width1, height2)
    } else if width1 > width2 && height1 < height2 {
        (width2, height1)
    } else {
        (width2, height2)
    }
}

fn standardize_sizes(image1: DynamicImage, image2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image1.dimensions(), image2.dimensions());
    (
        image1.resize_exact(width, height, Triangle),
        image2.resize_exact(width, height, Triangle),
    )
}

fn combine_images(image1: DynamicImage, image2: DynamicImage) -> Vec<u8> {
    let vec1 = image1.to_rgba8().into_vec();
    let vec2 = image2.to_rgba8().into_vec();

    alternate_pixels(vec1, vec2)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    let mut vec = Vec::new();
    let mut i = 0;

    while i < vec1.len() {
        if i % 8 == 0 {
            vec = concat([vec, vec1[i..=i + 3].to_vec()]);
        } else {
            vec = concat([vec, vec2[i..=i + 3].to_vec()]);
        }
        i += 4;
    }

    vec
}

fn save_image(image: &FloatingImage, image_format: ImageFormat) -> Result<(), ImageDataError> {
    if let Err(error) = image::save_buffer_with_format(
        &image.name,
        &image.data,
        image.width,
        image.height,
        image::ColorType::Rgba8,
        image_format,
    ) {
        Err(ImageDataError::UnableToSaveImage(error))
    } else {
        Ok(())
    }
}
