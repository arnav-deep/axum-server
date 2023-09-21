use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    hc.do_get("/hello?name=Banana&age=12")
        .await?
        .print()
        .await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    );
    req_login.await?.print().await?;

    hc.do_get("/hello/Unana").await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket 1"
        }),
    );

    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    let req_create_ticket = hc.do_post("/api/tickets", json!({"title": "Ticket 2"}));

    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    hc.do_delete("/api/tickets/0").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
