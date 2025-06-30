use image::{io::Reader, DynamicImage, GenericImageView};
use std::{error::Error, fs::File, io::Write};
use webp::Encoder;

use super::function;

// Discordの10MB制限に合わせて画像をWebP形式でリサイズ・圧縮
pub fn less10mb_webp(input_path: &str, output_path: &str, mut output_max_reso: u32) {
    let mut output_size;
    function::debug_print("lossless");
        convert_png2webp(input_path, output_path, output_max_reso, 0.0);
        output_size = File::open(output_path).unwrap().metadata().unwrap().len();
        if output_size < 10*1000*1000-1{
            return;
        }
        let mut output_quality: f32 = 100.0;
    while output_size > 10*1000*1000-1 {
        function::debug_print(output_quality.to_string().as_str());
        convert_png2webp(input_path, output_path, output_max_reso, output_quality);
        output_size = File::open(output_path).unwrap().metadata().unwrap().len();
        output_quality -= 10.0;
    }
    return;
}

pub fn convert_png2webp(
    input_path: &str,
    output_path: &str,
    output_max_reso: u32,
    output_quality: f32,
) -> Result<(), Box<dyn std::error::Error>> {

    let mut resize_image: Option<DynamicImage> = None;
    let origin_image = image::ImageReader::open(input_path)?.decode()?;
    function::debug_print(output_max_reso.to_string().as_str());
    if origin_image.width() <= output_max_reso && origin_image.height() <= output_max_reso || output_max_reso == 0 {
        function::debug_print("画像コピー");
        resize_image = Some(origin_image);
    } else if origin_image.width() == origin_image.height() {
        resize_image = Some(origin_image.resize_exact(
            output_max_reso,
            output_max_reso,
            image::imageops::FilterType::Lanczos3,
        ));
    } else if origin_image.width() > origin_image.height() {
        resize_image = Some(origin_image.resize_exact(
            output_max_reso,
            (origin_image.height() as f32 / origin_image.width() as f32 * output_max_reso as f32)
                as u32,
            image::imageops::FilterType::Lanczos3,
        ));
    } else if origin_image.height() > origin_image.width() {
        resize_image = Some(origin_image.resize_exact(
            (origin_image.width() as f32 / origin_image.height() as f32 * output_max_reso as f32)
                as u32,
            output_max_reso,
            image::imageops::FilterType::Lanczos3,
        ));
    }

    if let Some(resized_image) = resize_image {
        function::debug_print(format!("{}x{}", resized_image.width(), resized_image.height()).as_str());
        let rgba_image = resized_image.to_rgba8();
        let webp_image: webp::WebPMemory;
        if output_quality == 0.0 {
            webp_image = Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height())
                .encode_lossless();
        } else {
            webp_image = Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height())
                .encode(output_quality);
        }
        function::debug_print(output_path);
        File::create(output_path)?.write_all(&webp_image)?;
    }

    Ok(())
}
