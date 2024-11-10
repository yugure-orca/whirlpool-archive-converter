use clap::Subcommand;

pub mod event;

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
}
