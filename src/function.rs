use regex::Regex;

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