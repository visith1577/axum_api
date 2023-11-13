use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(
        "http://localhost:3000"
    )?;
    hc.do_get("/root2/visith").await?.print().await?;
    hc.do_get("/").await?.print().await?;

    let login_req = hc.do_post(
        "/api/login", 
        json!({
            "username" : "visith",
            "pwd" : "root"
        })
    );
    
    // let login_req_fail = hc.do_post(
    //     "/api/login", 
    //     json!({
    //         "username" : "visith",
    //         "pwd" : "wrong" // wrong pwd
    //     })
    // );

    login_req.await?.print().await?;

    hc.do_get("/root2/visith").await?.print().await?;

    let req_create_ticket = hc.do_post("/api/tickets", 
        json!({
            "title": "Ticket AAA"
        }),
    );

    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;
    // hc.do_delete("/api/tickets/0").await?.print().await?;

    Ok(())
}