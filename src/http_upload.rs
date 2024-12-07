use reqwest::multipart::{self, Form, Part};
use std::error::Error;
use std::{collections::HashMap, path::PathBuf};
use tokio;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct UploadData {
    pub(crate) users_name: Vec<String>,
    pub(crate) file_path: PathBuf,
    pub(crate) world_name: String,
}

pub async fn pictures_upload(datas: Vec<UploadData>) -> Result<(), Box<dyn std::error::Error>> {
    for data in datas {
        http_send(data.world_name, data.users_name, data.file_path).await?;
    }
    Ok(())
}

pub async fn http_send(
    world_name: String,
    users_name: Vec<String>,
    picture_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://192.168.1.132/idms/datawrite.php";

    let picture_name: String = picture_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let mut file: File = File::open(picture_path).await?;
    let mut file_content: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_content).await?;
    let file_part: Part = Part::bytes(file_content).file_name(picture_name.clone());

    let form = Form::new()
        .text(
            "tag",
            (world_name + "\n" + &users_name.join("\n")).to_string(),
        )
        .text(
            "date",
            format!(
                "{}{}{}{}{}{}",
                &picture_name[7..11],
                &picture_name[12..14],
                &picture_name[15..17],
                &picture_name[18..20],
                &picture_name[21..23],
                &picture_name[24..26]
            ),
        )
        .part("file", file_part);

    let client = reqwest::Client::new();
    let resp = client.post(url).multipart(form).send().await?;
    Ok(())
}
