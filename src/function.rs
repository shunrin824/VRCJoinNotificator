use chrono::{DateTime, Local};
use regex::Regex;
use std::{
    env::current_exe,
    fs,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

// ユーザーリストにユーザーを追加
pub fn user_push(users_name: &mut Vec<String>, user_name: &str) {
    let user_name: String = rm_id(user_name.to_string());
    users_name.push(user_name);
    users_name.shrink_to_fit();
}

// ユーザーリストからユーザーを削除
pub fn user_remove(users_name: &mut Vec<String>, user_name: &str) {
    let user_name: String = rm_id(user_name.to_string());
    users_name.retain(|s| s != &user_name);
    users_name.shrink_to_fit();
}

// ユーザー名からユーザーIDを除去
pub fn rm_id(user_name: String) -> String {
    return Regex::new(r"\(usr_.*\)$")
        .unwrap()
        .replace_all(&user_name, "")
        .to_string();
}

// デバッグモードが有効な場合のみメッセージを出力
pub fn debug_print(message: &str) {
    if let Some(debug_mode) = config_read("debug_mode") {
        if debug_mode.contains("true") {
            println!("{} DEBUG : {}", time_print(), message);
        }
    }
    return;
}

pub fn system_print(message: &str) {
    println!("{} SYSTEM: {}", time_print(), message);
    return;
}

// config.txtから指定された設定項目の値を取得
pub fn config_read(config_type: &str) -> Option<String> {
    let mut config_path: PathBuf = current_exe().unwrap();
    config_path.pop();
    config_path.push("config.txt");
    match fs::read_to_string(config_path) {
        Ok(config_data) => {
            let config_lines: Vec<String> = config_data.lines().map(|s| s.to_string()).collect();
            for config_line in config_lines {
                if config_line.contains(config_type) {
                    let remove_char_len: usize = config_type.len() + 1;
                    return Some(config_line[remove_char_len..].to_owned());
                }
            }
            return None;
        }
        Err(_) => {
            system_print("config.txtが見つかりませんでした。(ログの解析は続行されますが、config必須な処理を実行出来ません。)");
            return None;
        }
    }
}

//時間を表示する関数
pub fn time_print() -> String {
    let local_time = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    return local_time;
}
