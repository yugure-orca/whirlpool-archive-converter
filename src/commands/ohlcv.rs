use crate::model::{event::{
  convert::build_whirlpool_events, definition::{PoolInitializedEventPayload, ProgramDeployedEventPayload, TradedEventPayload}, WhirlpoolEvent,
  WhirlpoolEventBlock, WhirlpoolEventTransaction, definition::TradeDirection,
}};
use anchor_lang::Discriminator;
use anyhow::Result;
use flate2::{read::GzDecoder, write::GzEncoder};
use replay_engine::{decoded_instructions, replay_engine::ReplayEngine};
use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader, BufWriter},
};
use whirlpool_replayer::{schema::{WhirlpoolState, WhirlpoolTransaction}, serde::AccountDataStoreConfig, Slot};
use anchor_lang::AccountDeserialize;

pub async fn process(
  whirlpool_state_file_path: String,
  whirlpool_token_file_path: String,
  whirlpool_event_file_path: String,
  whirlpool_ohlcv_daily_file_path: String,
  whirlpool_ohlcv_minutely_file_path: String,
) -> Result<()> {
  let (state, event_block_iter, decimals) = build_with_local_file_storage(
    whirlpool_state_file_path,
    whirlpool_token_file_path,
    whirlpool_event_file_path,
    &AccountDataStoreConfig::OnDisk(None),
  );

  let daily_timestamp = 0i64;

  let mut ohlcv_data_manager = OhlcvDataManager::new(daily_timestamp);

  state.accounts.traverse(|pubkey, data| {
    if data.starts_with(&whirlpool_base::state::Whirlpool::DISCRIMINATOR) {
      let whirlpool = whirlpool_base::state::Whirlpool::try_deserialize(&mut data.as_slice()).unwrap();
      let mint_a = whirlpool.token_mint_a.to_string();
      let mint_b = whirlpool.token_mint_b.to_string();
      let decimals_a = *decimals.get(&mint_a).unwrap();
      let decimals_b = *decimals.get(&mint_b).unwrap();
      ohlcv_data_manager.initialize_with_previous_close(Metadata {
        whirlpool: pubkey.to_string(),
        whirlpools_config: whirlpool.whirlpools_config.to_string(),
        mint_a,
        mint_b,
        tick_spacing: whirlpool.tick_spacing,
        decimals_a,
        decimals_b,
      }, whirlpool.sqrt_price);
    }
    Ok(())
  })?;

  for event_block in event_block_iter {
    event_block.transactions.iter().for_each(|transaction| {
      transaction.events.iter().for_each(|event| {
        match event {
          WhirlpoolEvent::Traded(traded) => {
            ohlcv_data_manager.process_traded_event(event_block.block_time, traded);
          }
          WhirlpoolEvent::PoolInitialized(pool_initialized) => {
            ohlcv_data_manager.process_pool_initialized_event(event_block.slot, event_block.block_time, pool_initialized);
          }
          _ => { /* ignore */ }
        }
      });
    });
  }

  println!("{:?}", ohlcv_data_manager.data.get("21gTfxAnhUDjJGZJDkTXctGFKT8TeiXx6pN1CEg9K1uW"));

  Ok(())
}

pub fn build_with_local_file_storage(
  whirlpool_state_file_path: String,
  whirlpool_token_file_path: String,
  whirlpool_event_file_path: String,
  account_data_store_config: &AccountDataStoreConfig,
) -> (
  WhirlpoolState,
  Box<dyn Iterator<Item = WhirlpoolEventBlock> + Send>,
  HashMap<String, u8>,
) {
  let state = whirlpool_replayer::io::load_from_local_whirlpool_state_file(
      &whirlpool_state_file_path,
      account_data_store_config,
  );
  let token =
      whirlpool_replayer::io::load_from_local_whirlpool_token_file(&whirlpool_token_file_path);
  let event_iter = load_from_local_whirlpool_event_file(
      &whirlpool_event_file_path,
  );

  let decimals = token
      .tokens
      .iter()
      .map(|t| (t.mint.clone(), t.decimals))
      .collect();

  (state, Box::new(event_iter), decimals)
}

fn load_from_local_whirlpool_event_file(
  whirlpool_event_file_path: &str,
) -> impl Iterator<Item = WhirlpoolEventBlock> {
  let file = File::open(whirlpool_event_file_path).unwrap();

  let decoder = GzDecoder::new(file);
  let buf = BufReader::new(decoder);

  let iter = buf.lines().map(|jsonl| jsonl.unwrap()).map(|jsonl| {
      let t: Result<WhirlpoolEventBlock, serde_json::Error> =
          serde_json::from_str(jsonl.as_str());
      t.unwrap()
  });

  iter
}

#[derive(Debug)]
struct OhlcvDataManager {
  timestamp: i64,
  data: HashMap<String, OhlcvData>,
}

impl OhlcvDataManager {
  pub fn new(timestamp: i64) -> Self {
    Self { timestamp, data: HashMap::new() }
  }

  pub fn initialize_with_previous_close(&mut self, metadata: Metadata, previous_close_sqrt_price: u128) {
    self.data.insert(metadata.whirlpool.clone(), OhlcvData {
      metadata,
      initial_state: InitialState::Existing(previous_close_sqrt_price),
      estimated_fees: EstimatedFees::default(),
      daily: SqrtPriceOhlcvDataUnit {
        timestamp: self.timestamp,
        open: previous_close_sqrt_price,
        high: previous_close_sqrt_price,
        low: previous_close_sqrt_price,
        close: previous_close_sqrt_price,
        volume_a_to_b: VolumeData::default(),
        volume_b_to_a: VolumeData::default(),
      },
      minutely: HashMap::new(),
    });
  }

  pub fn process_pool_initialized_event(&mut self, slot: u64, block_time: i64, pool_initialized: &PoolInitializedEventPayload) {
    let metadata = Metadata {
      whirlpool: pool_initialized.whirlpool.clone(),
      whirlpools_config: pool_initialized.config.clone(),
      mint_a: pool_initialized.token_mint_a.clone(),
      mint_b: pool_initialized.token_mint_b.clone(),
      tick_spacing: pool_initialized.tick_spacing,
      decimals_a: pool_initialized.token_decimals_a,
      decimals_b: pool_initialized.token_decimals_b,
    };
    let initial_sqrt_price = pool_initialized.sqrt_price;

    self.data.insert(metadata.whirlpool.clone(), OhlcvData {
      metadata,
      initial_state: InitialState::New(initial_sqrt_price, slot, block_time),
      estimated_fees: EstimatedFees::default(),
      daily: SqrtPriceOhlcvDataUnit {
        timestamp: self.timestamp,
        open: initial_sqrt_price,
        high: initial_sqrt_price,
        low: initial_sqrt_price,
        close: initial_sqrt_price,
        volume_a_to_b: VolumeData::default(),
        volume_b_to_a: VolumeData::default(),
      },
      minutely: HashMap::new(),
    });
  }

  pub fn process_traded_event(&mut self, block_time: i64, traded: &TradedEventPayload) {
    let whirlpool = self.data.get_mut(&traded.whirlpool).unwrap();

    // updating estimated_fees
    let post_transfer_fee = calc_post_transfer_fee(traded.transfer_in.amount, traded.transfer_in.transfer_fee_bps, traded.transfer_in.transfer_fee_max);
    let trade_fee = calc_trade_fee(post_transfer_fee, traded.fee_rate);
    let (liquidity_provider_fee, protocol_fee) = split_fee(trade_fee, traded.protocol_fee_rate);
    match traded.trade_direction {
      TradeDirection::AtoB => {
        whirlpool.estimated_fees.liquidity_provider_fee_a += liquidity_provider_fee;
        whirlpool.estimated_fees.protocol_fee_a += protocol_fee;
      }
      TradeDirection::BtoA => {
        whirlpool.estimated_fees.liquidity_provider_fee_b += liquidity_provider_fee;
        whirlpool.estimated_fees.protocol_fee_b += protocol_fee;
      }
    }

    // updating daily
    whirlpool.daily.high = whirlpool.daily.high.max(traded.new_sqrt_price);
    whirlpool.daily.low = whirlpool.daily.low.min(traded.new_sqrt_price);
    whirlpool.daily.close = traded.new_sqrt_price;
    match traded.trade_direction {
      TradeDirection::AtoB => {
        whirlpool.daily.volume_a_to_b.total_in += traded.transfer_in.amount as u128;
        whirlpool.daily.volume_a_to_b.total_out += traded.transfer_out.amount as u128;
        whirlpool.daily.volume_a_to_b.count += 1;
      }
      TradeDirection::BtoA => {
        whirlpool.daily.volume_b_to_a.total_in += traded.transfer_in.amount as u128;
        whirlpool.daily.volume_b_to_a.total_out += traded.transfer_out.amount as u128;
        whirlpool.daily.volume_b_to_a.count += 1;
      }
    }

    // updating minutely
    let minutely_timestamp = block_time / 60 * 60;
    let minutely_data = whirlpool.minutely.entry(minutely_timestamp).or_insert(SqrtPriceOhlcvDataUnit {
      timestamp: minutely_timestamp,
      open: traded.old_sqrt_price,
      high: traded.old_sqrt_price,
      low: traded.old_sqrt_price,
      close: traded.old_sqrt_price,
      volume_a_to_b: VolumeData::default(),
      volume_b_to_a: VolumeData::default(),
    });

    minutely_data.high = minutely_data.high.max(traded.new_sqrt_price);
    minutely_data.low = minutely_data.low.min(traded.new_sqrt_price);
    minutely_data.close = traded.new_sqrt_price;
    match traded.trade_direction {
      TradeDirection::AtoB => {
        minutely_data.volume_a_to_b.total_in += traded.transfer_in.amount as u128;
        minutely_data.volume_a_to_b.total_out += traded.transfer_out.amount as u128;
        minutely_data.volume_a_to_b.count += 1;
      }
      TradeDirection::BtoA => {
        minutely_data.volume_b_to_a.total_in += traded.transfer_in.amount as u128;
        minutely_data.volume_b_to_a.total_out += traded.transfer_out.amount as u128;
        minutely_data.volume_b_to_a.count += 1;
      }
    }
  }
}

#[derive(Debug)]
struct OhlcvData {
  metadata: Metadata,
  initial_state: InitialState,
  estimated_fees: EstimatedFees,
  daily: SqrtPriceOhlcvDataUnit,
  minutely: HashMap<i64, SqrtPriceOhlcvDataUnit>,
}

#[derive(Debug)]
struct Metadata {
  whirlpool: String,
  whirlpools_config: String,
  mint_a: String,
  mint_b: String,
  tick_spacing: u16,
  decimals_a: u8,
  decimals_b: u8,
}

#[derive(Debug)]
enum InitialState {
  Existing(u128), // previous close sqrt price
  New(u128, u64, i64), // initial sqrt price, slot, block time
}

#[derive(Default, Debug)]
struct EstimatedFees {
  liquidity_provider_fee_a: u64,
  liquidity_provider_fee_b: u64,
  protocol_fee_a: u64,
  protocol_fee_b: u64,
}

#[derive(Debug)]
struct SqrtPriceOhlcvDataUnit {
  timestamp: i64,
  open: u128,
  high: u128,
  low: u128,
  close: u128,
  volume_a_to_b: VolumeData,
  volume_b_to_a: VolumeData,
}

#[derive(Default, Debug)]
struct VolumeData {
  total_in: u128,
  total_out: u128,
  count: u64,
}

fn calc_post_transfer_fee(amount: u64, fee_bps: Option<u16>, fee_max: Option<u64>) -> u64 {
  // TODO: implement
  amount
}

fn calc_trade_fee(amount: u64, fee_rate: u16) -> u64 {
  (amount as u128 * fee_rate as u128 / 1_000_000 as u128).try_into().unwrap()
}

fn split_fee(amount: u64, protocol_fee_rate: u16) -> (u64, u64) {
  let protocol_fee = (amount as u128 * protocol_fee_rate as u128 / 10_000 as u128).try_into().unwrap();
  let liquidity_provider_fee = amount - protocol_fee;
  (liquidity_provider_fee, protocol_fee)
}
