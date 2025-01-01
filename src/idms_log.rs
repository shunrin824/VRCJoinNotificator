use reqwest::multipart::Form;
use reqwest::header::{self, HeaderValue, AUTHORIZATION};

#[path = "./function.rs"]
mod function;

//idmsに簡易ログを送信する関数 
pub async fn http_send(log_lines: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let config_idms_url: String = function::config_idms_url();
    if !config_idms_url.contains("none") {
        println!("System: ログの送信を開始します。");
        let url = config_idms_url;
        let memo: String = log_lines.join("\n");
        let form = Form::new()
            .text("writetype", "new")
            .text("tag", "vrc log_data")
            .text("type", "txt")
            .text("memo", memo);

        let client = reqwest::Client::new();
        if (function::config_idms_auth_username() == "none") {
            let resp = client.post(url).multipart(form).send().await?;
        }else{
            let idms_auth = format!("Basic {}", base64::encode(&format!("{}:{}", function::config_idms_auth_username(),function::config_idms_auth_password())));
            let resp = client.post(url).header(AUTHORIZATION, idms_auth).multipart(form).send().await?;
        }
        println!("System: ログの送信が完了しました。");
    }
    Ok(())
}
