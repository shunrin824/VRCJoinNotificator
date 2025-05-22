use core::time;
use std::{path::PathBuf, thread};
use sysinfo::System;

mod function;
mod idms;
mod log_read;
mod xsoverlay;
mod webhook;

//build時のコマンド
//time cross build -r --target "x86_64-pc-windows-gnu"; date

//コンソールにログ吐き出す関数
fn log_print(log_line: String, log_content: String) -> String {
    return format!(
        "{}/{}/{} {} {}",
        &log_line[0..4],
        &log_line[5..7],
        &log_line[8..10],
        &log_line[11..19],
        &log_content
    )
    .to_string(); //最初の4つは日時の表示。
}

//改行ごとに配列にされたlogファイルから必要な情報を取ってくる関数
async fn log_analyze(
    log_lines: &mut Vec<String>,
    number_of_lines: &usize,
    users_name: &mut Vec<String>,
    mut world_name: String,
    log_formated_lines: &mut Vec<String>,
) -> (usize, Vec<String>, String, Vec<idms::UploadData>) {
    let log_length: usize = log_lines.len().try_into().unwrap(); //この呼び出しで解析する行数を代入
    let mut join_data: Vec<String> = Vec::new(); //join通知用のユーザー名の配列
    let mut left_data: Vec<String> = Vec::new(); //left通知用のユーザー名の配列
    let mut upload_datas: Vec<idms::UploadData> = Vec::new();
    log_lines.drain(..number_of_lines); //前のループで解析済みのデータを破棄
    if log_lines.len() != 0 {
        for log_line in log_lines {
            if log_line.contains("[Behaviour] OnPlayerJoined") {
                //プレイヤーがJoinした場合
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
                //プレイヤーがLeftした場合
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
                //動画などが再生された場合
                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!("URL : {}", &log_line[77..].to_string()),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
                xsoverlay::send2_xsoverlay("URL", &log_line[77..]);
            }
            if log_line.contains("Resolving URL") {
                //ストリーミングの映像が再生された場合
                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!("URL : {}", &log_line[65..].to_string()),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
                xsoverlay::send2_xsoverlay("URL", &log_line[66..]);
            }
            if log_line.contains("[VRC Camera] Took screenshot to") {
                //写真が撮影された場合
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
                //ワールドに移動した場合
                world_name = log_line[72..].to_string();
                let log_formated: String = log_print(
                    log_line.to_string(),
                    format!("ROOM: {}", &log_line[72..].to_string()),
                );
                println!("{}", log_formated);
                log_formated_lines.push(log_formated);
            }
            if log_line.contains("Received Notification"){
                //inviteやReqinをDiscordに送信する処理
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
    let mut log_file_path = log_read::log_file_path(); //最新のログファイルのパスをlog_file_pathに代入。
    let mut number_of_lines: usize = 0; //既に処理した行数を保存する変数
    let mut users_name: Vec<String> = Vec::new(); //現時点でのインスタンス内のユーザーを保存する変数
    let mut world_name: String = "".to_string();
    let mut upload_datas: Vec<idms::UploadData> = Vec::new();
    let mut log_lines: Vec<String> = Vec::new();
    let mut log_formated_lines: Vec<String> = Vec::new();
    println!(
        "System: VRCJoinNotificatorの初期化が完了しました。\nSystem: ログの解析を開始します。"
    );
    loop {
        log_lines = log_read::log_file_read(&log_file_path); //最新のログファイルを行ごとの配列でlog_linesに代入
        (number_of_lines, users_name, world_name, upload_datas) = log_analyze(
            &mut log_lines,
            &mut number_of_lines,
            &mut users_name,
            world_name,
            &mut log_formated_lines,
        ).await; //ログを解析して色々する関数

        match idms::pictures_upload(upload_datas).await {
            //画像をsdmsにアップロードする処理
            Ok(_) => (),
            Err(e) => println!("idmsへの送信でエラーが発生しました。{}", e),
        };
        upload_datas = Vec::new(); //初期化処理
        thread::sleep(time::Duration::from_millis(100)); //負荷軽減のために100ms待機

        //VRCが終了した時に新しいログファイルを読み込みに行くためのif
        //VRCが終了すると、ログを整形してidmsに送信し、VRCが起動してログが書き込まれるのを待つ。
        if System::new_all()
            .processes_by_name("VRChat".as_ref())
            .count()
            < 1
        {
            println!("System: VRChatが終了しました。");
            idms::idms_log_send(log_formated_lines).await?;
            println!("System: VRChatの起動を待っています。\nSystem: VRCJoinNotificatorを終了する場合はXボタン、またはCtrl+Cで終了して下さい。");
            while log_file_path == log_read::log_file_path() {
                thread::sleep(time::Duration::from_secs(15)); //負荷を掛けないように15秒のsleep
            }
            println!(
                "System: VRChatの起動を確認しました。\nSystem: VRCJoinNotificatorを初期化します。"
            );
            log_file_path = log_read::log_file_path(); //最新のログファイルのパスをlog_file_pathに代入。
                                                       //下3行は初期化
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
