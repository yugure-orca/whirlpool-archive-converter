use clap::Parser;
use commands::Commands;

mod commands;
mod model;

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
        Commands::Ohlcv {
            whirlpool_state_file_path,
            whirlpool_token_file_path,
            whirlpool_event_file_path,
            whirlpool_ohlcv_daily_file_path,
            whirlpool_ohlcv_minutely_file_path,
        } => commands::ohlcv::process(
            whirlpool_state_file_path,
            whirlpool_token_file_path,
            whirlpool_event_file_path,
            whirlpool_ohlcv_daily_file_path,
            whirlpool_ohlcv_minutely_file_path,
        )
        .await
        .unwrap(),
    }
}
