use serde::{Deserialize, Serialize};
use std::net::UdpSocket;

// XSOverlayの通知データ構造体

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct XsoverlayData {
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

// XSOverlayにUDPで通知を送信
pub fn send2_xsoverlay(title: &str, content: &str) {
    let number_of_rows: f32 = content.matches("\n").count() as f32;
    let data = XsoverlayData {
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

// 複数ユーザーの名前を結合してXSOverlayに送信
pub fn vec2xsoverlay(notification_type: i32, user_vec: Vec<String>, number_of_users: usize) {
    let notification_data: String = user_vec.join("\n");
    match notification_type {
        1 => send2_xsoverlay(&format!("JOIN [{: >3}人]",number_of_users), &notification_data),
        2 => send2_xsoverlay(&format!("LEFT [{: >3}人]",number_of_users), &notification_data),
        _ => println!("不明なエラーが発生しました。"),
    }
}
