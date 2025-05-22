use image::{io::Reader, DynamicImage, GenericImageView};
use std::{error::Error, fs::File, io::Write};
use webp::Encoder;

pub fn convert_png2webp(
    input_path: &str,
    output_path: &str,
    output_max_reso: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    //アスペクト比が16:9でない可能性があるので、そこを柔軟に行うプログラムを書いてください。
    //アップロード処理、テンポラリーに変換ファイルを置く処理などは書いてあるはずです。
    //長辺から出ないようにお願いします。

    let mut resize_image: Option<DynamicImage> = None;
    //画像を読み込む
    let origin_image = image::ImageReader::open(input_path)?.decode()?;
    if origin_image.width() == origin_image.height() {
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
        //元画像をRGBAデータに変換
        let rgba_image = resized_image.to_rgba8();

        //ロスレスwebp
        let webp_image = Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height())
            .encode_lossless();

        //ファイルに出力
        File::create(output_path)?.write_all(&webp_image)?;
    }

    Ok(())
}
