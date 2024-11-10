use super::{
    super::serde::{serialize_decimal_price, serialize_u128},
    DecimalPrice, Decimals, PubkeyString, TokenProgram,
};
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct PoolInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PoolInitializedEventOrigin,

    #[serde(rename = "ts")]
    pub tick_spacing: u16,
    #[serde(rename = "sp", serialize_with = "serialize_u128")]
    pub sqrt_price: u128,
    #[serde(rename = "dp", serialize_with = "serialize_decimal_price")]
    pub decimal_price: DecimalPrice,

    #[serde(rename = "c")]
    pub config: PubkeyString,
    #[serde(rename = "tma")]
    pub token_mint_a: PubkeyString,
    #[serde(rename = "tmb")]
    pub token_mint_b: PubkeyString,
    #[serde(rename = "f")]
    pub funder: PubkeyString,
    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "ft")]
    pub fee_tier: PubkeyString,

    #[serde(rename = "tpa")]
    pub token_program_a: TokenProgram,
    #[serde(rename = "tpb")]
    pub token_program_b: TokenProgram,

    // decimals
    #[serde(rename = "tda")]
    pub token_decimals_a: Decimals,
    #[serde(rename = "tdb")]
    pub token_decimals_b: Decimals,

    // pool state
    #[serde(rename = "cti")]
    pub current_tick_index: i32,
    #[serde(rename = "fr")]
    pub fee_rate: u16,
    #[serde(rename = "pfr")]
    pub protocol_fee_rate: u16,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum PoolInitializedEventOrigin {
    #[serde(rename = "ip")]
    InitializePool,
    #[serde(rename = "ipv2")]
    InitializePoolV2,
}
