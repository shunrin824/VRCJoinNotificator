use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

// VRChatの最新ログファイルパスを取得
pub fn log_file_path() -> PathBuf {
    let mut latest_log_file: String = "".to_owned();
    let mut latest_log_time: i64 = 0;
    let log_dir = PathBuf::from(env::var("USERPROFILE").expect("error"))
        .join("AppData")
        .join("LocalLow")
        .join("VRChat")
        .join("VRChat");
    let files = log_dir.read_dir().expect("This Directory is nothing.");
    for file_path in files {
        let log_file_name: String = file_path
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if log_file_name.contains("output_log_") {
            let log_time: i64 = format!(
                "{}{}{}{}{}{}",
                &log_file_name[11..15],
                &log_file_name[16..18],
                &log_file_name[19..21],
                &log_file_name[22..24],
                &log_file_name[25..27],
                &log_file_name[28..30]
            )
            .parse::<i64>()
            .unwrap();
            if latest_log_time < log_time {
                latest_log_time = log_time;
                latest_log_file = log_file_name;
            }
        }
    }
    let log_file_path = log_dir.join(latest_log_file);
    return log_file_path;
}

// ログファイルを文字列として読み込み、行単位で分割して返す
pub fn log_file_read(log_file_path: &PathBuf) -> Vec<String> {
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

// ログ文字列を改行で分割してVec<String>に変換
pub fn log_in_vec(log_data: &str) -> Vec<String> {
    return log_data.lines().map(|s| s.to_string()).collect();
}
