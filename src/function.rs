use regex::Regex;
use std::{env::current_exe, fs, path::PathBuf};

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
    if config_read("debug_mode").contains("true") {
        println!("Debug: {}", message);
    }
    return;
}
// config.txtから指定された設定項目の値を取得
pub fn config_read(config_type: &str) -> String {
    let mut config_path: PathBuf = current_exe().unwrap();
    config_path.pop();
    config_path.push("config.txt");
    match fs::read_to_string(config_path) {
        Ok(config_data) => {
            let config_lines: Vec<String> = config_data.lines().map(|s| s.to_string()).collect();
            for config_line in config_lines {
                if (config_type == "idms_server_url") {
                    if (config_line.contains("idms_server_url")) {
                        return config_line[16..].to_owned();
                    }
                } else if (config_type == "idms_server_auth_username") {
                    if (config_line.contains("idms_server_auth_username")) {
                        return config_line[26..].to_owned();
                    }
                } else if (config_type == "idms_server_auth_password") {
                    if (config_line.contains("idms_server_auth_password")) {
                        return config_line[26..].to_owned();
                    }
                } else if (config_type == "discord_webhook_url") {
                    if (config_line.contains("discord_webhook_url")) {
                        return config_line[20..].to_owned();
                    }
                } else if (config_type == "discord_webhook_image_resolution") {
                    if (config_line.contains("discord_webhook_image_resolution")) {
                        return config_line[33..].to_owned();
                    }
                } else if (config_type == "debug_mode") {
                    if (config_line.contains("debug_mode")) {
                        return config_line[11..].to_owned();
                    }
                } else if (config_type == "max_convertpic_threads") {
                    if (config_line.contains("max_convertpic_threads")) {
                        return config_line[23..].to_owned();
                    }
                }
            }
            return "none".to_owned();
        }
        Err(_) => {
            println!("Error : config.txtが見つかりませんでした。(ログの解析は続行されますが、config必須な処理を実行出来ません。)");
            return "none".to_owned();
        }
    }
}
