use base64;
use reqwest::header::AUTHORIZATION;
use reqwest::multipart::{Form, Part};
use std::{fs, path::PathBuf};

// SDMS (Shunrin Data Management System) へのデータ送信処理
#[path = "./function.rs"]
mod function;
#[path = "./webhook.rs"]
mod webhook;

#[derive(Clone)]
pub struct UploadData {
    pub users_name: Vec<String>,
    pub file_path: PathBuf,
    pub world_name: String,
}

// SDMSサーバーにデータを送信
pub async fn idms_send(form: Form) -> Result<(), Box<dyn std::error::Error>> {
    let url = function::config_read("idms_server_url");
    let client = reqwest::Client::new();
    if (function::config_read("idms_server_auth_username") == "none") {
        let resp = client.post(url).multipart(form).send().await?;
    } else {
        let idms_auth = format!(
            "Basic {}",
            base64::encode(&format!(
                "{}:{}",
                function::config_read("idms_server_auth_username"),
                function::config_read("idms_server_auth_password")
            ))
        );
        let resp = client
            .post(url)
            .header(AUTHORIZATION, idms_auth)
            .multipart(form)
            .send()
            .await?;
    }
    return Ok(());
}

// SDMSにVRChatログを送信
pub async fn idms_log_send(log_lines: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if !function::config_read("idms_server_url").contains("none") {
        println!("System: ログの送信を開始します。");
        let memo: String = log_lines.join("\n");
        let form = Form::new()
            .text("writetype", "new")
            .text("tag", "vrc log_data")
            .text("type", "txt")
            .text("memo", memo);
        idms_send(form).await?;
        println!("System: ログの送信が完了しました。");
    }
    return Ok(());
}

// SDMSにスクリーンショットをアップロード
async fn idms_file_send(
    world_name: &String,
    users_name: &Vec<String>,
    picture_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    if function::config_read("idms_server_url").len() >= 1
        && !function::config_read("idms_server_url").contains("none")
    {
        let picture_name: String = picture_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        let file = fs::read(&picture_path)?;
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
        idms_send(form).await?;
    } else {
        function::debug_print("SDMS URLが設定されていません。");
    }
    return Ok(());
}

// スクリーンショットデータをSDMSおよびDiscordに送信
pub async fn pictures_upload(datas: Vec<UploadData>) -> Result<(), Box<dyn std::error::Error>> {
    function::debug_print("アップロード処理を開始します。");
    for data in datas {
        if function::config_read("idms_server_url").len() >= 1
            && !function::config_read("idms_server_url").contains("none")
        {
            idms_file_send(&data.world_name, &data.users_name, &data.file_path).await?;
        }

        if function::config_read("discord_webhook_url").len() >= 1
            && !function::config_read("discord_webhook_url").contains("none")
        {
            webhook::discord_webhook_file(&data.world_name, &data.users_name, &data.file_path).await?;
        }
    }
    return Ok(());
}
