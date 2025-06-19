use image::{io::Reader, DynamicImage, GenericImageView};
use std::{error::Error, fs::File, io::Write};
use webp::Encoder;

use super::function;

//10MB未満に収まるように画像のリサイズを試行
pub fn less10mb_webp(input_path: &str, output_path: &str,mut output_max_reso: u32){
    let mut output_size;
    function::debug_print("lossless");
        convert_png2webp(input_path, output_path, output_max_reso, 0.0);
        output_size = File::open(output_path).unwrap().metadata().unwrap().len();
        if output_size < 10*1000*1000-1{
            return;
        }
        let mut output_quality :f32 = 100.0;
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
    //アスペクト比が16:9でない可能性があるので、そこを柔軟に行うプログラムを書いてください。
    //アップロード処理、テンポラリーに変換ファイルを置く処理などは書いてあるはずです。
    //長辺から出ないようにお願いします。

    let mut resize_image: Option<DynamicImage> = None;
    //画像を読み込む
    let origin_image = image::ImageReader::open(input_path)?.decode()?;
    function::debug_print(output_max_reso.to_string().as_str());
    if origin_image.width() <= output_max_reso && origin_image.height() <= output_max_reso || output_max_reso == 0 {
        function::debug_print("画像コピー");
        resize_image = Some(origin_image);
    } else if origin_image.width() == origin_image.height() {
        //正方形の場合のリサイズ
        resize_image = Some(origin_image.resize_exact(
            output_max_reso,
            output_max_reso,
            image::imageops::FilterType::Lanczos3,
        ));
    } else if origin_image.width() > origin_image.height() {
        //横長の場合のリサイズ
        resize_image = Some(origin_image.resize_exact(
            output_max_reso,
            (origin_image.height() as f32 / origin_image.width() as f32 * output_max_reso as f32)
                as u32,
            image::imageops::FilterType::Lanczos3,
        ));
    } else if origin_image.height() > origin_image.width() {
        //縦長の場合のリサイズ
        resize_image = Some(origin_image.resize_exact(
            (origin_image.width() as f32 / origin_image.height() as f32 * output_max_reso as f32)
                as u32,
            output_max_reso,
            image::imageops::FilterType::Lanczos3,
        ));
    }

    if let Some(resized_image) = resize_image {
        function::debug_print(format!("{}x{}", resized_image.width(), resized_image.height()).as_str());
        //元画像をRGBAデータに変換
        let rgba_image = resized_image.to_rgba8();
        let webp_image: webp::WebPMemory;
        if output_quality == 0.0 {
            //可逆webp
            webp_image = Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height())
                .encode_lossless();
        } else {
            //不可逆webp
            webp_image = Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height())
                .encode(output_quality);
        }
        //ファイルに出力
        function::debug_print(output_path);
        File::create(output_path)?.write_all(&webp_image)?;
    }

    Ok(())
}
