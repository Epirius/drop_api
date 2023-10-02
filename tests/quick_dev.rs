#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://127.0.0.1:4000")?;
    client.do_get("/hello2/Felix").await?.print().await?;
    //client.do_get("/hello?name=Felix").await?.print().await?;

    let req_login = client.do_post("/api/login",json!({
        "username": "admin",
        "password": "password"
    })).await?;
    req_login.print().await?;

    Ok(())
}