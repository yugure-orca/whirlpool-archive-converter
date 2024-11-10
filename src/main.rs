use clap::Parser;
use commands::Commands;

mod commands;
mod event;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    match args.command {
        Commands::Event {
            whirlpool_state_file_path,
            whirlpool_token_file_path,
            whirlpool_transaction_file_path,
            whirlpool_event_file_path,
        } => commands::event::process(
            whirlpool_state_file_path,
            whirlpool_token_file_path,
            whirlpool_transaction_file_path,
            whirlpool_event_file_path,
        )
        .await
        .unwrap(),
    }
}
