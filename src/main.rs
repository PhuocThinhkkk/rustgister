#![allow(warnings)]

mod req_client;
use req_client::req_handler;
use dotenvy::dotenv;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    session: String,
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    if args.session.is_empty() {
        println!("Please provide a session using --session <SESSION_VALUE>");
        return Ok(());
    }
    dotenv().ok();
    let res = req_handler::getAllStudyProgramRegist(&args.session).await;
    let res2 = req_handler::getRegistSemesterCreditQuota(&args.session).await;
    let res3 = req_handler::getAllClassRegisted(&args.session).await;
    let res4 = req_handler::getAllClassAllowRegist(&args.session).await;

    println!("[DOG]");

    println!("{}", res);
    println!("{}", res2);
    println!("{}", res3);
    println!("{}", res4);
    Ok(())
}

