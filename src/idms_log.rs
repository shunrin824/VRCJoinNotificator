use reqwest::multipart::Form;

#[path = "./function.rs"]
mod function;

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
        let resp = client.post(url).multipart(form).send().await?;
        if !resp.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to send the request",
            )));
        }
        println!("System: ログの送信が完了しました。");
    }
    Ok(())
}
