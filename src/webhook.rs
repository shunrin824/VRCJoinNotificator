use base64;
use regex::Regex;
use reqwest::header::AUTHORIZATION;
use reqwest::multipart::{self, Form, Part};
use std::f32::consts::E;
use std::{env, fs, path::PathBuf};

#[path = "./function.rs"]
mod function;

//invite OR RequestInviteが来たことを解析する関数
pub async fn invite_format(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg_type: &str;
    let mut user: &str;
    let mut details: &str;
    let mut user_name: &str;
    if (content.contains("type: invite")) {
        msg_type = "invite";
    }
    if (content.contains("username:")) {
        match Regex::new(r"(username:)(.*)(, sender user id:)")
            .unwrap()
            .captures(content)
        {
            Some(captures) => user_name = &captures.get(2).map_or("", |m| m.as_str()),
            None => todo!(),
        }
        if (user_name.len() >= 2) {
            discord_webhook_text(
                "invite".to_owned(),
                format!("{}さんから招待が届きました。", user_name).to_owned(),
            )
            .await?;
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

    //写真を読み込む
    let file = fs::read(&picture_path)?;
    let file_part = Part::bytes(file).file_name(picture_name.clone());

    //Discordに送信するデータを整形
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
        .part("file", file_part);

    //Discordに送信する。
    discord_webhook_send(form).await?;
    return Ok(());
}
