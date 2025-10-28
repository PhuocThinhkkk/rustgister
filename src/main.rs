#![allow(warnings)]

mod req_client;
use req_client::req_handler;
use dotenvy::dotenv;
mod app;
use app::App;
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
    let mut app = App::new(&args.session);
    app.set_client_metadata().await;
    app.set_all_class_allow_regist().await;
    app.set_all_class_registed().await;

    println!("{:#?}", app);

    println!("[DOG]");

    Ok(())
}

