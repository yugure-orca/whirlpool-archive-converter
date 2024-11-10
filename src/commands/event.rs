use crate::event::{
    convert::build_whirlpool_events, definition::ProgramDeployedEventPayload, WhirlpoolEvent,
    WhirlpoolEventBlock, WhirlpoolEventTransaction,
};
use anyhow::Result;
use flate2::write::GzEncoder;
use replay_engine::{decoded_instructions, replay_engine::ReplayEngine};
use std::{collections::HashMap, fs::File, io::LineWriter, io::Write};
use whirlpool_replayer::{schema::WhirlpoolTransaction, serde::AccountDataStoreConfig, Slot};

pub async fn process(
    whirlpool_state_file_path: String,
    whirlpool_token_file_path: String,
    whirlpool_transaction_file_path: String,
    whirlpool_event_file_path: String,
) -> Result<()> {
    let f = File::create(whirlpool_event_file_path).unwrap();
    let encoder = GzEncoder::new(f, flate2::Compression::default());
    let mut writer = LineWriter::new(encoder);

    // build replayer
    let (mut replay_engine, mut transaction_iter, decimals) = build_with_local_file_storage(
        whirlpool_state_file_path,
        whirlpool_token_file_path,
        whirlpool_transaction_file_path,
        &AccountDataStoreConfig::OnDisk(None),
    );

    let mut next_whirlpool_transaction = transaction_iter.next();
    while next_whirlpool_transaction.is_some() {
        let whirlpool_transaction = next_whirlpool_transaction.unwrap();

        let slot = Slot {
            slot: whirlpool_transaction.slot,
            block_height: whirlpool_transaction.block_height,
            block_time: whirlpool_transaction.block_time,
        };

        replay_engine.update_slot(slot.slot, slot.block_height, slot.block_time);

        let mut event_block_transactions: Vec<WhirlpoolEventTransaction> = Vec::new();

        for transaction in whirlpool_transaction.transactions {
            let mut events: Vec<WhirlpoolEvent> = vec![];

            for instruction in transaction.clone().instructions {
                let name = instruction.name;
                let payload = instruction.payload.to_string();
                let decoded = decoded_instructions::from_json(&name, &payload).unwrap();

                match decoded {
                    decoded_instructions::DecodedInstruction::ProgramDeployInstruction(
                        deploy_instruction,
                    ) => {
                        replay_engine.update_program_data(deploy_instruction.program_data);

                        events.push(WhirlpoolEvent::ProgramDeployed(
                            ProgramDeployedEventPayload {},
                        ));
                    }
                    decoded_instructions::DecodedInstruction::WhirlpoolInstruction(
                        whirlpool_instruction,
                    ) => {
                        let result = replay_engine
                            .replay_instruction(&whirlpool_instruction)
                            .unwrap();

                        events.extend(build_whirlpool_events(
                            &whirlpool_instruction,
                            &decimals,
                            replay_engine.get_accounts(),
                            &result.snapshot,
                        ));
                    }
                }
            }

            event_block_transactions.push(WhirlpoolEventTransaction {
                signature: transaction.signature,
                payer: transaction.payer,
                events,
            });
        }

        let event_block = WhirlpoolEventBlock {
            slot: whirlpool_transaction.slot,
            block_height: whirlpool_transaction.block_height,
            block_time: whirlpool_transaction.block_time,
            transactions: event_block_transactions,
        };

        let jsonl = serde_json::to_string(&event_block).unwrap();
        writer.write_all(jsonl.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();

        next_whirlpool_transaction = transaction_iter.next();
    }

    writer.flush().unwrap();

    Ok(())
}

pub fn build_with_local_file_storage(
    whirlpool_state_file_path: String,
    whirlpool_token_file_path: String,
    whirlpool_transaction_file_path: String,
    account_data_store_config: &AccountDataStoreConfig,
) -> (
    ReplayEngine,
    Box<dyn Iterator<Item = WhirlpoolTransaction> + Send>,
    HashMap<String, u8>,
) {
    let state = whirlpool_replayer::io::load_from_local_whirlpool_state_file(
        &whirlpool_state_file_path,
        account_data_store_config,
    );
    let token =
        whirlpool_replayer::io::load_from_local_whirlpool_token_file(&whirlpool_token_file_path);
    let transaction_iter = whirlpool_replayer::io::load_from_local_whirlpool_transaction_file(
        &whirlpool_transaction_file_path,
    );

    let replay_engine = ReplayEngine::new(
        Slot::new(state.slot, state.block_height, state.block_time),
        state.program_data,
        state.accounts,
    );

    let decimals = token
        .tokens
        .iter()
        .map(|t| (t.mint.clone(), t.decimals))
        .collect();

    (replay_engine, Box::new(transaction_iter), decimals)
}
