use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
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
    

    login_req.await?.print().await?;

    let req_create_ticket = hc.do_post(
		"/api/tickets",
		json!({
			"title": "Ticket AAA"
		}),
	);
    
    req_create_ticket.await?.print().await?;

	hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}