use reqwest::multipart::Form;

pub async fn http_send(log_lines: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://192.168.1.132/idms/datawrite.php";
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
    Ok(())
}