use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() ->Result<()> {
    // hc is HTTP CLIENT instance to interact with localhost server
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/src/main.rs").await?.print().await?;
    // hc is use to perform different type http request like post, put, get delete etc.
    hc.do_get("/hello?name=Rana").await?.print().await?;
    hc.do_get("/hello2/rana").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;
    hc.do_post("/api/login", json!({
        "username": "demo1",
        "pwd": "welcome"
    })).await?.print().await?;

    Ok(())
}

/*
    The chain .await?.print().await?; is a sequence of asynchronous operations:

    The first .await? waits for the completion of an asynchronous operation (likely sending an HTTP request) and propagates any errors that may occur during the operation.
    Then, the .print() method is called to print the response details to the console.
    Finally, the second .await? waits for the completion of the printing operation and propagates any potential errors.

*/