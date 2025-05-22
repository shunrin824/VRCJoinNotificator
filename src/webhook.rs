use regex::Regex;
use reqwest::multipart::{self, Form, Part};
use std::{fs, path::PathBuf};
use tempfile;

#[path = "./function.rs"]
mod function;

#[path = "./image.rs"]
mod image;

//invite OR RequestInviteが来たことを解析する関数
pub async fn invite_format(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg_type: String = "none".to_string();
    let mut user_name: &str;
    if (content.contains("type: requestInvite")) {
        msg_type = "request invite".to_owned();
    } else if (content.contains("type: invite")) {
        msg_type = "invite".to_owned();
    }
    if (content.contains("username:")) {
        if let Some (captures) = Regex::new(r"(username:)(.*)(, sender user id:)")
            .unwrap()
            .captures(content)
        {
            user_name = &captures.get(2).map_or("", |m| m.as_str())
        }else{
            println!("不明なエラーが発生しました。 webhook.rs invite_format");
            return Ok(());
        }
        if (user_name.len() >= 2) {
            if msg_type == "invite" {
                discord_webhook_text(
                    "Invite".to_owned(),
                    format!("{}さんから招待が届きました。", user_name).to_owned(),
                )
                .await?;
            } else if msg_type == "request invite" {
                discord_webhook_text(
                    "ReqIn".to_owned(),
                    format!(
                        "{}さんがあなたのインスタンスに入りたがっています。",
                        user_name
                    )
                    .to_owned(),
                )
                .await?;
            }
        }
    }
    return Ok(());
}

//整形されたデータをdiscordに転送するだけの関数
pub async fn discord_webhook_send(form: Form) -> Result<(), Box<dyn std::error::Error>> {
    let url = function::config_read("discord_webhook_url");
    let client = reqwest::Client::new();
    if (url == "none") {
        println!("system: 不明なエラーが発生しました。discordへの送信処理をスキップします。\nerror: webhook.rs > discord_webhook");
    } else {
        let resp = client.post(url).multipart(form).send().await?;
    }
    return Ok(());
}

//discordに送信するためのテキストデータを整形する関数
pub async fn discord_webhook_text(
    actor_name: String,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let form = multipart::Form::new()
        .text("username", actor_name)
        .text("content", content);

    //Discordに送信する。
    discord_webhook_send(form).await?;
    return Ok(());
}

//discordに送信する画像データを整形する関数
pub async fn discord_webhook_file(
    world_name: &String,
    users_name: &Vec<String>,
    picture_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    //パスからファイル名を抽出
    let picture_name: String = picture_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let mut file_part: Option<Part> = None;

    //写真を読み込む。しかし、容量が大きい場合は縮小
    if let Ok(picture_metadata) = picture_path.metadata() {
        if picture_metadata.len() < 10 * 1000 * 1000 {
            let file = fs::read(&picture_path)?;
            file_part = Some(Part::bytes(file).file_name(picture_name.clone()));
        } else {
            if let Ok(dir) = tempfile::tempdir() {
                let converted_image_path = dir.path().join(&picture_name);
                if let Ok(_) = image::convert_png2webp(
                    &picture_path.to_str().unwrap(),
                    converted_image_path.to_str().unwrap(),
                    1920,
                ) {
                    let file = fs::read(converted_image_path)?;
                    file_part = Some(Part::bytes(file).file_name(picture_name.clone()));
                } else {
                    println!("不明なエラーが発生しました。discord_webhook_file");
                    return Ok(());
                }
            } else {
                println!("不明なエラーが発生しました。discord_webhook_file");
                return Ok(());
            };
        }
    } else {
        println!("不明なエラーが発生しました。discord_webhook_file");
        return Ok(());
    }

    //Discordに送信するデータを整形
    if let Some(upload_file_part) = file_part {
        let form = multipart::Form::new()
            .text(
                "content",
                format!(
                    "ワールド名: {}\n\nユーザー\n{}",
                    world_name,
                    users_name.join("\n")
                ),
            )
            .text("username", "写真")
            .part("file", upload_file_part);

        //Discordに送信する。
        discord_webhook_send(form).await?;
    } else {
        println!("不明なエラーが発生しました。discord_webhook_file");
    }
    return Ok(());
}
