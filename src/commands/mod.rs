use clap::Subcommand;

pub mod event;
pub mod ohlcv;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Event {
        #[arg(long, short = 's', id = "whirlpool-state-file-path")]
        whirlpool_state_file_path: String,
        #[arg(long, short = 't', id = "whirlpool-token-file-path")]
        whirlpool_token_file_path: String,
        #[arg(long, short = 'x', id = "whirlpool-transaction-file-path")]
        whirlpool_transaction_file_path: String,
        #[arg(long, short = 'e', id = "whirlpool-event-file-path")]
        whirlpool_event_file_path: String,
    },
    Ohlcv {
        #[arg(long, short = 's', id = "whirlpool-state-file-path")]
        whirlpool_state_file_path: String,
        #[arg(long, short = 't', id = "whirlpool-token-file-path")]
        whirlpool_token_file_path: String,
        #[arg(long, short = 'e', id = "whirlpool-event-file-path")]
        whirlpool_event_file_path: String,
        #[arg(long, short = 'd', id = "whirlpool-ohlcv-daily-file-path")]
        whirlpool_ohlcv_daily_file_path: String,
        #[arg(long, short = 'm', id = "whirlpool-ohlcv-minutely-file-path")]
        whirlpool_ohlcv_minutely_file_path: String,
    },
}
