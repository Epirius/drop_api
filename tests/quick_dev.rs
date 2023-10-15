#![allow(unused)]

use anyhow::Result;
use serde_json::json;

// #[tokio::test]
// async fn quick_dev() -> Result<()> {
//     let client = httpc_test::new_client("http://127.0.0.1:4000")?;
//     // client.do_get("/hello2/Felix").await?.print().await?;
//     //client.do_get("/hello?name=Felix").await?.print().await?;
//
//     let req_login = client
//         .do_post(
//             "/api/login",
//             json!({
//                 "username": "admin",
//                 "password": "password"
//             }),
//         )
//         .await?;
//     req_login.print().await?;
//
//     // client.do_get("/api/podcast/meta/45038721-5339-56b7-86bc-9bd0d44409f3").await?.print().await?;
//     client
//         .do_get("/api/podcast/episode/2897322f-acd0-553b-829b-961e9f52b5cc")
//         .await?
//         .print()
//         .await?;
//     // client.do_get("/hello?name=felix").await?.print().await?;
//     // let req_get_podcast = client.do_get("/api/podcast/Test_Uuid").await?.print().await?;
//
//     // let req_create_ticket = client.do_post("/api/tickets",
//     //     json!(
//     //         {
//     //             "title": "Test tickets",
//     //         }
//     //     ),
//     // );
//     // req_create_ticket.await?.print().await?;
//     //
//     // client.do_get("/api/tickets").await?.print().await?;
//
//     Ok(())
// }
