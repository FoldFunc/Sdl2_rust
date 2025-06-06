use reqwest::Client;
use reqwest::cookie::Jar;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct Register {
    email: String,
    password: String,
}

fn build_client_with_cookies(cookie_jar: Arc<Jar>) -> Result<Client, reqwest::Error> {
    Client::builder().cookie_provider(cookie_jar).build()
}

#[tokio::main]
pub async fn send_data_register(vals: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if vals.len() < 2 {
        return Err("Not enough values provided".into());
    }

    println!("Start sending data to the server");
    let cookie_jar = Arc::new(Jar::default());
    let client = build_client_with_cookies(cookie_jar.clone())?;

    let register_data = Register {
        email: vals[0].clone(),
        password: vals[1].clone(),
    };

    let res = client
        .post("http://localhost:8080/api/register")
        .json(&register_data)
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Response: {}", res.text().await?);
    Ok(())
}
