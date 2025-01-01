use regex::Regex;
use std::{env::current_exe, fs, path::PathBuf};

//現在のユーザーリストにユーザーを追加する関数
pub fn user_push(users_name: &mut Vec<String>, user_name: &str) {
    let user_name: String = rm_id(user_name.to_string());
    users_name.push(user_name);
    users_name.shrink_to_fit();
}

//現在のユーザーリストからユーザーを削除する関数
pub fn user_remove(users_name: &mut Vec<String>, user_name: &str) {
    let user_name: String = rm_id(user_name.to_string());
    users_name.retain(|s| s != &user_name);
    users_name.shrink_to_fit();
}

//ユーザー名からユーザーIDを取り除く関数
pub fn rm_id(user_name: String) -> String {
    return Regex::new(r"\(usr_.*\)$")
        .unwrap()
        .replace_all(&user_name, "")
        .to_string();
}

//configからidmsのip取ってくる関数(無かったらString: "none"を返答)
pub fn config_idms_url() -> String {
    let mut config_path: PathBuf = current_exe().unwrap();
    config_path.pop();
    config_path.push("config.txt");
    match fs::read_to_string(config_path) {
        Ok(config_data) => {
            let config_lines: Vec<String> = config_data.lines().map(|s| s.to_string()).collect();
            for config_line in config_lines {
                if (config_line.contains("idms_server_url")) {
                    return config_line[16..].to_owned();
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
