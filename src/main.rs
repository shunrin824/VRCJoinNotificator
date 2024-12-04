use core::time;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, BufReader};
use std::{env, fs::read_to_string, iter::Iterator, net::UdpSocket, path::PathBuf, thread};
use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

//XSOverlayに送るデータ用の構造体
#[derive(Serialize, Deserialize)]
struct xsoverlay_data {
    messageType: i32,
    index: i32,
    timeout: f32,
    height: f32,
    opacity: f32,
    volume: f32,
    audioPath: String,
    title: String,
    content: String,
    useBase64Icon: bool,
    icon: String,
    sourceApp: String,
}

//配列に複数のユーザー名がある場合にString型に治す関数
fn vec2xsoverlay(notification_type: i32, user_vec: Vec<String>) {
    let notification_data: String = user_vec.join("\n");
    match notification_type {
        1 => send2_xsoverlay("join", &notification_data),
        2 => send2_xsoverlay("left", &notification_data),
        _ => println!("不明なエラーが発生しました。"),
    }
}

//logを改行ごとに配列にする関数
fn log_in_vec(log_data: &str) -> Vec<String> {
    return log_data.lines().map(|s| s.to_string()).collect();
}

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

//XSOverlayに通知用のデータを送信する関数
fn send2_xsoverlay(title: &str, content: &str) {
    let number_of_rows: f32 = content.matches("\n").count() as f32;
    let data = xsoverlay_data {
        messageType: 1,
        index: 0,
        timeout: number_of_rows,
        height: 100.0 + number_of_rows * 10.0,
        opacity: 1.0,
        volume: 0.7,
        audioPath: String::from(""),
        title: String::from(title),
        content: String::from(content),
        useBase64Icon: false,
        icon: String::from(""),
        sourceApp: String::from(""),
    };
    let strdata: String = serde_json::to_string(&data).unwrap();
    let socket = UdpSocket::bind("127.0.0.1:42068").unwrap();
    socket.connect("127.0.0.1:42069").unwrap();
    socket.send(strdata.as_bytes()).unwrap();
}

//改行ごとに配列にされたlogファイルから必要な情報を取ってくる関数
fn log_analyze(
    log_data: &mut Vec<String>,
    number_of_lines: &usize,
    users_name: &mut Vec<String>,
) -> (usize, Vec<String>) {
    let log_length: usize = log_data.len().try_into().unwrap(); //この呼び出しで解析する行数を代入
    let mut join_data: Vec<String> = Vec::new(); //join通知用のユーザー名の配列
    let mut left_data: Vec<String> = Vec::new(); //left通知用のユーザー名の配列
    log_data.drain(..number_of_lines); //前のループで解析済みのデータを破棄
    if log_data.len() != 0 {
        for log_line in log_data {
            if log_line.contains("[Behaviour] OnPlayerJoined") {
                //プレイヤーがJoinした場合
                user_push(users_name, &log_line[61..]);
                join_data.push(rm_id((&log_line[61..]).to_string()));
                println!(
                    "{}/{}/{} {} JOIN: [{: >3}人] {}",
                    &log_line[0..4],
                    &log_line[5..7],
                    &log_line[8..10],
                    &log_line[11..19],
                    &users_name.len(),
                    rm_id((&log_line[61..]).to_string())
                );
            } else if log_line.contains("[Behaviour] OnPlayerLeft ") {
                //プレイヤーがLeftした場合
                user_remove(users_name, &log_line[59..]);
                left_data.push(rm_id((&log_line[59..]).to_string()));
                println!(
                    "{}/{}/{} {} LEFT: [{: >3}人] {}",
                    &log_line[0..4],
                    &log_line[5..7],
                    &log_line[8..10],
                    &log_line[11..19],
                    &users_name.len(),
                    rm_id((&log_line[59..]).to_string())
                );
            } else if log_line.contains("Attempting to resolve URL") {
                //動画などが再生された場合
                println!(
                    "{}/{}/{} {} URL : {}",
                    &log_line[0..4],
                    &log_line[5..7],
                    &log_line[8..10],
                    &log_line[11..19],
                    &log_line[77..]
                );
                //send2_xsoverlay("URL", &log_line[77..]);
            } else if log_line.contains("[VRC Camera] Took screenshot to") {
                //写真が撮影された場合
                println!(
                    "{}/{}/{} {} CAM : {}",
                    &log_line[0..4],
                    &log_line[5..7],
                    &log_line[8..10],
                    &log_line[11..19],
                    &log_line[67..]
                );
            }
        }
    }
    match join_data.len().try_into() {
        Ok(0) => (),
        Ok(1) => send2_xsoverlay("join", &join_data[0]),
        Ok(_) => vec2xsoverlay(1, join_data),
        Err(_) => println!("不明なエラーが発生しました。変数join_dataに異常が発生しています。"),
    }
    match left_data.len().try_into() {
        Ok(0) => (),
        Ok(1) => send2_xsoverlay("left", &left_data[0]),
        Ok(_) => vec2xsoverlay(2, left_data),
        Err(_) => println!("不明なエラーが発生しました。変数left_dataに異常が発生しています。"),
    }

    return (log_length, users_name.to_vec());
}

//logファイルをメモリに読み込む関数
fn log_file_read(log_file_path: &PathBuf) -> Vec<String> {
    match read_to_string(log_file_path) {
        Ok(log_data) => {
            let log_lines: Vec<String> = log_in_vec(&log_data);
            return log_lines;
        }
        Err(e) => {
            println!("Error:{}", e);
            panic!("ログファイルを読み込めません。");
        }
    }
}

//ログファイルのパスを確定するための関数
fn log_file_path() -> PathBuf {
    let log_file_path = PathBuf::from(env::var("USERPROFILE").expect("error"))
        .join("AppData")
        .join("LocalLow")
        .join("VRChat")
        .join("VRChat")
        .join("output_log_2024-11-29_23-57-45.txt"); //これを変更予定です。
    return log_file_path;
}

fn main() {
    let mut number_of_lines: usize = 0;
    let mut users_name: Vec<String> = Vec::new();
    loop {
        let log_file_path = log_file_path(); //最新のログファイルのパスをlog_file_pathに代入。
        let mut log_lines = log_file_read(&log_file_path);
        (number_of_lines, users_name) =
            log_analyze(&mut log_lines, &mut number_of_lines, &mut users_name);
        //ログを1行ずつ見て、通知したり標準出力に出力したり、サーバーに送ったりする予定です。
        //このあたりに、終了条件を書く予定。多分、ログの中の"OnApplicationQuit"がトリガーと思われますが、VRCがエラー落ちした場合も考えないといけないかも...
        thread::sleep(time::Duration::from_millis(500));
    }
}
