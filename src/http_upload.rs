use reqwest::multipart::{self, Form, Part};
use std::error::Error;
use std::io::Write;
use std::{collections::HashMap, fs, path::PathBuf};
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

    let mut file = fs::read(&picture_path)?;
    let file_part = Part::bytes(file)
        .file_name(picture_name.clone())
        .mime_str("image/png")?;
    let form = Form::new()
        .text("writetype", "new")
        .text(
            "tag",
            format!("{}00and00{}", world_name, users_name.join("00and00")),
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
        .text("type", "vrc")
        .part("file", file_part);

    let client = reqwest::Client::new();
    let resp = client.post(url).multipart(form).send().await?;
    if !resp.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to send the request",
        )));
    }
    Ok(())
}
