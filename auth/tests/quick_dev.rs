// #[tokio::test]
// async fn quick_dev()->Result<()>{
//     let hc = httpc_test::new_client("http://127.0.0.1:9090")?;
//     hc.do_get("/hello").await?.println().await?;
//     hc.do_get("/hello2?name=cci").await?.println().await?;
//     hc.do_get("/src/main.rs").await?.println().await?;

//     Ok(())
// }