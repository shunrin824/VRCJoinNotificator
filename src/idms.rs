use base64;
use reqwest::header::AUTHORIZATION;
use reqwest::multipart::{self, Form, Part};
use std::{fs, path::PathBuf};
//idmsとsdmsは同一のプロジェクトです。表記ゆれがありますが、sdmsへ統一予定です。
//Shunrin Data Management Systemの略です。
#[path = "./function.rs"]
mod function;

pub struct UploadData {
    pub users_name: Vec<String>,
    pub file_path: PathBuf,
    pub world_name: String,
}

//idmsに実際にデータを送信する関数
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
    println!("idms_send");
    return Ok(())
}

//idmsに簡易ログを送信する関数
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
    return Ok(())
}

//idmsに写真を送信する関数
async fn idms_file_send(
    world_name: String,
    users_name: Vec<String>,
    picture_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    if function::config_read("idms_server_url").len() >= 1 && !function::config_read("idms_server_url").contains("none") {
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
    }else{
        print!("degbug idmsのurlが見つかりません。");
    }
    return Ok(())
}

//複数のデータをそれぞれidms_file_send()に送る関数
pub async fn pictures_upload(datas: Vec<UploadData>) -> Result<(), Box<dyn std::error::Error>> {
    for data in datas {
        idms_file_send(data.world_name, data.users_name, data.file_path).await?;
    }
    return Ok(())
}
