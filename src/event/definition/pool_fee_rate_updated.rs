use super::PubkeyString;
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct PoolFeeRateUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PoolFeeRateUpdatedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyString,
    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,

    #[serde(rename = "ofr")]
    pub old_fee_rate: u16,
    #[serde(rename = "nfr")]
    pub new_fee_rate: u16,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum PoolFeeRateUpdatedEventOrigin {
    #[serde(rename = "sfr")]
    SetFeeRate,
}
