use super::PubkeyString;
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct FeeTierInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: FeeTierInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyString,

    #[serde(rename = "ft")]
    pub fee_tier: PubkeyString,

    #[serde(rename = "ts")]
    pub tick_spacing: u16,

    #[serde(rename = "dfr")]
    pub default_fee_rate: u16,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum FeeTierInitializedEventOrigin {
    #[serde(rename = "ift")]
    InitializeFeeTier,
}
