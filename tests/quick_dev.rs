#![allow(unused)]

use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    // Assuming httpc_test has a function `new_client` that returns a Result
    let hc = httpc_test::new_client("http://localhost:8080")?;
    
    // Assuming httpc_test client has an async method `do_get` that returns a Result
    // hc.do_get("/hello?name=Abhi").await?.print().await?;
    hc.do_get("/hello2/Abhi").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login", 
        json!({
            "username":"demo1",
            "pwd":"welcome"
        })
        
    );
req_login.await?.print().await?;

let req_create_ticket = hc.do_post("/api/tickets", 
json!({
    "title":"TICKET AAA"
}),
);

req_create_ticket.await?.print().await?;

hc.do_get("/api/tickets").await?.print().await?;

hc.do_delete("/api/tickets/3").await?.print().await?;

hc.do_get("/api/tickets").await?.print().await?;
    Ok(())
}