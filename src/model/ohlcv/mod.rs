use serde_derive::Serialize;
use super::serde::{string_decimal_price, string_u128, string_u64};

/*

Whirlpool OHLCV Daily JSON Lines Format

To reduce data size, we use short field names.

Each line is a JSON object with the following schema:

{
  whirlpool(w): String(base58 encoding),
  whirlpoolsConfig(wc): String(base58 encoding),
  tokenA(ta): { mint(m): String(base58 encoding), decimals(d): u8 },
  tokenB(tb): { mint(m): String(base58 encoding), decimals(d): u8 },
  tickSpacing(ts): u16,
  initialState(is):
    { t: "existing(e)", p: { previousCloseSqrtPrice(pcsp): String, previousCloseDecimalPrice(pcdp): String } } |
    { t: "new(n)", p: { initialSqrtPrice(isp): String, initialDecimalPrice(idp): String, initializedSlot: u64, initializedBlockTime(ibt): i64 } },
  estimatedFees(ef): {
    liquidityProviderFeeA(lpfa): u64,
    liquidityProviderFeeB(lpfb): u64,
    protocolFeeA(pfa): u64,
    protocolFeeB(pfb): u64,
  },
  daily(d): {
    timestamp(t): i64(UTC, UNIX timestamp in seconds, first second of the day),
    ohlc(ohlc): { sqrtPrice(sp): { open(o): String, high(h): String, low(l): String, close(c): String }, decimalPrice(dp): { open(o): String, high(h): String, low(l): String, close(c): String } },
    volume(v): {
      ab: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
      ba: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
    },
  },
}

Whirlpool OHLCV Minutely JSON Lines Format

To reduce data size, we use short field names.
Also, data for minutes with no trades at all will be omitted.

Each line is a JSON object with the following schema:

{
  whirlpool(w): String(base58 encoding),
  whirlpoolsConfig(wc): String(base58 encoding),
  tokenA(ta): { mint(m): String(base58 encoding), decimals(d): u8 },
  tokenB(tb): { mint(m): String(base58 encoding), decimals(d): u8 },
  tickSpacing(ts): u16,
  initialState(is):
    { t: "existing(e)", p: { previousCloseSqrtPrice(pcsp): String, previousCloseDecimalPrice(pcdp): String } } |
    { t: "new(n)", p: { initialSqrtPrice(isp): String, initialDecimalPrice(idp): String, initializedSlot: u64, initializedBlockTime(ibt): i64 } },
  estimatedFees(ef): {
    liquidityProviderFeeA(lpfa): u64,
    liquidityProviderFeeB(lpfb): u64,
    protocolFeeA(pfa): u64,
    protocolFeeB(pfb): u64,
  },
  daily(d): {
    timestamp(t): i64(UTC, UNIX timestamp in seconds, first second of the day),
    ohlc(ohlc): { sqrtPrice(sp): { open(o): String, high(h): String, low(l): String, close(c): String }, decimalPrice(dp): { open(o): String, high(h): String, low(l): String, close(c): String } },
    volume(v): {
      ab: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
      ba: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
    },
  },
  minutely(m): [
    {
      timestamp(t): i64(UTC, UNIX timestamp in seconds, first second of the minute),
      ohlc(ohlc): { sqrtPrice(sp): { open(o): String, high(h): String, low(l): String, close(c): String }, decimalPrice(dp): { open(o): String, high(h): String, low(l): String, close(c): String } },
      volume(v): {
        ab: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
        ba: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
      },
    },
    ...
  ],
}

*/

pub type PubkeyString = String;
pub type DecimalPrice = bigdecimal::BigDecimal;
pub type Decimals = u8;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct WhirlpoolOhlcvDailyData {
  #[serde(flatten)]
  metadata: WhirlpoolOhlcvMetadata,
  #[serde(rename = "is")]
  initial_state: InitialState,
  #[serde(rename = "ef")]
  estimated_fees: EstimatedFees,
  #[serde(rename = "d")]
  daily: WhirlpoolOhlcvDataUnit,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct WhirlpoolOhlcvMinutelyData {
  #[serde(flatten)]
  metadata: WhirlpoolOhlcvMetadata,
  #[serde(rename = "is")]
  initial_state: InitialState,
  #[serde(rename = "ef")]
  estimated_fees: EstimatedFees,
  #[serde(rename = "d")]
  daily: WhirlpoolOhlcvDataUnit,
  #[serde(rename = "m")]
  minutely: Vec<WhirlpoolOhlcvDataUnit>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct WhirlpoolOhlcvMetadata {
  #[serde(rename = "w")]
  whirlpool: PubkeyString,
  #[serde(rename = "wc")]
  whirlpools_config: PubkeyString,
  #[serde(rename = "ta")]
  token_a: TokenData,
  #[serde(rename = "tb")]
  token_b: TokenData,
  #[serde(rename = "ts")]
  tick_spacing: u16,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct TokenData {
  #[serde(rename = "m")]
  mint: PubkeyString,
  #[serde(rename = "d")]
  decimals: Decimals,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum InitialState {
  Existing {
    #[serde(rename = "pcsp", with = "string_u128")]
    previous_close_sqrt_price: u128,
    #[serde(rename = "pcdp", with = "string_decimal_price")]
    previous_close_decimal_price: DecimalPrice,
  },
  New {
    #[serde(rename = "isp", with = "string_u128")]
    initial_sqrt_price: u128,
    #[serde(rename = "idp", with = "string_decimal_price")]
    initial_decimal_price: DecimalPrice,
    #[serde(rename = "is")]
    initialized_slot: u64,
    #[serde(rename = "ibt")]
    initialized_block_time: i64,
  },
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct EstimatedFees {
  #[serde(rename = "lpfa", with = "string_u64")]
  liquidity_provider_fee_a: u64,
  #[serde(rename = "lpfb", with = "string_u64")]
  liquidity_provider_fee_b: u64,
  #[serde(rename = "pfa", with = "string_u64")]
  protocol_fee_a: u64,
  #[serde(rename = "pfb", with = "string_u64")]
  protocol_fee_b: u64,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct WhirlpoolOhlcvDataUnit {
  #[serde(rename = "t")]
  timestamp: i64,
  ohlcv: WhirlpoolOhlcvData,
  #[serde(rename = "v")]
  volume: VolumeData,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct WhirlpoolOhlcvData {
  #[serde(rename = "sp")]
  sqrt_price: SqrtPriceData,
  #[serde(rename = "dp")]
  decimal_price: DecimalPriceData,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct SqrtPriceData {
  #[serde(rename = "o", with = "string_u128")]
  open: u128,
  #[serde(rename = "h", with = "string_u128")]
  high: u128,
  #[serde(rename = "l", with = "string_u128")]
  low: u128,
  #[serde(rename = "c", with = "string_u128")]
  close: u128,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct DecimalPriceData {
  #[serde(rename = "o", with = "string_decimal_price")]
  open: DecimalPrice,
  #[serde(rename = "h", with = "string_decimal_price")]
  high: DecimalPrice,
  #[serde(rename = "l", with = "string_decimal_price")]
  low: DecimalPrice,
  #[serde(rename = "c", with = "string_decimal_price")]
  close: DecimalPrice,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct VolumeData {
  ab: VolumeDirectionData,
  ba: VolumeDirectionData,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct VolumeDirectionData {
  #[serde(rename = "ti", with = "string_u128")]
  total_in: u128,
  #[serde(rename = "to", with = "string_u128")]
  total_out: u128,
  #[serde(rename = "c")]
  count: u64,
}
