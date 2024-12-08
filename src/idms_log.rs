use reqwest::multipart::{self, Form, Part};
use std::error::Error;
use std::io::Write;
use std::string;
use std::{collections::HashMap, fs, path::PathBuf};
use tokio;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::function;
use crate::log_read;

fn log_print(log_line: String, log_content: String) -> String {
    return format!(
        "{}/{}/{} {} {}",
        &log_line[0..4],
        &log_line[5..7],
        &log_line[8..10],
        &log_line[11..19],
        &log_content
    ); //最初の4つは日時の表示。
}

async fn http_send(log_lines: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://192.168.1.132/idms/datawrite.php";
    let memo: String = log_lines.join("\n");
    let form = Form::new()
        .text("writetype", "new")
        .text("tag", "vrc log_data")
        .text("type", "txt")
        .text("memo", memo);

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

pub async fn send_idms_log(log_file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut log_lines: Vec<String> = Vec::new();
    let mut users_name: Vec<String> = Vec::new();
    let mut log_data: Vec<String> = Vec::new();
    log_data = log_read::log_file_read(&log_file_path);
    if log_data.len() != 0 {
        for log_line in log_data {
            if log_line.contains("[Behaviour] OnPlayerJoined") {
                //プレイヤーがJoinした場合
                function::user_push(&mut users_name, &log_line[61..]);
                log_lines.push(log_print(
                    log_line.to_string(),
                    format!(
                        "JOIN: [{: >3}人] {}",
                        &users_name.len(),
                        function::rm_id((&log_line[61..]).to_string())
                    ),
                ));
            }
            if log_line.contains("[Behaviour] OnPlayerLeft ") {
                //プレイヤーがLeftした場合
                function::user_remove(&mut users_name, &log_line[59..]);
                log_lines.push(log_print(
                    log_line.to_string(),
                    format!(
                        "LEFT: [{: >3}人] {}",
                        &users_name.len(),
                        function::rm_id((&log_line[59..]).to_string())
                    ),
                ));
            }
            if log_line.contains("Attempting to resolve URL") {
                //動画などが再生された場合
                log_lines.push(log_print(
                    log_line.to_string(),
                    format!("URL : {}", &log_line[77..].to_string()),
                ));
            }
            if log_line.contains("[VRC Camera] Took screenshot to") {
                //写真が撮影された場合
                log_lines.push(log_print(
                    log_line.to_string(),
                    format!("CAM : {}", &log_line[67..].to_string()),
                ));
            }
            if log_line.contains("Joining or Creating Room") {
                //ワールドに移動した場合
                log_lines.push(log_print(
                    log_line.to_string(),
                    format!("ROOM: {}", &log_line[72..].to_string()),
                ))
            }
        }
    }
    http_send(log_lines).await?;
    Ok(())
}
