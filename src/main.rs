use core::time;
use std::{collections::VecDeque, path::PathBuf, thread};
use sysinfo::System;

mod function;
mod idms;
mod log_read;
mod webhook;
mod xsoverlay;

// ビルドコマンド例
// time cross build -r --target "x86_64-pc-windows-gnu"; date

// VRChatログの日時フォーマットを整形する関数
fn log_print(log_line: String, log_content: String) -> String {
    return format!(
        "{}/{}/{} {} {}",
        &log_line[0..4],
        &log_line[5..7],
        &log_line[8..10],
        &log_line[11..19],
        &log_content
    )
    .to_string();
}

// VRChatログファイルを解析し、プレイヤーのJoin/Leave、URL、写真撮影、ワールド移動、通知を処理する関数
async fn log_analyze(
    log_lines: &mut Vec<String>,
    number_of_lines: &usize,
    users_name: &mut Vec<String>,
    mut world_name: String,
    log_formated_lines: &mut Vec<String>,
) -> (usize, Vec<String>, String, Vec<idms::UploadData>) {
    let log_length: usize = log_lines.len().try_into().unwrap();
    let mut join_data: Vec<String> = Vec::new();
    let mut left_data: Vec<String> = Vec::new();
    let mut upload_datas: Vec<idms::UploadData> = Vec::new();
    log_lines.drain(..number_of_lines);
    if log_lines.len() != 0 {
        for log_line in log_lines {
            if log_line.contains("[Behaviour] OnPlayerJoined") {
                function::user_push(users_name, &log_line[61..]);
                join_data.push(function::rm_id((&log_line[61..]).to_string()));
                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!(
                        "JOIN: [{: >3}人] {}",
                        &users_name.len(),
                        function::rm_id((&log_line[61..]).to_string())
                    ),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
            }
            if log_line.contains("[Behaviour] OnPlayerLeft ") {
                function::user_remove(users_name, &log_line[59..]);
                left_data.push(function::rm_id((&log_line[59..]).to_string()));

                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!(
                        "LEFT: [{: >3}人] {}",
                        &users_name.len(),
                        function::rm_id((&log_line[59..]).to_string())
                    ),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
            }
            if log_line.contains("Attempting to resolve URL") {
                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!("URL : {}", &log_line[77..].to_string()),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
                xsoverlay::send2_xsoverlay("URL", &log_line[77..]);
            }
            if log_line.contains("Resolving URL") {
                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!("URL : {}", &log_line[65..].to_string()),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
                xsoverlay::send2_xsoverlay("URL", &log_line[66..]);
            }
            if log_line.contains("[VRC Camera] Took screenshot to") {
                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!("CAM : {}", &log_line[67..].to_string()),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
                let upload_data = idms::UploadData {
                    users_name: users_name.to_vec(),
                    file_path: PathBuf::from(&log_line[67..]),
                    world_name: world_name.clone(),
                };
                upload_datas.push(upload_data);
            }
            if log_line.contains("Joining or Creating Room") {
                world_name = log_line[72..].to_string();
                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!("ROOM: {}", &log_line[72..].to_string()),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
            }
            if log_line.contains("Received Notification") {
                webhook::invite_format(&log_line).await;
            }
        }
    }
    match join_data.len().try_into() {
        Ok(0) => (),
        Ok(1) => {
            xsoverlay::send2_xsoverlay(&format!("JOIN [{: >3}人]", users_name.len()), &join_data[0])
        }
        Ok(_) => xsoverlay::vec2xsoverlay(1, join_data, users_name.len()),
        Err(_) => {
            println!("Error : 不明なエラーが発生しました。変数join_dataに異常が発生しています。")
        }
    }
    match left_data.len().try_into() {
        Ok(0) => (),
        Ok(1) => {
            xsoverlay::send2_xsoverlay(&format!("LEFT [{: >3}人]", users_name.len()), &left_data[0])
        }
        Ok(_) => xsoverlay::vec2xsoverlay(2, left_data, users_name.len()),
        Err(_) => {
            println!("Error : 不明なエラーが発生しました。変数left_dataに異常が発生しています。")
        }
    }

    return (log_length, users_name.to_vec(), world_name, upload_datas);
}
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "System: VRCJoinNotificatorが起動しました。\nSystem: VRCJoinNotificatorを初期化中です。"
    );
    let mut log_file_path = log_read::log_file_path();
    let mut number_of_lines: usize = 0;
    let mut users_name: Vec<String> = Vec::new();
    let mut world_name: String = "".to_string();
    let mut upload_datas: Vec<idms::UploadData> = Vec::new();
    let mut log_lines: Vec<String> = Vec::new();
    let mut log_formated_lines: Vec<String> = Vec::new();
    let mut upload_queue: VecDeque<Vec<idms::UploadData>> = VecDeque::new();
    let mut upload_handles: Vec<thread::JoinHandle<()>> = Vec::new();

    let max_pic_convert_threads: usize;
    if let Ok(config_max_str) = function::config_read("max_convertpic_threads").parse::<usize>() {
        max_pic_convert_threads = config_max_str;
    } else {
        max_pic_convert_threads = 1;
    }

    function::debug_print("System: debug_modeが有効になっています。");
    println!(
        "System: VRCJoinNotificatorの初期化が完了しました。\nSystem: ログの解析を開始します。"
    );
    loop {
        log_lines = log_read::log_file_read(&log_file_path);
        (number_of_lines, users_name, world_name, upload_datas) = log_analyze(
            &mut log_lines,
            &mut number_of_lines,
            &mut users_name,
            world_name,
            &mut log_formated_lines,
        )
        .await;
        //マルチスレッド(最大スレッド数: max_pic_convert_threads)でのDiscordとSDMSへのアップロード処理
        upload_handles.retain(|handle| !handle.is_finished());

        if !upload_datas.is_empty() {
            upload_queue.push_back(upload_datas.clone());
            upload_datas = Vec::new();
        }

        if upload_handles.len() < max_pic_convert_threads {
            if let Some(data) = upload_queue.pop_front() {
                let handle = thread::spawn(move || {
                    let _ = idms::pictures_upload(data);
                });
                upload_handles.push(handle);
            }
        }

        thread::sleep(time::Duration::from_millis(100));

        // VRChatプロセス終了時の処理
        if System::new_all()
            .processes_by_name("VRChat".as_ref())
            .count()
            < 1
        {
            println!("System: VRChatが終了しました。");
            idms::idms_log_send(log_formated_lines).await?;

            if upload_handles.len() > 0 {
                println!("System: 現在アップロード処理中です。");
                while upload_handles.len() > 0 {
                    thread::sleep(time::Duration::from_millis(200));
                }
                println!("System: アップロードが完了しました。");
            }
            println!("System: VRChatの起動を待っています。\nSystem: VRCJoinNotificatorを終了する場合はXボタン、またはCtrl+Cで終了して下さい。");
            while log_file_path == log_read::log_file_path() {
                thread::sleep(time::Duration::from_secs(15)); //負荷を掛けないように15秒のsleep
            }
            println!(
                "System: VRChatの起動を確認しました。\nSystem: VRCJoinNotificatorを初期化します。"
            );
            log_file_path = log_read::log_file_path();
            number_of_lines = 0;
            users_name = Vec::new();
            world_name = "".to_string();
            log_formated_lines = Vec::new();
            println!(
                        "System: VRCJoinNotificatorの初期化が完了しました。\nSystem: ログの解析を再開します。"
                    );
        }
    }
}
