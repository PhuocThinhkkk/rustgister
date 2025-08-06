mod req_client;
use req_client::req_handler;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let res = req_handler::getAllStudyProgramRegist().await;

    println!("{}", res);
    println!("hi");
    Ok(())
}

