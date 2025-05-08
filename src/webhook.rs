use base64;
use reqwest::header::AUTHORIZATION;
use reqwest::multipart::{self, Form, Part};
use std::f32::consts::E;
use std::{env, fs, path::PathBuf};

//整形されたデータをdiscordに転送するだけの関数
pub async fn discord_webhook_send(form: Form) -> Result<(), Box<dyn std::error::Error>> {
    let url = function::config_read("discord_webhook_url");
    let client = reqwest::Client::new();
    if (url == "none") {
        println!("system: 不明なエラーが発生しました。discordへの送信処理をスキップします。\nerror: webhook.rs > discord_webhook");
        return E;
    } else {
        let resp = client.post(url).multipart(form).send().await?;
    }
    return Ok(());
}

//discordに送信する画像データを整形する関数
pub async fn discord_webhook_file(
    world_name: String,
    users_name: Vec<String>,
    picture_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    //パスからファイル名を抽出
    let picture_name: String = picture_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    //写真を読み込む
    let file = fs::read(&picture_path)?;
    let file_part = Part::bytes(file)
        .file_name(picture_name.clone())
        .mime_str("image/png")?;

    //Discordに送信するデータを整形
    let form = Form::new()
        .text(
            "payload_json",
            format!(
                "ワールド名:{}\nユーザー一覧\n[{}]",
                world_name,
                users_name.join("]\n[")
            ),
        )
        .part("file", file_part);

    //Discordに送信する。
    discord_webhook_send(form).await?;
    return Ok(());
}
