#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::client("http://localhost:4000")?;
    client.do_get("/hello").await?.print().await?;
    Ok(())
}