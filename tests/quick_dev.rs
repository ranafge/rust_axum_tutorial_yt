use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() ->Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Rana").await?.print().await?;
    hc.do_get("/hello2/rana").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;
    hc.do_post("/api/login", json!({
        "username": "demo1",
        "pwd": "welcome"
    })).await?.print().await?;

    Ok(())
}