use core::time;
use regex::Regex;
use std::process::Command;
use std::{path::PathBuf, thread};

mod http_upload;
mod log_read;
mod xsoverlay;

//現在のユーザーリストにユーザーを追加する関数
fn user_push(users_name: &mut Vec<String>, user_name: &str) {
    let user_name: String = rm_id(user_name.to_string());
    users_name.push(user_name);
    users_name.shrink_to_fit();
}

//現在のユーザーリストからユーザーを削除する関数
fn user_remove(users_name: &mut Vec<String>, user_name: &str) {
    let user_name: String = rm_id(user_name.to_string());
    users_name.retain(|s| s != &user_name);
    users_name.shrink_to_fit();
}

//ユーザー名からユーザーIDを取り除く関数
fn rm_id(user_name: String) -> String {
    return Regex::new(r"\(usr_.*\)$")
        .unwrap()
        .replace_all(&user_name, "")
        .to_string();
}

//コンソールにログ吐き出す関数
fn log_print(log_line: String, log_content: String) {
    println!(
        "{}",
        format!(
            "{}/{}/{} {} {}",
            &log_line[0..4],
            &log_line[5..7],
            &log_line[8..10],
            &log_line[11..19],
            &log_content
        )
    ); //最初の4つは日時の表示。
}

//改行ごとに配列にされたlogファイルから必要な情報を取ってくる関数
fn log_analyze(
    log_data: &mut Vec<String>,
    number_of_lines: &usize,
    users_name: &mut Vec<String>,
    mut world_name: String,
) -> (usize, Vec<String>, String, Vec<http_upload::UploadData>) {
    let log_length: usize = log_data.len().try_into().unwrap(); //この呼び出しで解析する行数を代入
    let mut join_data: Vec<String> = Vec::new(); //join通知用のユーザー名の配列
    let mut left_data: Vec<String> = Vec::new(); //left通知用のユーザー名の配列
    let mut upload_datas: Vec<http_upload::UploadData> = Vec::new();
    log_data.drain(..number_of_lines); //前のループで解析済みのデータを破棄
    if log_data.len() != 0 {
        for log_line in log_data {
            if log_line.contains("[Behaviour] OnPlayerJoined") {
                //プレイヤーがJoinした場合
                user_push(users_name, &log_line[61..]);
                join_data.push(rm_id((&log_line[61..]).to_string()));
                log_print(
                    log_line.to_string(),
                    format!(
                        "JOIN: [{: >3}人] {}",
                        &users_name.len(),
                        rm_id((&log_line[61..]).to_string())
                    ),
                );
            }
            if log_line.contains("[Behaviour] OnPlayerLeft ") {
                //プレイヤーがLeftした場合
                user_remove(users_name, &log_line[59..]);
                left_data.push(rm_id((&log_line[59..]).to_string()));

                log_print(
                    log_line.to_string(),
                    format!(
                        "LEFT: [{: >3}人] {}",
                        &users_name.len(),
                        rm_id((&log_line[59..]).to_string())
                    ),
                );
            }
            if log_line.contains("Attempting to resolve URL") {
                //動画などが再生された場合
                log_print(
                    log_line.to_string(),
                    format!("URL : {}", &log_line[77..].to_string()),
                );
                xsoverlay::send2_xsoverlay("URL", &log_line[77..]);
            }
            if log_line.contains("[VRC Camera] Took screenshot to") {
                //写真が撮影された場合
                log_print(
                    log_line.to_string(),
                    format!("CAM : {}", &log_line[67..].to_string()),
                );
                let upload_data = http_upload::UploadData {
                    users_name: users_name.to_vec(),
                    file_path: PathBuf::from(&log_line[67..]),
                    world_name: world_name.clone(),
                };
                upload_datas.push(upload_data);
            }
            if log_line.contains("Joining or Creating Room") {
                //ワールドに移動した場合
                world_name = log_line[72..].to_string();
                log_print(
                    log_line.to_string(),
                    format!("ROOM: {}", &log_line[72..].to_string()),
                )
            }
        }
    }
    match join_data.len().try_into() {
        Ok(0) => (),
        Ok(1) => {
            xsoverlay::send2_xsoverlay(&format!("JOIN [{: >3}人]", users_name.len()), &join_data[0])
        }
        Ok(_) => xsoverlay::vec2xsoverlay(1, join_data, users_name.len()),
        Err(_) => println!("不明なエラーが発生しました。変数join_dataに異常が発生しています。"),
    }
    match left_data.len().try_into() {
        Ok(0) => (),
        Ok(1) => {
            xsoverlay::send2_xsoverlay(&format!("LEFT [{: >3}人]", users_name.len()), &left_data[0])
        }
        Ok(_) => xsoverlay::vec2xsoverlay(2, left_data, users_name.len()),
        Err(_) => println!("不明なエラーが発生しました。変数left_dataに異常が発生しています。"),
    }

    return (log_length, users_name.to_vec(), world_name, upload_datas);
}
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut log_file_path = log_read::log_file_path(); //最新のログファイルのパスをlog_file_pathに代入。
    let mut number_of_lines: usize = 0; //既に処理した行数を保存する変数
    let mut users_name: Vec<String> = Vec::new(); //現時点でのインスタンス内のユーザーを保存する変数
    let mut world_name: String = "".to_string();
    let mut upload_datas: Vec<http_upload::UploadData> = Vec::new();
    loop {
        let mut log_lines = log_read::log_file_read(&log_file_path); //最新のログファイルを行ごとの配列でlog_linesに代入
        (number_of_lines, users_name, world_name, upload_datas) = log_analyze(
            &mut log_lines,
            &mut number_of_lines,
            &mut users_name,
            world_name,
        ); //ログを解析して色々する関数
        http_upload::pictures_upload(upload_datas).await?;
        upload_datas = Vec::new();

        thread::sleep(time::Duration::from_millis(500));//負荷を掛けないように500msのsleep

        //VRCが終了した時のためのやつ。
        match Command::new("Get-Process -name VRChat").output() {
            Ok(_) => (),
            Err(_) => {
                log_file_path = log_read::log_file_path(); //最新のログファイルのパスをlog_file_pathに代入。
                //下3行は初期化
                number_of_lines = 0;
                users_name = Vec::new();
                world_name = "".to_string();
            }
        }
    }

    return Ok(());
}
