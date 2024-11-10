use super::super::serde::serialize_u128;
use super::{Decimals, PubkeyString};
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RewardEmissionsUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: RewardEmissionsUpdatedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,

    #[serde(rename = "ri")]
    pub reward_index: u8,

    #[serde(rename = "rm")]
    pub reward_mint: PubkeyString,

    #[serde(rename = "rd")]
    pub reward_decimals: Decimals,

    #[serde(rename = "oepsx64", serialize_with = "serialize_u128")]
    pub old_emissions_per_second_x64: u128,

    #[serde(rename = "nepsx64", serialize_with = "serialize_u128")]
    pub new_emissions_per_second_x64: u128,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum RewardEmissionsUpdatedEventOrigin {
    #[serde(rename = "sre")]
    SetRewardEmissions,
    #[serde(rename = "srev2")]
    SetRewardEmissionsV2,
}