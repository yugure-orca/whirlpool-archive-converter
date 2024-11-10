use super::PubkeyString;
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RewardAuthorityUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: RewardAuthorityUpdatedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,

    #[serde(rename = "ri")]
    pub reward_index: u8,

    #[serde(rename = "ora")]
    pub old_reward_authority: PubkeyString,
    #[serde(rename = "nra")]
    pub new_reward_authority: PubkeyString,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum RewardAuthorityUpdatedEventOrigin {
    #[serde(rename = "sra")]
    SetRewardAuthority,
    #[serde(rename = "srabsa")]
    SetRewardAuthorityBySuperAuthority,
}