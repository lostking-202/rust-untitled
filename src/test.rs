use http::Request;
use reqwest::Error;
use serde_json;
use reqwest::Client;
use reqwest;

#[tokio::test]
async fn test_post_request() -> Result<(), Error> {
    let client=Client::new();
    let response = client.post("http://localhost:57009/v1/chat/completions")
        .body(r#"{
        "model": "Llama-3-8B-Instruct",
        "messages": [
            {"role": "user", "content": "today is really a tough day"}
        ],
        "stream": True
    }"#)
        .send()
        .await?;
    println!("{}",response.status());
    Ok(())
}

#[test]
fn post_request(){
    // 发送 GET 请求
    let response = reqwest::blocking::

    // 打印响应状态码
    println!("Status: {}", response.status());

    // 打印响应体
    println!("Response body: {}", response.text()?);

    Ok(())
}