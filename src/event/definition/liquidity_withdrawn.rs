use super::super::serde::{serialize_decimal_price, serialize_u128};
use super::{DecimalPrice, PubkeyString, TransferInfo};
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct LiquidityWithdrawnEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: LiquidityWithdrawnEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "pa")]
    pub position_authority: PubkeyString,
    #[serde(rename = "p")]
    pub position: PubkeyString,
    #[serde(rename = "lta")]
    pub lower_tick_array: PubkeyString,
    #[serde(rename = "uta")]
    pub upper_tick_array: PubkeyString,

    #[serde(rename = "ld", serialize_with = "serialize_u128")]
    pub liquidity_delta: u128,

    // transfer info
    #[serde(rename = "ta")]
    pub transfer_a: TransferInfo,
    #[serde(rename = "tb")]
    pub transfer_b: TransferInfo,

    // position state
    #[serde(rename = "lti")]
    pub lower_tick_index: i32,
    #[serde(rename = "uti")]
    pub upper_tick_index: i32,
    #[serde(rename = "ldp", serialize_with = "serialize_decimal_price")]
    pub lower_decimal_price: DecimalPrice,
    #[serde(rename = "udp", serialize_with = "serialize_decimal_price")]
    pub upper_decimal_price: DecimalPrice,
    #[serde(rename = "opl", serialize_with = "serialize_u128")]
    pub old_position_liquidity: u128,
    #[serde(rename = "npl", serialize_with = "serialize_u128")]
    pub new_position_liquidity: u128,

    // pool state
    #[serde(rename = "owl", serialize_with = "serialize_u128")]
    pub old_whirlpool_liquidity: u128,
    #[serde(rename = "nwl", serialize_with = "serialize_u128")]
    pub new_whirlpool_liquidity: u128,
    #[serde(rename = "wsp", serialize_with = "serialize_u128")]
    pub whirlpool_sqrt_price: u128,
    #[serde(rename = "wcti")]
    pub whirlpool_current_tick_index: i32,
    #[serde(rename = "wdp", serialize_with = "serialize_decimal_price")]
    pub whirlpool_decimal_price: DecimalPrice,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum LiquidityWithdrawnEventOrigin {
    #[serde(rename = "dl")]
    DecreaseLiquidity,
    #[serde(rename = "dlv2")]
    DecreaseLiquidityV2,
}
